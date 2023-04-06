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
        let level = event.metadata().level();
        let target = event.metadata().target();
        let name = event.metadata().name();
        let fields = event.metadata().fields();
        worker::console_log!("[{level}] [{target}] [{name}] {fields}");
    }
}
