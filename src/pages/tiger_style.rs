//! "Standards" — the standing orders page, `/standards`.
//!
//! One editorial page: manifesto, lineage, principles, then all
//! seventeen language guides as expandable chapters rendered from
//! `crate::tiger_data` (Matt's own guides, verbatim). Full references
//! live on quiet per-language pages linked only from their chapter —
//! the chapter HTML plus seventeen references would push this page
//! far past a sane weight. No chapter needs JavaScript: `<details>`
//! is native, and everything is in the prerendered HTML.
use crate::tiger_data::{tiger_reference, TIGER_GUIDES};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

/// The badge every chapter icon shares.
///
/// A rounded tile, a gradient ring (the `ts-ring-grad` gradient
/// defined once beside the chapters), a soft halo, and a dashed
/// orbit ring. Inside sits the project's own mark — see
/// `icon_glyph` — recolored into the Qompass palette. The ring
/// and halo stay still at rest; the stylesheet turns them only
/// while the chapter's summary is hovered, focused, or open.
const ICON_BADGE: &str = r#"<rect x="3" y="3" width="42" height="42" rx="11" fill="var(--chart-raise)" stroke="url(#ts-ring-grad)" stroke-width="1.6"/><circle class="ts-icon-halo" cx="24" cy="24" r="11" fill="var(--cherenkov)"/><circle class="ts-icon-ring" cx="24" cy="24" r="17.5" fill="none" stroke="var(--cherenkov)" stroke-opacity=".5" stroke-width="1.3" stroke-dasharray="3.5 5.5"/>"#;

/// The mark at the centre of one chapter's icon badge, by slug.
///
/// Each is the language or tool's own official logo, paths taken
/// verbatim from the project's published artwork and recolored
/// into the Qompass palette — the `ts-icon-grad` gradient for
/// the monochrome marks, flat palette tones where the original
/// is multi-color (Python's two snakes, Mojo's flame). Sources:
/// simple-icons 15.16.1 for most; Python from the official
/// logo (Wikimedia Commons, `Python-logo-notext.svg`); Odin
/// from the odin-lang/artwork emblem; Mojo's fire from
/// Modular's wordmark asset on mojolang.org. WGSL is the
/// exception: the W3C and the `GPUWeb` group publish a WebGPU
/// logo but no WGSL mark of its own, so the chapter keeps a
/// drawn wave glyph rather than borrow WebGPU's.
fn icon_glyph(slug: &str) -> &'static str {
    match slug {
        "aseprite" => {
            r#"<g transform="translate(13.5,13.5) scale(0.875)"><path fill="url(#ts-icon-grad)" d="M4.006 0v1.6h15.988V0zm15.988 1.6v1.6h1.6V1.6zm1.6 1.6v14.4h-1.6v1.6H4.006v-1.6h-1.6V3.2H.809v17.6h1.599v1.6h1.599V24h15.988v-1.6h1.6v-1.6h1.598V3.2zm-19.187 0h1.599V1.6h-1.6zm4.796 3.2v6.4h1.6V6.4zm7.995 0v6.4h1.599V6.4z"/></g>"#
        }
        "bash" => {
            r#"<g transform="translate(13.5,13.5) scale(0.875)"><path fill="url(#ts-icon-grad)" d="M21.038,4.9l-7.577-4.498C13.009,0.134,12.505,0,12,0c-0.505,0-1.009,0.134-1.462,0.403L2.961,4.9 C2.057,5.437,1.5,6.429,1.5,7.503v8.995c0,1.073,0.557,2.066,1.462,2.603l7.577,4.497C10.991,23.866,11.495,24,12,24 c0.505,0,1.009-0.134,1.461-0.402l7.577-4.497c0.904-0.537,1.462-1.529,1.462-2.603V7.503C22.5,6.429,21.943,5.437,21.038,4.9z M15.17,18.946l0.013,0.646c0.001,0.078-0.05,0.167-0.111,0.198l-0.383,0.22c-0.061,0.031-0.111-0.007-0.112-0.085L14.57,19.29 c-0.328,0.136-0.66,0.169-0.872,0.084c-0.04-0.016-0.057-0.075-0.041-0.142l0.139-0.584c0.011-0.046,0.036-0.092,0.069-0.121 c0.012-0.011,0.024-0.02,0.036-0.026c0.022-0.011,0.043-0.014,0.062-0.006c0.229,0.077,0.521,0.041,0.802-0.101 c0.357-0.181,0.596-0.545,0.592-0.907c-0.003-0.328-0.181-0.465-0.613-0.468c-0.55,0.001-1.064-0.107-1.072-0.917 c-0.007-0.667,0.34-1.361,0.889-1.8l-0.007-0.652c-0.001-0.08,0.048-0.168,0.111-0.2l0.37-0.236 c0.061-0.031,0.111,0.007,0.112,0.087l0.006,0.653c0.273-0.109,0.511-0.138,0.726-0.088c0.047,0.012,0.067,0.076,0.048,0.151 l-0.144,0.578c-0.011,0.044-0.036,0.088-0.065,0.116c-0.012,0.012-0.025,0.021-0.038,0.028c-0.019,0.01-0.038,0.013-0.057,0.009 c-0.098-0.022-0.332-0.073-0.699,0.113c-0.385,0.195-0.52,0.53-0.517,0.778c0.003,0.297,0.155,0.387,0.681,0.396 c0.7,0.012,1.003,0.318,1.01,1.023C16.105,17.747,15.736,18.491,15.17,18.946z M19.143,17.859c0,0.06-0.008,0.116-0.058,0.145 l-1.916,1.164c-0.05,0.029-0.09,0.004-0.09-0.056v-0.494c0-0.06,0.037-0.093,0.087-0.122l1.887-1.129 c0.05-0.029,0.09-0.004,0.09,0.056V17.859z M20.459,6.797l-7.168,4.427c-0.894,0.523-1.553,1.109-1.553,2.187v8.833 c0,0.645,0.26,1.063,0.66,1.184c-0.131,0.023-0.264,0.039-0.398,0.039c-0.42,0-0.833-0.114-1.197-0.33L3.226,18.64 c-0.741-0.44-1.201-1.261-1.201-2.142V7.503c0-0.881,0.46-1.702,1.201-2.142l7.577-4.498c0.363-0.216,0.777-0.33,1.197-0.33 c0.419,0,0.833,0.114,1.197,0.33l7.577,4.498c0.624,0.371,1.046,1.013,1.164,1.732C21.686,6.557,21.12,6.411,20.459,6.797z"/></g>"#
        }
        "c" => {
            r#"<g transform="translate(13.5,13.5) scale(0.875)"><path fill="url(#ts-icon-grad)" d="M16.5921 9.1962s-.354-3.298-3.627-3.39c-3.2741-.09-4.9552 2.474-4.9552 6.14 0 3.6651 1.858 6.5972 5.0451 6.5972 3.184 0 3.5381-3.665 3.5381-3.665l6.1041.365s.36 3.31-2.196 5.836c-2.552 2.5241-5.6901 2.9371-7.8762 2.9201-2.19-.017-5.2261.034-8.1602-2.97-2.938-3.0101-3.436-5.9302-3.436-8.8002 0-2.8701.556-6.6702 4.047-9.5502C7.444.72 9.849 0 12.254 0c10.0422 0 10.7172 9.2602 10.7172 9.2602z"/></g>"#
        }
        "cpp" => {
            r#"<g transform="translate(13.5,13.5) scale(0.875)"><path fill="url(#ts-icon-grad)" d="M22.394 6c-.167-.29-.398-.543-.652-.69L12.926.22c-.509-.294-1.34-.294-1.848 0L2.26 5.31c-.508.293-.923 1.013-.923 1.6v10.18c0 .294.104.62.271.91.167.29.398.543.652.69l8.816 5.09c.508.293 1.34.293 1.848 0l8.816-5.09c.254-.147.485-.4.652-.69.167-.29.27-.616.27-.91V6.91c.003-.294-.1-.62-.268-.91zM12 19.11c-3.92 0-7.109-3.19-7.109-7.11 0-3.92 3.19-7.11 7.11-7.11a7.133 7.133 0 016.156 3.553l-3.076 1.78a3.567 3.567 0 00-3.08-1.78A3.56 3.56 0 008.444 12 3.56 3.56 0 0012 15.555a3.57 3.57 0 003.08-1.778l3.078 1.78A7.135 7.135 0 0112 19.11zm7.11-6.715h-.79v.79h-.79v-.79h-.79v-.79h.79v-.79h.79v.79h.79zm2.962 0h-.79v.79h-.79v-.79h-.79v-.79h.79v-.79h.79v.79h.79z"/></g>"#
        }
        "go" => {
            r#"<g transform="translate(12.75,12.59) scale(0.9375)"><path fill="url(#ts-icon-grad)" d="M1.811 10.231c-.047 0-.058-.023-.035-.059l.246-.315c.023-.035.081-.058.128-.058h4.172c.046 0 .058.035.035.07l-.199.303c-.023.036-.082.07-.117.07zM.047 11.306c-.047 0-.059-.023-.035-.058l.245-.316c.023-.035.082-.058.129-.058h5.328c.047 0 .07.035.058.07l-.093.28c-.012.047-.058.07-.105.07zm2.828 1.075c-.047 0-.059-.035-.035-.07l.163-.292c.023-.035.07-.07.117-.07h2.337c.047 0 .07.035.07.082l-.023.28c0 .047-.047.082-.082.082zm12.129-2.36c-.736.187-1.239.327-1.963.514-.176.046-.187.058-.34-.117-.174-.199-.303-.327-.548-.444-.737-.362-1.45-.257-2.115.175-.795.514-1.204 1.274-1.192 2.22.011.935.654 1.706 1.577 1.835.795.105 1.46-.175 1.987-.77.105-.13.198-.27.315-.434H10.47c-.245 0-.304-.152-.222-.35.152-.362.432-.97.596-1.274a.315.315 0 01.292-.187h4.253c-.023.316-.023.631-.07.947a4.983 4.983 0 01-.958 2.29c-.841 1.11-1.94 1.8-3.33 1.986-1.145.152-2.209-.07-3.143-.77-.865-.655-1.356-1.52-1.484-2.595-.152-1.274.222-2.419.993-3.424.83-1.086 1.928-1.776 3.272-2.02 1.098-.2 2.15-.07 3.096.571.62.41 1.063.97 1.356 1.648.07.105.023.164-.117.2m3.868 6.461c-1.064-.024-2.034-.328-2.852-1.029a3.665 3.665 0 01-1.262-2.255c-.21-1.32.152-2.489.947-3.529.853-1.122 1.881-1.706 3.272-1.95 1.192-.21 2.314-.095 3.33.595.923.63 1.496 1.484 1.648 2.605.198 1.578-.257 2.863-1.344 3.962-.771.783-1.718 1.273-2.805 1.495-.315.06-.63.07-.934.106zm2.78-4.72c-.011-.153-.011-.27-.034-.387-.21-1.157-1.274-1.81-2.384-1.554-1.087.245-1.788.935-2.045 2.033-.21.912.234 1.835 1.075 2.21.643.28 1.285.244 1.905-.07.923-.48 1.425-1.228 1.484-2.233z"/></g>"#
        }
        "kotlin" => {
            r#"<g transform="translate(13.5,13.5) scale(0.875)"><path fill="url(#ts-icon-grad)" d="M24 24H0V0h24L12 12Z"/></g>"#
        }
        "lua" => {
            r#"<g transform="translate(13.5,13.5) scale(0.875)"><path fill="url(#ts-icon-grad)" d="M.38 10.377l-.272-.037c-.048.344-.082.695-.101 1.041l.275.016c.018-.34.051-.682.098-1.02zM4.136 3.289l-.184-.205c-.258.232-.509.48-.746.734l.202.188c.231-.248.476-.49.728-.717zM5.769 2.059l-.146-.235c-.296.186-.586.385-.863.594l.166.219c.27-.203.554-.399.843-.578zM1.824 18.369c.185.297.384.586.593.863l.22-.164c-.205-.271-.399-.555-.58-.844l-.233.145zM1.127 16.402l-.255.104c.129.318.274.635.431.943l.005.01.245-.125-.005-.01c-.153-.301-.295-.611-.421-.922zM.298 9.309l.269.063c.076-.332.168-.664.272-.986l-.261-.087c-.108.332-.202.672-.28 1.01zM.274 12.42l-.275.01c.012.348.04.699.083 1.043l.273-.033c-.042-.336-.069-.68-.081-1.02zM.256 14.506c.073.34.162.682.264 1.014l.263-.08c-.1-.326-.187-.658-.258-.99l-.269.056zM11.573.275L11.563 0c-.348.012-.699.039-1.044.082l.034.273c.338-.041.68-.068 1.02-.08zM23.221 8.566c.1.326.186.66.256.992l.27-.059c-.072-.34-.16-.682-.262-1.014l-.264.081zM17.621 1.389c-.309-.164-.627-.314-.947-.449l-.107.252c.314.133.625.281.926.439l.128-.242zM15.693.572c-.332-.105-.67-.199-1.01-.277l-.063.268c.332.076.664.168.988.273l.085-.264zM6.674 1.545c.298-.15.606-.291.916-.418L7.486.873c-.317.127-.632.272-.937.428l-.015.008.125.244.015-.008zM23.727 11.588l.275-.01a11.797 11.797 0 0 0-.082-1.045l-.273.033c.041.338.068.682.08 1.022zM13.654.105c-.346-.047-.696-.08-1.043-.098l-.014.273c.339.018.683.051 1.019.098l.038-.273zM9.544.527l-.058-.27c-.34.072-.681.16-1.014.264l.081.262c.325-.099.659-.185.991-.256zM1.921 5.469l.231.15c.185-.285.384-.566.592-.834l-.217-.17c-.213.276-.417.563-.606.854zM.943 7.318l.253.107c.132-.313.28-.625.439-.924l-.243-.128c-.163.307-.314.625-.449.945zM18.223 21.943l.145.234c.295-.186.586-.385.863-.594l-.164-.219c-.272.204-.557.4-.844.579zM21.248 19.219l.217.17c.215-.273.418-.561.607-.854l-.23-.148c-.186.285-.385.564-.594.832zM19.855 20.715l.184.203c.258-.23.51-.479.746-.732l-.201-.188c-.23.248-.477.488-.729.717zM22.359 17.504l.244.129c.162-.307.314-.625.449-.945l-.254-.107a11.27 11.27 0 0 1-.439.923zM23.617 13.629l.273.039c.049-.346.082-.695.102-1.043l-.275-.014c-.018.338-.051.682-.1 1.018zM23.156 15.621l.264.086c.107-.332.201-.67.279-1.01l-.268-.063c-.077.333-.169.665-.275.987zM22.453 6.672c.154.303.297.617.424.932l.256-.104c-.131-.322-.277-.643-.436-.953l-.244.125zM8.296 23.418c.331.107.67.201 1.009.279l.062-.268c-.331-.076-.663-.168-.986-.273l-.085.262zM10.335 23.889c.345.049.696.082 1.043.102l.014-.275c-.339-.018-.682-.051-1.019-.098l-.038.271zM17.326 22.449c-.303.154-.613.297-.926.424l.104.256c.318-.131.639-.275.947-.434l.004-.002-.123-.246-.006.002zM4.613 21.467c.274.213.562.418.854.605l.149-.23c-.285-.184-.565-.385-.833-.592l-.17.217zM12.417 23.725l.009.275c.348-.014.699-.041 1.045-.084l-.035-.271c-.336.041-.68.068-1.019.08zM6.37 22.604c.307.162.625.314.946.449l.107-.254c-.313-.133-.624-.279-.924-.439l-.129.244zM3.083 20.041c.233.258.48.51.734.746l.188-.201c-.249-.23-.49-.477-.717-.729l-.205.184zM14.445 23.475l.059.27c.34-.074.68-.162 1.014-.266l-.082-.262c-.325.099-.659.185-.991.258zM21.18.129A2.689 2.689 0 1 0 21.18 5.507 2.689 2.689 0 1 0 21.18.129zM15.324 15.447c0 .471.314.66.852.66.67 0 1.297-.396 1.297-1.016v-.645c-.23.107-.379.141-1.107.24-.735.109-1.042.306-1.042.761zM12 2.818c-5.07 0-9.18 4.109-9.18 9.18 0 5.068 4.11 9.18 9.18 9.18 5.07 0 9.18-4.111 9.18-9.18 0-5.07-4.11-9.18-9.18-9.18zm-2.487 13.77H5.771v-6.023h.769v5.346h2.974v.677zm4.13 0h-.619v-.67c-.405.57-.811.793-1.446.793-.843 0-1.38-.463-1.38-1.182v-3.271h.686v3c0 .52.347.85.893.85.719 0 1.181-.578 1.181-1.461v-2.389h.686v4.33zm-.53-8.393c0-1.484 1.205-2.689 2.689-2.689s2.688 1.205 2.688 2.689-1.203 2.688-2.688 2.688-2.689-1.203-2.689-2.688zm5.567 7.856v.52c-.223.059-.33.074-.471.074-.34 0-.637-.238-.711-.57-.381.406-.918.637-1.471.637-.877 0-1.422-.463-1.422-1.248 0-.527.256-.916.76-1.123.266-.107.414-.141 1.389-.264.545-.066.719-.191.719-.48v-.182c0-.412-.348-.645-.967-.645-.645 0-.957.24-1.016.77h-.693c.041-1 .686-1.404 1.734-1.404 1.066 0 1.627.412 1.627 1.182v2.412c0 .215.133.338.373.338.041-.002.074-.002.149-.017z"/></g>"#
        }
        "mojo" => {
            r##"<g transform="translate(-37.23,11.4) scale(0.86)"><path d="M79.2321 12.6961C79.4347 13.4689 78.572 14.293 78.572 14.293C78.572 14.293 76.9992 15.2958 77.306 10.993C77.6128 6.69032 70.5651 2.46143 70.5651 2.46143C70.5651 2.46143 70.6053 2.8716 70.5632 5.708C70.5102 9.27747 68.3808 8.05374 68.3808 8.05374C66.2492 10.253 66.443 15.2195 66.443 15.2195C63.8042 17.4671 63.7117 12.794 63.7117 12.794C63.7117 12.794 58.6875 17.6808 64.0605 22.9161C64.0605 22.9161 64.8484 23.6746 63.5892 23.4332C62.6912 23.2612 62.4883 22.9151 62.4883 22.9151C62.4883 22.9151 62.6597 25.0822 68.9953 26.4386C69.1182 26.4626 71.0873 26.836 73.3114 26.4086C78.192 25.3528 79.044 23.3911 79.044 23.3911C79.044 23.3911 78.9407 23.5682 78.1309 23.6473C77.2606 23.7323 78.0077 22.9241 78.0077 22.9241C83.742 19.4562 79.0296 11.9232 79.2321 12.6961Z" fill="#7C5CFF"/><path d="M77.1449 19.8332C78.2872 18.1103 77.4129 16.8261 77.4129 16.8261C75.6233 17.7119 75.0885 17.0924 74.9299 16.5903C74.8672 16.3915 74.8566 16.1804 74.893 15.9757C76.0545 9.43627 71.5631 5.90137 71.6754 6.77467C71.7095 7.03869 71.6623 7.38522 71.5792 7.74106C71.3829 8.58355 71.0078 9.37699 70.4966 10.0858C69.7926 11.0621 68.5203 13.1564 67.8277 16.307C66.8364 20.8171 63.9854 17.5515 63.9854 17.5515C63.9854 17.5515 62.3149 23.0489 69.122 26.4613C69.5229 26.5285 70.7915 26.7047 72.3209 26.5521C76.7936 24.5257 78.3996 20.5673 78.3996 20.5673C78.3996 20.5673 76.0026 21.556 77.1449 19.8332Z" fill="#5B8CFF"/><path d="M75.5851 18.6611C75.5858 18.6593 75.5862 18.6587 75.5862 18.6587L75.5851 18.6611Z" fill="#C9A227"/><path d="M75.5851 18.6611C75.5619 18.7183 75.0823 19.8392 73.3031 20.1241C71.4773 20.4163 72.2797 14.2892 72.2797 14.2892C67.7901 16.3366 68.6239 21.148 68.6239 21.148C68.6239 21.148 67.9066 21.8144 66.9881 21.1471C66.6761 20.9205 67.3591 25.0131 70.2929 26.5885C70.8374 26.6215 71.5174 26.6282 72.2609 26.5577C74.2807 25.206 75.3565 20.9686 75.3565 20.9686L74.43 21.5194L75.5851 18.6611Z" fill="#C9A227"/><path d="M62.6954 12.1968C62.6954 12.1968 62.4336 9.97873 63.7164 8.99973C63.9168 8.84671 64.0519 8.72262 64.1507 8.49529C64.2313 8.31058 64.3562 8.01702 64.2955 7.79726C64.1439 7.24686 66.0057 10.2918 62.6954 12.1968Z" fill="#7C5CFF"/><path d="M72.2747 26.5567C73.0905 25.7316 74.1795 22.4334 74.1795 22.4334C74.1795 22.4334 73.9436 23.1091 73.0634 23.0197C71.8528 22.8964 71.189 19.7913 71.189 19.7913C71.189 19.7913 70.1815 22.431 69.9699 22.8221C69.5243 23.6458 68.7019 22.8213 68.7019 22.8213C68.4836 24.1635 70.2186 26.0874 70.7166 26.6077C71.1783 26.6203 71.708 26.611 72.2747 26.5567Z" fill="#E8EEF7"/></g>"##
        }
        "nix" => {
            r#"<g transform="translate(13.5,13.5) scale(0.875)"><path fill="url(#ts-icon-grad)" d="M7.352 1.592l-1.364.002L5.32 2.75l1.557 2.713-3.137-.008-1.32 2.34H14.11l-1.353-2.332-3.192-.006-2.214-3.865zm6.175 0l-2.687.025 5.846 10.127 1.341-2.34-1.59-2.765 2.24-3.85-.683-1.182h-1.336l-1.57 2.705-1.56-2.72zm6.887 4.195l-5.846 10.125 2.696-.008 1.601-2.76 4.453.016.682-1.183-.666-1.157-3.13-.008L21.778 8.1l-1.365-2.313zM9.432 8.086l-2.696.008-1.601 2.76-4.453-.016L0 12.02l.666 1.157 3.13.008-1.575 2.71 1.365 2.315L9.432 8.086zM7.33 12.25l-.006.01-.002-.004-1.342 2.34 1.59 2.765-2.24 3.85.684 1.182H7.35l.004-.006h.001l1.567-2.698 1.558 2.72 2.688-.026-.004-.006h.01L7.33 12.25zm2.55 3.93l1.354 2.332 3.192.006 2.215 3.865 1.363-.002.668-1.156-1.557-2.713 3.137.008 1.32-2.34H9.881Z"/></g>"#
        }
        "odin" => {
            r#"<g transform="translate(13.25,13.25) scale(0.0827)"><g transform="matrix(0.23131,0,0,0.231311,-26.4611,-156.862)"><path fill="url(#ts-icon-grad)" fill-rule="evenodd" clip-rule="evenodd" d="M430.516,1595.76C413.94,1584.38 400.839,1573.36 400.839,1573.36L827.125,835.012C827.125,835.012 844.672,841.52 859.166,848.469C877.436,857.226 892.569,865.763 892.569,865.763C1099.2,985.063 1170.11,1249.68 1050.81,1456.31C931.507,1662.95 666.89,1733.85 460.256,1614.55C460.256,1614.55 445.783,1606.23 430.516,1595.76ZM856.543,928.162L496.282,1552.15C668.477,1651.57 888.991,1592.48 988.408,1420.29C1087.82,1248.09 1028.74,1027.58 856.543,928.162ZM321.659,1487.36C307.172,1466.38 302.015,1456.32 302.015,1456.32C227.54,1326.96 222.078,1162.46 302.018,1024C381.958,885.54 527.146,808.023 676.415,807.839C676.415,807.839 696.568,807.787 713.348,809.394C738.628,811.816 755.435,815.078 755.435,815.078L347.771,1521.17C347.771,1521.17 338.1,1511.16 321.659,1487.36ZM633.291,882.532C524.653,895.735 423.297,958.044 364.417,1060.03C305.537,1162.01 302.255,1280.94 345.139,1381.63L633.291,882.532Z"/></g></g>"#
        }
        "php" => {
            r#"<g transform="translate(12.75,12.75) scale(0.9375)"><path fill="url(#ts-icon-grad)" d="M7.01 10.207h-.944l-.515 2.648h.838c.556 0 .97-.105 1.242-.314.272-.21.455-.559.55-1.049.092-.47.05-.802-.124-.995-.175-.193-.523-.29-1.047-.29zM12 5.688C5.373 5.688 0 8.514 0 12s5.373 6.313 12 6.313S24 15.486 24 12c0-3.486-5.373-6.312-12-6.312zm-3.26 7.451c-.261.25-.575.438-.917.551-.336.108-.765.164-1.285.164H5.357l-.327 1.681H3.652l1.23-6.326h2.65c.797 0 1.378.209 1.744.628.366.418.476 1.002.33 1.752a2.836 2.836 0 0 1-.305.847c-.143.255-.33.49-.561.703zm4.024.715l.543-2.799c.063-.318.039-.536-.068-.651-.107-.116-.336-.174-.687-.174H11.46l-.704 3.625H9.388l1.23-6.327h1.367l-.327 1.682h1.218c.767 0 1.295.134 1.586.401s.378.7.263 1.299l-.572 2.944h-1.389zm7.597-2.265a2.782 2.782 0 0 1-.305.847c-.143.255-.33.49-.561.703a2.44 2.44 0 0 1-.917.551c-.336.108-.765.164-1.286.164h-1.18l-.327 1.682h-1.378l1.23-6.326h2.649c.797 0 1.378.209 1.744.628.366.417.477 1.001.331 1.751zM17.766 10.207h-.943l-.516 2.648h.838c.557 0 .971-.105 1.242-.314.272-.21.455-.559.551-1.049.092-.47.049-.802-.125-.995s-.524-.29-1.047-.29z"/></g>"#
        }
        "python" => {
            r##"<g transform="translate(13.21,13.27) scale(0.1955)"><path fill="#5B8CFF" d="M55.023-0.077c-25.971,0-26.25,10.081-26.25,12.156c0,3.148,0,12.594,0,12.594h26.75v3.781 c0,0-27.852,0-37.375,0c-7.949,0-17.938,4.833-17.938,26.25c0,19.673,7.792,27.281,15.656,27.281c2.335,0,9.344,0,9.344,0 s0-9.765,0-13.125c0-5.491,2.721-15.656,15.406-15.656c15.91,0,19.971,0,26.531,0c3.902,0,14.906-1.696,14.906-14.406 c0-13.452,0-17.89,0-24.219C82.054,11.426,81.515-0.077,55.023-0.077z M40.273,8.392c2.662,0,4.813,2.15,4.813,4.813 c0,2.661-2.151,4.813-4.813,4.813s-4.813-2.151-4.813-4.813C35.46,10.542,37.611,8.392,40.273,8.392z"/><path fill="#C9A227" d="M55.397,109.923c25.959,0,26.282-10.271,26.282-12.156c0-3.148,0-12.594,0-12.594H54.897v-3.781 c0,0,28.032,0,37.375,0c8.009,0,17.938-4.954,17.938-26.25c0-23.322-10.538-27.281-15.656-27.281c-2.336,0-9.344,0-9.344,0 s0,10.216,0,13.125c0,5.491-2.631,15.656-15.406,15.656c-15.91,0-19.476,0-26.532,0c-3.892,0-14.906,1.896-14.906,14.406 c0,14.475,0,18.265,0,24.219C28.366,100.497,31.562,109.923,55.397,109.923z M70.148,101.454c-2.662,0-4.813-2.151-4.813-4.813 s2.15-4.813,4.813-4.813c2.661,0,4.813,2.151,4.813,4.813S72.809,101.454,70.148,101.454z"/></g>"##
        }
        "rust" => {
            r#"<g transform="translate(13.5,13.5) scale(0.875)"><path fill="url(#ts-icon-grad)" d="M23.8346 11.7033l-1.0073-.6236a13.7268 13.7268 0 00-.0283-.2936l.8656-.8069a.3483.3483 0 00-.1154-.578l-1.1066-.414a8.4958 8.4958 0 00-.087-.2856l.6904-.9587a.3462.3462 0 00-.2257-.5446l-1.1663-.1894a9.3574 9.3574 0 00-.1407-.2622l.49-1.0761a.3437.3437 0 00-.0274-.3361.3486.3486 0 00-.3006-.154l-1.1845.0416a6.7444 6.7444 0 00-.1873-.2268l.2723-1.153a.3472.3472 0 00-.417-.4172l-1.1532.2724a14.0183 14.0183 0 00-.2278-.1873l.0415-1.1845a.3442.3442 0 00-.49-.328l-1.076.491c-.0872-.0476-.1742-.0952-.2623-.1407l-.1903-1.1673A.3483.3483 0 0016.256.955l-.9597.6905a8.4867 8.4867 0 00-.2855-.086l-.414-1.1066a.3483.3483 0 00-.5781-.1154l-.8069.8666a9.2936 9.2936 0 00-.2936-.0284L12.2946.1683a.3462.3462 0 00-.5892 0l-.6236 1.0073a13.7383 13.7383 0 00-.2936.0284L9.9803.3374a.3462.3462 0 00-.578.1154l-.4141 1.1065c-.0962.0274-.1903.0567-.2855.086L7.744.955a.3483.3483 0 00-.5447.2258L7.009 2.348a9.3574 9.3574 0 00-.2622.1407l-1.0762-.491a.3462.3462 0 00-.49.328l.0416 1.1845a7.9826 7.9826 0 00-.2278.1873L3.8413 3.425a.3472.3472 0 00-.4171.4171l.2713 1.1531c-.0628.075-.1255.1509-.1863.2268l-1.1845-.0415a.3462.3462 0 00-.328.49l.491 1.0761a9.167 9.167 0 00-.1407.2622l-1.1662.1894a.3483.3483 0 00-.2258.5446l.6904.9587a13.303 13.303 0 00-.087.2855l-1.1065.414a.3483.3483 0 00-.1155.5781l.8656.807a9.2936 9.2936 0 00-.0283.2935l-1.0073.6236a.3442.3442 0 000 .5892l1.0073.6236c.008.0982.0182.1964.0283.2936l-.8656.8079a.3462.3462 0 00.1155.578l1.1065.4141c.0273.0962.0567.1914.087.2855l-.6904.9587a.3452.3452 0 00.2268.5447l1.1662.1893c.0456.088.0922.1751.1408.2622l-.491 1.0762a.3462.3462 0 00.328.49l1.1834-.0415c.0618.0769.1235.1528.1873.2277l-.2713 1.1541a.3462.3462 0 00.4171.4161l1.153-.2713c.075.0638.151.1255.2279.1863l-.0415 1.1845a.3442.3442 0 00.49.327l1.0761-.49c.087.0486.1741.0951.2622.1407l.1903 1.1662a.3483.3483 0 00.5447.2268l.9587-.6904a9.299 9.299 0 00.2855.087l.414 1.1066a.3452.3452 0 00.5781.1154l.8079-.8656c.0972.0111.1954.0203.2936.0294l.6236 1.0073a.3472.3472 0 00.5892 0l.6236-1.0073c.0982-.0091.1964-.0183.2936-.0294l.8069.8656a.3483.3483 0 00.578-.1154l.4141-1.1066a8.4626 8.4626 0 00.2855-.087l.9587.6904a.3452.3452 0 00.5447-.2268l.1903-1.1662c.088-.0456.1751-.0931.2622-.1407l1.0762.49a.3472.3472 0 00.49-.327l-.0415-1.1845a6.7267 6.7267 0 00.2267-.1863l1.1531.2713a.3472.3472 0 00.4171-.416l-.2713-1.1542c.0628-.0749.1255-.1508.1863-.2278l1.1845.0415a.3442.3442 0 00.328-.49l-.49-1.076c.0475-.0872.0951-.1742.1407-.2623l1.1662-.1893a.3483.3483 0 00.2258-.5447l-.6904-.9587.087-.2855 1.1066-.414a.3462.3462 0 00.1154-.5781l-.8656-.8079c.0101-.0972.0202-.1954.0283-.2936l1.0073-.6236a.3442.3442 0 000-.5892zm-6.7413 8.3551a.7138.7138 0 01.2986-1.396.714.714 0 11-.2997 1.396zm-.3422-2.3142a.649.649 0 00-.7715.5l-.3573 1.6685c-1.1035.501-2.3285.7795-3.6193.7795a8.7368 8.7368 0 01-3.6951-.814l-.3574-1.6684a.648.648 0 00-.7714-.499l-1.473.3158a8.7216 8.7216 0 01-.7613-.898h7.1676c.081 0 .1356-.0141.1356-.088v-2.536c0-.074-.0536-.0881-.1356-.0881h-2.0966v-1.6077h2.2677c.2065 0 1.1065.0587 1.394 1.2088.0901.3533.2875 1.5044.4232 1.8729.1346.413.6833 1.2381 1.2685 1.2381h3.5716a.7492.7492 0 00.1296-.0131 8.7874 8.7874 0 01-.8119.9526zM6.8369 20.024a.714.714 0 11-.2997-1.396.714.714 0 01.2997 1.396zM4.1177 8.9972a.7137.7137 0 11-1.304.5791.7137.7137 0 011.304-.579zm-.8352 1.9813l1.5347-.6824a.65.65 0 00.33-.8585l-.3158-.7147h1.2432v5.6025H3.5669a8.7753 8.7753 0 01-.2834-3.348zm6.7343-.5437V8.7836h2.9601c.153 0 1.0792.1772 1.0792.8697 0 .575-.7107.7815-1.2948.7815zm10.7574 1.4862c0 .2187-.008.4363-.0243.651h-.9c-.09 0-.1265.0586-.1265.1477v.413c0 .973-.5487 1.1846-1.0296 1.2382-.4576.0517-.9648-.1913-1.0275-.4717-.2704-1.5186-.7198-1.8436-1.4305-2.4034.8817-.5599 1.799-1.386 1.799-2.4915 0-1.1936-.819-1.9458-1.3769-2.3153-.7825-.5163-1.6491-.6195-1.883-.6195H5.4682a8.7651 8.7651 0 014.907-2.7699l1.0974 1.151a.648.648 0 00.9182.0213l1.227-1.1743a8.7753 8.7753 0 016.0044 4.2762l-.8403 1.8982a.652.652 0 00.33.8585l1.6178.7188c.0283.2875.0425.577.0425.8717zm-9.3006-9.5993a.7128.7128 0 11.984 1.0316.7137.7137 0 01-.984-1.0316zm8.3389 6.71a.7107.7107 0 01.9395-.3625.7137.7137 0 11-.9405.3635z"/></g>"#
        }
        "scala" => {
            r#"<g transform="translate(13.25,13.25) scale(0.8958)"><path fill="url(#ts-icon-grad)" d="M4.589 24c4.537 0 13.81-1.516 14.821-3v-5.729c-.957 1.408-10.284 2.912-14.821 2.912V24zM4.589 16.365c4.537 0 13.81-1.516 14.821-3V7.636c-.957 1.408-10.284 2.912-14.821 2.912v5.817zM4.589 8.729c4.537 0 13.81-1.516 14.821-3V0C18.453 1.408 9.126 2.912 4.589 2.912v5.817z"/></g>"#
        }
        "typescript" => {
            r#"<g transform="translate(13.5,13.5) scale(0.875)"><path fill="url(#ts-icon-grad)" d="M1.125 0C.502 0 0 .502 0 1.125v21.75C0 23.498.502 24 1.125 24h21.75c.623 0 1.125-.502 1.125-1.125V1.125C24 .502 23.498 0 22.875 0zm17.363 9.75c.612 0 1.154.037 1.627.111a6.38 6.38 0 0 1 1.306.34v2.458a3.95 3.95 0 0 0-.643-.361 5.093 5.093 0 0 0-.717-.26 5.453 5.453 0 0 0-1.426-.2c-.3 0-.573.028-.819.086a2.1 2.1 0 0 0-.623.242c-.17.104-.3.229-.393.374a.888.888 0 0 0-.14.49c0 .196.053.373.156.529.104.156.252.304.443.444s.423.276.696.41c.273.135.582.274.926.416.47.197.892.407 1.266.628.374.222.695.473.963.753.268.279.472.598.614.957.142.359.214.776.214 1.253 0 .657-.125 1.21-.373 1.656a3.033 3.033 0 0 1-1.012 1.085 4.38 4.38 0 0 1-1.487.596c-.566.12-1.163.18-1.79.18a9.916 9.916 0 0 1-1.84-.164 5.544 5.544 0 0 1-1.512-.493v-2.63a5.033 5.033 0 0 0 3.237 1.2c.333 0 .624-.03.872-.09.249-.06.456-.144.623-.25.166-.108.29-.234.373-.38a1.023 1.023 0 0 0-.074-1.089 2.12 2.12 0 0 0-.537-.5 5.597 5.597 0 0 0-.807-.444 27.72 27.72 0 0 0-1.007-.436c-.918-.383-1.602-.852-2.053-1.405-.45-.553-.676-1.222-.676-2.005 0-.614.123-1.141.369-1.582.246-.441.58-.804 1.004-1.089a4.494 4.494 0 0 1 1.47-.629 7.536 7.536 0 0 1 1.77-.201zm-15.113.188h9.563v2.166H9.506v9.646H6.789v-9.646H3.375z"/></g>"#
        }
        "wgsl" => {
            r##"<path d="M12.5 20.5 Q16.5 15.5 20.5 20.5 T28.5 20.5 T36.5 20.5" fill="none" stroke="#5B8CFF" stroke-width="2" stroke-linecap="round"/><path d="M12.5 28 Q16.5 23 20.5 28 T28.5 28 T36.5 28" fill="none" stroke="#7C5CFF" stroke-width="2" stroke-linecap="round"/>"##
        }
        "zig" => {
            r#"<g transform="translate(13.5,13.5) scale(0.875)"><path fill="url(#ts-icon-grad)" d="m23.53 1.02-7.686 3.45h-7.06l-2.98 3.452h7.173L.47 22.98l7.681-3.607h7.065v-.002l2.978-3.45-7.148-.001 12.482-14.9zM0 4.47v14.901h1.883l2.98-3.45H3.451v-8h.942l2.824-3.45H0zm22.117 0-2.98 3.608h1.412v7.844h-.942l-2.98 3.45H24V4.47h-1.883z"/></g>"#
        }
        _ => r##"<circle cx="24" cy="24" r="4" fill="#9DBCFF"/>"##,
    }
}

/// The one-sentence header summary for a chapter, where one is used.
///
/// Three chapters' imported scope lines are long, multi-sentence
/// skill descriptions whose substance the guide body already
/// carries; their headers keep one sentence summarizing the
/// approach instead, and nothing is repeated inside. Every other
/// chapter folds its scope line into the chapter as the lead and
/// its header is name-only.
fn chapter_headline(slug: &str) -> Option<&'static str> {
    match slug {
        "c" => Some("Safety-first C — explicit contracts, checked bounds, and disciplined manual ownership, hardened against overflow and use-after-free."),
        "nix" => Some("Safety-first Nix — pure evaluation, explicit inputs, and pinned sources, validated by a pinned toolchain."),
        "typescript" => Some("Safety-first TypeScript under the strict profile — explicit contracts, strict null handling, and bounded async work."),
        _ => None,
    }
}

/// Reference links for one chapter's body, by slug.
///
/// The official language or tool documentation, the standard or
/// toolchain reference the guide names, and the lineage link where
/// it fits. Every URL verified live when this list was written.
fn chapter_references(slug: &str) -> &'static [(&'static str, &'static str)] {
    match slug {
        "aseprite" => &[
            ("Aseprite", "https://www.aseprite.org/"),
            ("Aseprite CLI docs", "https://www.aseprite.org/docs/cli/"),
            ("Aseprite scripting docs", "https://www.aseprite.org/docs/scripting/"),
        ],
        "bash" => &[
            ("Bash manual", "https://www.gnu.org/software/bash/manual/html_node/index.html"),
            ("ShellCheck", "https://www.shellcheck.net/"),
        ],
        "c" => {
            &[("C reference (cppreference)", "https://en.cppreference.com/w/c"), ("clangd", "https://clangd.llvm.org/")]
        }
        "cpp" => &[
            ("C++ reference (cppreference)", "https://en.cppreference.com/w/cpp"),
            ("C++ Core Guidelines", "https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines"),
        ],
        "go" => &[("Go documentation", "https://go.dev/doc/"), ("Effective Go", "https://go.dev/doc/effective_go")],
        "kotlin" => &[
            ("Kotlin documentation", "https://kotlinlang.org/docs/home.html"),
            ("Kotlin standard library", "https://kotlinlang.org/api/latest/jvm/stdlib/"),
        ],
        "lua" => &[("Lua 5.4 reference manual", "https://www.lua.org/manual/5.4/"), ("LuaJIT", "https://luajit.org/")],
        "mojo" => &[
            ("Mojo documentation", "https://docs.modular.com/mojo/"),
            ("Mojo manual", "https://docs.modular.com/mojo/manual/"),
        ],
        "nix" => &[
            ("Nix manual", "https://nixos.org/manual/nix/stable/"),
            ("Nixpkgs manual", "https://nixos.org/manual/nixpkgs/stable/"),
            ("nix.dev", "https://nix.dev/"),
        ],
        "odin" => {
            &[("Odin", "https://odin-lang.org/"), ("Odin language overview", "https://odin-lang.org/docs/overview/")]
        }
        "php" => &[
            ("PHP manual", "https://www.php.net/manual/en/"),
            ("PSR-12 coding style", "https://www.php-fig.org/psr/psr-12/"),
        ],
        "python" => {
            &[("Python documentation", "https://docs.python.org/3/"), ("PEP 8", "https://peps.python.org/pep-0008/")]
        }
        "rust" => &[
            ("The Rust Book", "https://doc.rust-lang.org/book/"),
            ("Clippy documentation", "https://doc.rust-lang.org/clippy/"),
            ("Rust API Guidelines", "https://rust-lang.github.io/api-guidelines/"),
            ("TigerStyle", "https://tigerstyle.dev/"),
        ],
        "scala" => &[
            ("Scala documentation", "https://docs.scala-lang.org/"),
            ("Scala 3 Book", "https://docs.scala-lang.org/scala3/book/introduction.html"),
        ],
        "typescript" => {
            &[("TypeScript documentation", "https://www.typescriptlang.org/docs/"), ("Biome", "https://biomejs.dev/")]
        }
        "wgsl" => &[("WGSL specification", "https://www.w3.org/TR/WGSL/"), ("Bevy", "https://bevyengine.org/")],
        "zig" => &[
            ("Zig learning resources", "https://ziglang.org/learn/"),
            ("Zig language reference", "https://ziglang.org/documentation/master/"),
            ("TigerStyle", "https://tigerstyle.dev/"),
        ],
        _ => &[],
    }
}

#[component]
pub fn TigerStyle() -> impl IntoView {
    view! {
        <Title text="Standards - Qompass AI" />
        <section class="page tiger-page">
            <p class="page-bearing">"STANDING ORDERS"</p>
            <h1>"Standards"</h1>
            <p class="ts-subhead">"The orders the work is done under."</p>
            <p class="page-lede">
                "Tiger Style is Qompass AI's standing orders for code. Safety first, "
                "everything bounded, nothing hidden — in every language we ship."
            </p>

            <h2 class="band-label">"RESPECT WHERE DUE"</h2>
            <p>
                "This standard descends from "
                <a href="https://tigerstyle.dev/" rel="external">"TigerStyle"</a>
                ", the coding standard written for TigerBeetle — a database that "
                "cannot afford to be wrong — and, behind it, NASA JPL's "
                <a href="https://spinroot.com/gerard/pdf/P10.pdf" rel="external">
                    "Power of Ten"
                </a>
                ", Gerard J. Holzmann's rules for developing safety-critical code. "
                "It is adapted here with respect, and the debt is stated plainly."
            </p>
            <p>
                "What Qompass takes from both: determinism over cleverness — "
                "explicit limits, bounded work, deterministic interfaces. A program "
                "should verify itself while it runs, and shut down rather than run "
                "wrong. And doing it right the first time, because production "
                "rarely grants a second draft."
            </p>

            <h2 class="band-label">"THE ORDERS THEMSELVES"</h2>

            <div class="ts-principle">
                <h3 class="study-subhead">"Safety, then performance, then convenience"</h3>
                <p>
                    "The priority order, always. When the three compete — and they "
                    "compete daily — safety wins outright, performance is argued "
                    "with measurements, and developer experience takes what is left."
                </p>
                <svg class="ts-diagram" viewBox="0 0 560 108" role="img" aria-label="Priority bars: safety full width, performance narrower, developer experience narrowest">
                    <rect class="gfx-panel" x="0" y="0" width="470" height="26" />
                    <rect class="gfx-bar-top" x="0" y="0" width="470" height="26" />
                    <rect class="gfx-panel" x="0" y="41" width="340" height="26" />
                    <rect class="gfx-bar" x="0" y="41" width="340" height="26" />
                    <rect class="gfx-panel" x="0" y="82" width="220" height="26" />
                    <rect x="0" y="82" width="220" height="26" fill="var(--violet)" />
                    <text class="gfx-text" x="482" y="18">"SAFETY"</text>
                    <text class="gfx-text" x="352" y="59">"PERFORMANCE"</text>
                    <text class="gfx-text" x="232" y="100">"DEVELOPER EXPERIENCE"</text>
                </svg>
                <blockquote class="ts-quote">
                    <p>"Priority order, always: Safety > Performance > Developer Experience."</p>
                    <cite>"— the Rust guide, below"</cite>
                </blockquote>
            </div>

            <div class="ts-principle">
                <h3 class="study-subhead">"Contracts first"</h3>
                <p>
                    "Every substantial operation answers, before it is written: "
                    "accepted and rejected inputs, the trust boundary, the maximum "
                    "work, memory, output, and time, the owner of every allocation "
                    "and handle, and what failure and cancellation do. If the "
                    "contract cannot be stated, the operation is not understood "
                    "yet — and is not written yet."
                </p>
                <svg class="ts-diagram" viewBox="0 0 560 92" role="img" aria-label="A contract box with four labeled ports: inputs, bounds, owner, failure">
                    <rect class="gfx-panel" x="180" y="16" width="200" height="60" />
                    <text class="gfx-text" x="280" y="51" text-anchor="middle">"OPERATION"</text>
                    <text class="gfx-note" x="60" y="51">"INPUTS →"</text>
                    <text class="gfx-note" x="436" y="30">"→ BOUNDS"</text>
                    <text class="gfx-note" x="436" y="51">"→ OWNER"</text>
                    <text class="gfx-note" x="436" y="72">"→ FAILURE"</text>
                </svg>
            </div>

            <div class="ts-principle">
                <h3 class="study-subhead">"Bound everything"</h3>
                <p>
                    "Limits are named constants carrying units — output_bytes_max, "
                    "retry_count, queue_capacity, deadline — and index ranges are "
                    "validated with checked arithmetic before an end is computed. "
                    "A bound that lives only in someone's head is not a bound."
                </p>
                <svg class="ts-diagram" viewBox="0 0 560 76" role="img" aria-label="A capacity bar stopped by a brass MAX marker before the end">
                    <rect class="gfx-panel" x="0" y="28" width="470" height="20" />
                    <rect class="gfx-bar" x="0" y="28" width="330" height="20" />
                    <line x1="446" y1="14" x2="446" y2="62" stroke="var(--brass)" stroke-width="2" />
                    <text class="gfx-text" x="482" y="44">"MAX"</text>
                    <text class="gfx-note" x="0" y="70">"work grows to the limit — and stops there, by construction"</text>
                </svg>
                <blockquote class="ts-quote">
                    <p>"No unbounded loops, retries, queues, or growth."</p>
                    <cite>"— the Rust guide, below"</cite>
                </blockquote>
            </div>

            <div class="ts-principle">
                <h3 class="study-subhead">"Errors are data; assertions are law"</h3>
                <p>
                    "Invalid input, configuration, or protocol data gets a typed "
                    "error with bounded context; the caller owns recovery. A "
                    "corrupt internal invariant gets an assertion — the program "
                    "verifies itself while it runs and stops rather than run "
                    "wrong. Nothing unwraps external data, and no required "
                    "mutation hides inside an assertion."
                </p>
                <svg class="ts-diagram" viewBox="0 0 560 96" role="img" aria-label="A fork: bad input returns as a typed error; a broken invariant halts the program">
                    <text class="gfx-note" x="0" y="30">"bad input ───→ typed error ───→ caller recovers"</text>
                    <text class="gfx-note" x="0" y="66">"broken invariant ───→ assertion ───→ halt, loudly"</text>
                    <line x1="0" y1="80" x2="470" y2="80" stroke="var(--chart-line)" />
                </svg>
            </div>

            <div class="ts-principle">
                <h3 class="study-subhead">"Paired tools"</h3>
                <p>
                    "No guide ships alone. Each names its toolchain — language "
                    "server, formatter, linter, debugger — so the standard is "
                    "enforced by machines, not taste. The pairing is in every "
                    "chapter below: rust-analyzer, clippy, and rustfmt for Rust; "
                    "zls and zig fmt for Zig; shellcheck and shfmt for Bash."
                </p>
                <div class="ts-tools">
                    <span class="ts-tool">"LSP — see the contract"</span>
                    <span class="ts-tool">"FORMAT — one shape"</span>
                    <span class="ts-tool">"LINT — zero warnings"</span>
                    <span class="ts-tool">"DEBUG — inspect, don't guess"</span>
                </div>
            </div>

            <h2 class="band-label">"THE ORDERS, BY LANGUAGE"</h2>
            <p class="on-ai-note">
                "Seventeen chapters — the operational core of each language's "
                "guide, verbatim. Open one and read it here; the full reference "
                "behind each chapter is one link further, on its own page."
            </p>
            <svg class="ts-defs" aria-hidden="true">
                <defs>
                    <linearGradient
                        id="ts-ring-grad"
                        x1="0"
                        y1="0"
                        x2="48"
                        y2="48"
                        gradientUnits="userSpaceOnUse"
                    >
                        <stop offset="0" stop-color="#5B8CFF"></stop>
                        <stop offset=".55" stop-color="#7C5CFF"></stop>
                        <stop offset="1" stop-color="#C9A227"></stop>
                    </linearGradient>
                    <linearGradient
                        id="ts-icon-grad"
                        x1="13"
                        y1="13"
                        x2="35"
                        y2="35"
                        gradientUnits="userSpaceOnUse"
                    >
                        <stop offset="0" stop-color="#5FF2C0"></stop>
                        <stop offset=".5" stop-color="#5B8CFF"></stop>
                        <stop offset="1" stop-color="#7C5CFF"></stop>
                    </linearGradient>
                </defs>
            </svg>
            {TIGER_GUIDES
                .iter()
                .map(|g| {
                    let headline = chapter_headline(g.slug);
                    let lead = headline.is_none().then_some(g.scope);
                    let icon = format!("{ICON_BADGE}{}", icon_glyph(g.slug));
                    let refs = chapter_references(g.slug)
                        .iter()
                        .enumerate()
                        .map(|(i, &(label, url))| {
                            view! {
                                {(i > 0)
                                    .then(|| {
                                        view! { <span class="ts-ref-sep">" · "</span> }
                                    })}
                                <a href={url} rel="external">{label}</a>
                            }
                        })
                        .collect_view();
                    view! {
                        <details class="ts-chapter" id={format!("chapter-{}", g.slug)}>
                            <summary class="ts-summary">
                                <svg
                                    class="ts-icon"
                                    viewBox="0 0 48 48"
                                    aria-hidden="true"
                                    inner_html={icon}
                                ></svg>
                                <span class="ts-lang">{g.name}</span>
                                {headline
                                    .map(|h| {
                                        view! { <span class="ts-headline">{h}</span> }
                                    })}
                            </summary>
                            <div class="ts-guide">
                                {lead
                                    .map(|scope| {
                                        view! { <p class="ts-lead">{scope}</p> }
                                    })}
                                <div inner_html={g.html}></div>
                                <p class="ts-refs">
                                    <span class="ts-refs-label">"References"</span>
                                    {refs}
                                </p>
                            </div>
                            {(g.has_reference)
                                .then(|| {
                                    view! {
                                        <p class="ts-ref-link">
                                            <a href={format!("/standards/{}", g.slug)}>
                                                {format!("Read the full {} reference →", g.name)}
                                            </a>
                                        </p>
                                    }
                                })}
                        </details>
                    }
                })
                .collect_view()}

            <p class="ts-closing">
                "These orders are standing: they do not expire between releases, "
                "and they are not suspended for deadlines. Every project on the "
                <a href="/projects">"projects page"</a>
                " was built under them — that is what the phrase means when it "
                "appears in a repository: the work was done under these orders."
            </p>
        </section>
    }
}

/// Quiet standalone page for one language's full reference, linked
/// only from its chapter on `/standards`. Unknown slugs get the
/// site's standard fallback.
#[component]
pub fn TigerStyleDispatch() -> impl IntoView {
    let params = use_params_map();
    let reference = params.with(|map| map.get("slug").and_then(|slug| tiger_reference(slug.as_str())));
    reference.map_or_else(
        || {
            view! {
                <section class="page">
                    <h1>"Off chart"</h1>
                    <p>"That bearing does not exist on this chart."</p>
                </section>
            }
            .into_any()
        },
        |reference| {
            view! {
                <Title text=format!("{} Tiger Style reference - Qompass AI", reference.name) />
                <section class="page tiger-page">
                    <p class="page-bearing">"STANDING ORDERS — FULL REFERENCE"</p>
                    <h1>{format!("{} — the full reference", reference.name)}</h1>
                    <p class="ts-back">
                        <a href="/standards">"← The standing orders"</a>
                    </p>
                    <div class="ts-guide" inner_html={reference.html}></div>
                    <p class="ts-back">
                        <a href="/standards">"← The standing orders"</a>
                    </p>
                </section>
            }
            .into_any()
        },
    )
}
