use tracing::{Instrument, Subscriber, subscriber::set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tokio::task::JoinHandle;

pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
    where
        Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name,sink);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}

/*
 * Choosing between spawn and spawn_blocking
 * If your database operation (save_executions_to_database) is asynchronous (i.e., it uses await for database queries and does not block the running thread),
 * you should use tokio::spawn.
 * This is because your operation will not block the event loop, allowing other tasks to run concurrently without interference.
 * If your database operation is synchronous (i.e., it blocks the thread until it completes), then you should use tokio::spawn_blocking.
 * This moves the blocking operation off the main event loop threads, so it doesn't interfere with other asynchronous tasks that your application is handling.
 */

/// tokio::spawn_blocking: This function is used for running operations that are blocking in nature in a way that doesn't block the asynchronous runtime.
/// It's typically used for CPU-bound tasks or synchronous operations that would block the thread they're running on (e.g., long computations, synchronous I/O operations).
pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}

/// tokio::spawn: This function is used to run an asynchronous task.
/// It's suitable for operations that are non-blocking and can be awaited.
/// This is the go-to method when you're dealing with asynchronous code that you want to execute in the background,
/// such as making non-blocking I/O operations (e.g., asynchronous database queries, HTTP requests).
pub fn spawn_with_tracing<Fut>(future: Fut) -> tokio::task::JoinHandle<Fut::Output>
where
    Fut: std::future::Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    let current_span = tracing::Span::current();
    tokio::spawn(async move {
        future.instrument(current_span).await
    })
}

