/// Content-Security-Policy for every response.
///
/// KEEP IN SYNC with `deploy/Caddyfile` — the edge serves the same value,
/// and the end-to-end suite asserts this exact policy on every route.
///
/// The `script-src` hashes cover Leptos' inline hydration bootstrap: its
/// script bodies are byte-identical on every page (a wasm loader plus two
/// empty-state stubs and one empty script), so they can be pinned by hash
/// instead of weakening the policy with 'unsafe-inline'. If a Leptos
/// upgrade changes those bodies, the e2e CSP sweep fails loudly — update
/// the hashes from the built `dist/` HTML (sha256 of each inline script's
/// exact bytes, base64-encoded).
///
/// 'wasm-unsafe-eval' is present on evidence, not speculation: without
/// it the sweep captured `WebAssembly.instantiateStreaming(): Compiling
/// or instantiating WebAssembly module violates the following Content
/// Security policy directive` and hydration was dead. It permits WASM
/// compilation only; JavaScript eval stays forbidden.
#[cfg(feature = "ssr")]
const CONTENT_SECURITY_POLICY: &str = "default-src 'self'; script-src 'self' 'wasm-unsafe-eval' \
    'sha256-ErLwLzrp0nVL4JLFLg4KRpX9lYoIOvcxQy36WvA5dGA=' \
    'sha256-2e+rGxtst+bbxdLf6Dp32aL0uyPdikvr0awq53hVU58=' \
    'sha256-PEoA+SZ6ZbFCusLsUEMftpXvTp6Abhw9kvgQnCC8l3U=' \
    'sha256-47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFU='; \
    object-src 'none'; base-uri 'self'; form-action 'self'; frame-ancestors 'none'; \
    img-src 'self' data:; font-src 'self'; style-src 'self'; style-src-elem 'self'; \
    style-src-attr 'unsafe-inline'; connect-src 'self' https://pub.orcid.org; \
    worker-src 'self'; manifest-src 'self'";

/// Security headers on every response, mirroring `deploy/Caddyfile`.
///
/// The deployed static site is served by Caddy, which sets the same
/// headers at the edge; setting them here too keeps the policy testable
/// against the local server (the e2e suite asserts them on every route)
/// and correct for any future SSR-fronted serving.
#[cfg(feature = "ssr")]
async fn security_headers(request: axum::extract::Request, next: axum::middleware::Next) -> axum::response::Response {
    use axum::http::HeaderValue;
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert("content-security-policy", HeaderValue::from_static(CONTENT_SECURITY_POLICY));
    headers.insert("cross-origin-opener-policy", HeaderValue::from_static("same-origin"));
    headers.insert("cross-origin-resource-policy", HeaderValue::from_static("same-origin"));
    headers.insert(
        "permissions-policy",
        HeaderValue::from_static("camera=(), microphone=(), geolocation=(), payment=(), usb=(), interest-cohort=()"),
    );
    headers.insert("referrer-policy", HeaderValue::from_static("strict-origin-when-cross-origin"));
    headers.insert("strict-transport-security", HeaderValue::from_static("max-age=31536000; includeSubDomains"));
    headers.insert("x-content-type-options", HeaderValue::from_static("nosniff"));
    headers.insert("x-frame-options", HeaderValue::from_static("DENY"));
    response
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use qompass_ai::app::{shell, App};

    let conf = get_configuration(None)?;
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let options = leptos_options.clone();
            move || shell(options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(axum::middleware::from_fn(security_headers))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("listening on http://{addr}");
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[cfg(not(feature = "ssr"))]
fn main() {}
