#!/usr/bin/env python3
"""Generate the /on-ai Transformer explorer data from real GPT-2 (small).

Runs openai-community/gpt2 locally and records, for a small bank of
short original sentences, the model's actual internals:

  * token strings + ids
  * token-embedding PCA 2-D projection (computed here, labeled as such)
    plus a 48-dimension slice of the raw embedding vectors
  * attention weights for all 12 layers x 12 heads
  * residual-stream L2 norm per token per layer (layer 0 = embedding)
  * logit lens: per layer, the top-3 next-token guesses for the final
    position (final layer norm + unembedding applied at every layer)
  * the final position's top-50 logits plus a tail logit (logsumexp of
    every remaining logit) so a client can rebuild an honest softmax

Deterministic by construction: CPU only, eval mode (no dropout),
fixed seed, floats rounded to 3 decimals, attention rows renormalized
after rounding so every row still sums to 1. Re-running this script
must reproduce the committed JSON byte-for-byte.

Usage:
  python3 generate-transformer-explorer.py --out public/data/transformer-explorer.json
  python3 generate-transformer-explorer.py --check public/data/transformer-explorer.json
"""

import argparse
import json
import math
import sys

SENTENCES = [
    {
        "id": "pronoun",
        "label": "A pronoun finds its noun",
        "text": "The nurse told the patient that she would return soon",
        "note": "Watch who 'she' listens to. In the middle layers, heads "
                "link the pronoun back to 'nurse' — the model resolves "
                "reference with attention weights, not grammar rules.",
    },
    {
        "id": "repetition",
        "label": "A phrase repeats",
        "text": "the clock ticked, the clock ticked, the clock",
        "note": "When the phrase starts over, induction-style heads in "
                "the middle layers point the last 'clock' back at the "
                "first one — and at what followed it.",
    },
    {
        "id": "entities",
        "label": "Two people, one boat",
        "text": "Maria sold her old boat to Diego last spring",
        "note": "Names, a possession, and a transfer. Attention has to "
                "keep Maria, Diego, and the boat in separate lanes while "
                "the sentence moves them around.",
    },
    {
        "id": "factual",
        "label": "A fact it knows",
        "text": "The first man to walk on the moon was Neil",
        "note": "Ten tokens in, the model puts almost all of its "
                "probability on one surname. The logit lens below "
                "shows that answer assembling layers before the "
                "output — and competing names dying off as it does.",
    },
    {
        "id": "agreement",
        "label": "Agreement across a gap",
        "text": "The keys to the cabinet are on the table",
        "note": "'Keys' is plural, 'cabinet' is singular and much "
                "closer to the verb. The model still lands on 'are' — "
                "number has to survive the distance.",
    },
    {
        "id": "playful",
        "label": "Offshore, briefly",
        "text": "The robot checked the compass and sailed on",
        "note": "A lighter one. Even a toy sentence gets the full "
                "treatment: twelve layers of second-guessing before "
                "the next word.",
    },
    {
        "id": "coreference",
        "label": "They, and it",
        "text": "When the engineers finished the bridge, they opened it to traffic",
        "note": "Two pronouns, two different anchors: 'they' reaches "
                "back to the engineers, 'it' to the bridge. Same "
                "mechanism, different targets.",
    },
]

MODEL_ID = "openai-community/gpt2"
MAX_TOKENS = 14
SLICE_DIMS = 48
TOP_K = 50
ROUND = 3


def r3(x):
    return round(float(x), ROUND)


def renormalize_rows(mat):
    """Round a T x T attention matrix to 3 decimals, then repair each
    row's rounding drift on its largest entry so rows sum to 1.000."""
    out = []
    for row in mat:
        rounded = [r3(v) for v in row]
        drift = round(1.0 - sum(rounded), ROUND)
        if drift != 0.0:
            i = max(range(len(rounded)), key=lambda j: rounded[j])
            rounded[i] = r3(rounded[i] + drift)
        out.append(rounded)
    return out


def pca_2d(vectors):
    """Project an T x D matrix to 2-D with PCA via a full SVD (LAPACK,
    deterministic). Sign of each component is pinned (largest-|loading|
    entry made positive) so the projection is stable across runs."""
    import torch

    x = vectors - vectors.mean(dim=0, keepdim=True)
    _, _, vh = torch.linalg.svd(x, full_matrices=False)
    v = vh[:2].T.clone()
    for k in range(v.shape[1]):
        col = v[:, k]
        pivot = col[torch.argmax(torch.abs(col))]
        if pivot < 0:
            v[:, k] = -col
    proj = x @ v
    return [[r3(a), r3(b)] for a, b in proj.tolist()]


def top_entries(logits, k):
    import torch

    vals, idx = torch.topk(logits, k)
    return vals.tolist(), idx.tolist()


def load_model():
    """Load GPT-2 small from its safetensors checkpoint by hand.

    transformers 5.0's `from_pretrained` segfaults on primo (verified
    2026-10-13: crash inside the loading path, model construction and
    the tokenizer are fine), so the checkpoint is mapped onto the
    model directly: the raw GPT-2 key layout gains the `transformer.`
    prefix, the non-persistent causal-mask buffers (`*.attn.bias`) are
    dropped, and `lm_head.weight` is tied to the token embedding as in
    the original model. Verified against reference behaviour after
    loading: perplexity 27.2 on plain prose, 'Barack Obama was the
    44th President of the United' -> ' States' at 0.999.
    """
    import json
    import os

    import torch
    from huggingface_hub import hf_hub_download
    from safetensors.torch import load_file
    from transformers import GPT2Config, GPT2LMHeadModel

    weights_path = hf_hub_download(MODEL_ID, "model.safetensors")
    config_path = hf_hub_download(MODEL_ID, "config.json")
    revision = os.path.basename(os.path.dirname(weights_path))

    with open(config_path, encoding="utf-8") as f:
        config = GPT2Config(**json.load(f))
    model = GPT2LMHeadModel(config)
    state = {}
    for key, value in load_file(weights_path).items():
        if key == "lm_head.weight" or key.endswith(".attn.bias"):
            continue
        state["transformer." + key] = value
    missing, unexpected = model.load_state_dict(state, strict=False)
    assert unexpected == [], f"unexpected checkpoint keys: {unexpected}"
    assert missing == ["lm_head.weight"], f"missing keys: {missing}"
    model.tie_weights()
    model.eval()
    # Attention weights are only materialized by the eager
    # implementation; under SDPA `output_attentions` yields Nones.
    model.set_attn_implementation("eager")
    return model, revision


def generate(out_path):
    import torch
    from transformers import GPT2TokenizerFast

    torch.manual_seed(0)
    torch.set_grad_enabled(False)

    tokenizer = GPT2TokenizerFast.from_pretrained(MODEL_ID)
    model, revision = load_model()

    ln_f = model.transformer.ln_f
    lm_head = model.lm_head

    sentences = []
    for spec in SENTENCES:
        ids = tokenizer.encode(spec["text"])
        if len(ids) > MAX_TOKENS:
            raise SystemExit(
                f"sentence {spec['id']!r} tokenizes to {len(ids)} tokens "
                f"(> {MAX_TOKENS}): {spec['text']!r}"
            )
        input_ids = torch.tensor([ids], dtype=torch.long)
        out = model(
            input_ids, output_attentions=True, output_hidden_states=True
        )

        tokens = []
        for tid in ids:
            raw = tokenizer.convert_ids_to_tokens([tid])[0]
            tokens.append(
                {"t": raw.replace("\u0120", " "), "id": int(tid)}
            )

        hidden = out.hidden_states  # 13 x [1, T, 768]
        t_count = len(ids)

        # Embeddings: layer-0 hidden state (token + position, post-dropout
        # which is identity in eval mode).
        emb = hidden[0][0]
        emb_pca = pca_2d(emb)
        emb_slice = [
            [r3(v) for v in emb[t, :SLICE_DIMS].tolist()]
            for t in range(t_count)
        ]

        # Attention: 12 layers x 12 heads x T x T.
        attn = []
        for layer in range(12):
            heads = []
            for head in range(12):
                mat = out.attentions[layer][0, head].tolist()
                heads.append(renormalize_rows(mat))
            attn.append(heads)

        # Residual-stream norms per layer per token.
        res_norms = [
            [r3(v) for v in torch.linalg.vector_norm(h[0], dim=-1).tolist()]
            for h in hidden
        ]

        # Logit lens: final position, every layer, top-3. In
        # transformers v5 the base model returns its last hidden
        # state already passed through the final layer norm, so the
        # norm is applied here only to layers 0..11; layer 12 goes
        # straight to the unembedding (verified: reproduces the
        # model's own logits to 8e-05).
        lens = []
        for layer_idx, h in enumerate(hidden):
            vec = h[0, -1] if layer_idx == 12 else ln_f(h[0, -1])
            logits = lm_head(vec)
            vals, idxs = top_entries(logits, 3)
            lens.append(
                [
                    {
                        "t": tokenizer.convert_ids_to_tokens([i])[0].replace(
                            "\u0120", " "
                        ),
                        "id": int(i),
                        "logit": r3(v),
                    }
                    for v, i in zip(vals, idxs)
                ]
            )

        # Final distribution: top-50 logits + tail logsumexp.
        final_logits = out.logits[0, -1]
        vals, idxs = top_entries(final_logits, TOP_K)
        mask = torch.ones_like(final_logits, dtype=torch.bool)
        mask[idxs] = False
        tail = torch.logsumexp(final_logits[mask], dim=0)
        top50 = [
            {
                "t": tokenizer.convert_ids_to_tokens([i])[0].replace(
                    "\u0120", " "
                ),
                "id": int(i),
                "logit": r3(v),
            }
            for v, i in zip(vals, idxs)
        ]

        sentences.append(
            {
                "id": spec["id"],
                "label": spec["label"],
                "note": spec["note"],
                "text": spec["text"],
                "tokens": tokens,
                "emb_pca": emb_pca,
                "emb_slice": emb_slice,
                "attn": attn,
                "res_norms": res_norms,
                "logit_lens": lens,
                "top50": top50,
                "tail_logit": r3(tail),
            }
        )
        print(
            f"{spec['id']}: {t_count} tokens, greedy next = "
            f"{top50[0]['t']!r}",
            file=sys.stderr,
        )

    data = {
        "meta": {
            "model": MODEL_ID,
            "revision": revision,
            "vocab_size": 50257,
            "layers": 12,
            "heads": 12,
            "d_model": 768,
            "generated_by": "scripts/generate-transformer-explorer.py",
            "rounding": "floats rounded to 3 decimals; attention rows "
                        "renormalized after rounding",
        },
        "sentences": sentences,
    }
    with open(out_path, "w", encoding="utf-8") as f:
        json.dump(data, f, ensure_ascii=False, separators=(",", ":"))
        f.write("\n")
    print(f"wrote {out_path}", file=sys.stderr)


def check(path):
    with open(path, encoding="utf-8") as f:
        data = json.load(f)
    errors = []
    for s in data["sentences"]:
        t = len(s["tokens"])
        if t > MAX_TOKENS:
            errors.append(f"{s['id']}: {t} tokens > {MAX_TOKENS}")
        if len(s["emb_pca"]) != t or any(len(p) != 2 for p in s["emb_pca"]):
            errors.append(f"{s['id']}: emb_pca shape wrong")
        if len(s["emb_slice"]) != t or any(
            len(r) != SLICE_DIMS for r in s["emb_slice"]
        ):
            errors.append(f"{s['id']}: emb_slice shape wrong")
        if len(s["res_norms"]) != 13 or any(
            len(r) != t for r in s["res_norms"]
        ):
            errors.append(f"{s['id']}: res_norms shape wrong")
        if len(s["logit_lens"]) != 13 or any(
            len(r) != 3 for r in s["logit_lens"]
        ):
            errors.append(f"{s['id']}: logit_lens shape wrong")
        if len(s["top50"]) != TOP_K:
            errors.append(f"{s['id']}: top50 has {len(s['top50'])} entries")
        if len(s["attn"]) != 12:
            errors.append(f"{s['id']}: attn layers != 12")
        for li, layer in enumerate(s["attn"]):
            if len(layer) != 12:
                errors.append(f"{s['id']}: layer {li} heads != 12")
                continue
            for hi, head in enumerate(layer):
                if len(head) != t or any(len(r) != t for r in head):
                    errors.append(
                        f"{s['id']}: attn[{li}][{hi}] shape wrong"
                    )
                    continue
                for qi, row in enumerate(head):
                    total = sum(row)
                    if abs(total - 1.0) > 1e-3:
                        errors.append(
                            f"{s['id']}: attn[{li}][{hi}] row {qi} "
                            f"sums to {total}"
                        )
                    for ki, v in enumerate(row):
                        if ki > qi and v != 0.0:
                            errors.append(
                                f"{s['id']}: attn[{li}][{hi}] "
                                f"[{qi}][{ki}] = {v} above diagonal"
                            )
                        if math.isnan(v):
                            errors.append(f"{s['id']}: NaN in attention")
        # The logit lens at the final layer applies the model's own
        # head to the model's own last hidden state, so its top guess
        # must be the greedy next token from the full distribution.
        if s["logit_lens"][12][0]["id"] != s["top50"][0]["id"]:
            errors.append(
                f"{s['id']}: logit lens layer 12 top "
                f"{s['logit_lens'][12][0]['t']!r} != greedy "
                f"{s['top50'][0]['t']!r}"
            )
        # Softmax over top50 + tail reproduces a distribution; greedy
        # token is the top50 argmax by construction of topk order.
        logits = [e["logit"] for e in s["top50"]]
        if logits != sorted(logits, reverse=True):
            errors.append(f"{s['id']}: top50 not sorted descending")
        m = max(logits + [s["tail_logit"]])
        z = sum(math.exp(v - m) for v in logits) + math.exp(
            s["tail_logit"] - m
        )
        p_top1 = math.exp(logits[0] - m) / z
        if not (0.0 < p_top1 < 1.0):
            errors.append(f"{s['id']}: top1 probability {p_top1} invalid")
    if errors:
        for e in errors[:40]:
            print(f"CHECK FAIL: {e}", file=sys.stderr)
        print(f"{len(errors)} error(s)", file=sys.stderr)
        return 1
    print(
        f"check OK: {len(data['sentences'])} sentences, all shapes, "
        "row sums, causal zeros, and sort orders verified"
    )
    return 0


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out")
    ap.add_argument("--check")
    args = ap.parse_args()
    if args.check:
        return check(args.check)
    if not args.out:
        ap.error("--out is required unless --check is given")
    generate(args.out)
    return check(args.out)


if __name__ == "__main__":
    sys.exit(main())
