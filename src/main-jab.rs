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
use thruster_jab::{fetch, provide, JabDI};

pub type Ctx = TypedHyperContext<RequestConfig>;

pub struct ServerConfig {
    pub jab: Arc<JabDI>,
}

#[derive(Clone, Default)]
pub struct RequestConfig {
    pub jab: Arc<JabDI>,
}

trait Counter {
    fn incr(&self);
    fn get(&self) -> usize;
}

#[derive(Default)]
struct _Counter {
    val: AtomicUsize,
}

impl Counter for _Counter {
    fn incr(&self) {
        self.val.fetch_add(1, Ordering::Relaxed);
    }

    fn get(&self) -> usize {
        self.val.load(Ordering::Relaxed)
    }
}

trait StaticConfig {
    fn get_name(&self) -> String;
}

#[derive(Default)]
struct _StaticConfig {
    name: String,
}

impl StaticConfig for _StaticConfig {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

fn generate_context(request: HyperRequest, state: &ServerConfig, _path: &str) -> Ctx {
    Ctx::new(
        request,
        RequestConfig {
            jab: state.jab.clone(),
        },
    )
}

#[middleware_fn]
async fn ping(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let counter = fetch!(context.extra.jab, dyn Counter).get();

    context.set("Content-Type", "application/json");
    context.body(&format!(
        "{{
    requests: {}
}}",
        counter
    ));

    Ok(context)
}

#[middleware_fn]
async fn stats(context: Ctx, next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    fetch!(context.extra.jab, dyn Counter).incr();

    next(context).await
}

#[middleware_fn]
async fn name(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let name = fetch!(context.extra.jab, dyn StaticConfig).get_name();

    context.set("Content-Type", "application/json");
    context.body(&format!(
        "{{
    name: {}
}}",
        name
    ));

    Ok(context)
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let mut jab = JabDI::default();

    provide!(jab, dyn Counter, _Counter::default());
    provide!(
        jab,
        dyn StaticConfig,
        _StaticConfig {
            name: "Obi Wan".to_string()
        }
    );

    let app = App::<HyperRequest, Ctx, ServerConfig>::create(
        generate_context,
        ServerConfig { jab: Arc::new(jab) },
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
