use std::{
    error::Error,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use thruster::{
    context::typed_hyper_context::TypedHyperContext, m, middleware_fn, App, Context, HyperRequest,
    HyperServer, MiddlewareNext, MiddlewareResult, ThrusterServer,
};

pub type Ctx = TypedHyperContext<RequestConfig>;

pub struct ServerConfig {
    pub req_counter: Arc<AtomicUsize>,
    pub name: String,
}

#[derive(Clone, Default)]
pub struct RequestConfig {
    pub req_counter: Arc<AtomicUsize>,
    pub name: String,
}

fn generate_context(request: HyperRequest, state: &ServerConfig, _path: &str) -> Ctx {
    Ctx::new(
        request,
        RequestConfig {
            req_counter: state.req_counter.clone(),
            name: state.name.clone(),
        },
    )
}

#[middleware_fn]
async fn ping(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    context.set("Content-Type", "application/json");
    context.body(&format!(
        "{{
    requests: {}
}}",
        context.extra.req_counter.load(Ordering::Relaxed)
    ));

    Ok(context)
}

#[middleware_fn]
async fn stats(context: Ctx, next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    context.extra.req_counter.fetch_add(1, Ordering::Relaxed);

    next(context).await
}

#[middleware_fn]
async fn name(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    context.set("Content-Type", "application/json");
    context.body(&format!(
        "{{
    name: {}
}}",
        context.extra.name
    ));

    Ok(context)
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let app = App::<HyperRequest, Ctx, ServerConfig>::create(
        generate_context,
        ServerConfig {
            req_counter: Arc::new(AtomicUsize::new(0)),
            name: "Obi Wan".to_string(),
        },
    )
    .get("/ping", m![stats, ping])
    .get("/name", m![name]);

    let server = HyperServer::new(app);
    server
        .build(
            "0.0.0.0",
            std::env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse::<u16>()
                .expect("Could not parse PORT"),
        )
        .await;

    Ok(())
}
