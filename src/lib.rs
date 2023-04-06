pub struct WorkerLayer {
    level: tracing::Level,
}

impl WorkerLayer {
    pub fn new(level: tracing::Level) -> Self {
        Self { level }
    }
}

impl<S: tracing::Subscriber> tracing_subscriber::Layer<S> for WorkerLayer {
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        if event.metadata().level() > &self.level {
            return;
        }
        let date = worker::Date::now().to_string();
        let level = event.metadata().level();
        let target = event.metadata().target();
        let name = event.metadata().name();
        let fields = event
            .fields()
            .map(|field| format!("{}={:?}", field.name(), field))
            .collect::<Vec<_>>()
            .join(", ");
        worker::console_log!("{date}: [{level}] [{target}] [{name}] {fields}");
    }
}

static INIT: std::sync::Once = std::sync::Once::new();

pub fn init(env: &worker::Env) {
    use tracing_subscriber::prelude::*;

    INIT.call_once(|| {
        let level = env
            .secret("RUST_LOG")
            .map(|s| s.to_string())
            .unwrap_or("info".to_string())
            .parse::<tracing::Level>()
            .unwrap_or(tracing::Level::INFO);
        let subscriber = tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().without_time())
            .with(WorkerLayer::new(level));
        tracing::subscriber::set_global_default(subscriber).unwrap();
    });
}
