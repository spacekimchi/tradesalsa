// use sqlx::{Pool, Postgres};
// Pool<Postgres> is similar to PgPool
// PgPool is sqlx's version
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use secrecy::Secret;
use axum::{Extension,Router};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use std::sync::Arc;
use tera::Tera;
use tower_http::services::{ServeDir, ServeFile};
use std::fs;
use std::path::Path;
use axum_login::{
    login_required,
    tower_sessions::{ExpiredDeletion, Expiry, SessionManagerLayer, cookie::Key},
    AuthManagerLayerBuilder,
};
use axum_messages::MessagesManagerLayer;
use tokio::{signal, task::AbortHandle};
use tower_sessions_sqlx_store::PostgresStore;

use crate::configuration::Settings;
use crate::configuration::DatabaseSettings;
use crate::configuration::EmailSettings;
use crate::routes::health_check_routes;
use crate::routes::homepage_routes;
use crate::routes::auth_routes;
use crate::routes::protected_routes;
use crate::user::Backend;
use crate::constants::strings;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub hmac_secret: Secret<String>,
    pub tera: Arc<Tera>,
    pub email_settings: EmailSettings,
}

pub struct Application {
    port: u16,
    db_pool: PgPool,
    tera: Arc<Tera>,
    listener: TcpListener,
    base_url: String,
    redis_uri: Secret<String>,
    hmac_secret: Secret<String>,
    email_settings: EmailSettings,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        // Compile SCSS files to CSS at runtime
        compile_scss_to_css("scss", "public/css");
        let connection_pool = get_connection_pool(&configuration.database);

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address).await?;
        let port = listener.local_addr().unwrap().port();
        let tera = Tera::new("templates/**/*html")?;
        let tera = Arc::new(tera);

        Ok(Self {
            port,
            tera,
            listener,
            db_pool: connection_pool,
            base_url: configuration.application.base_url,
            redis_uri: configuration.redis_uri,
            hmac_secret: configuration.application.hmac_secret,
            email_settings: configuration.email,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), anyhow::Error> {
        run(
            self.db_pool, self.listener, self.base_url, self.redis_uri, self.hmac_secret, self.tera, self.email_settings
            ).await
    }
}

pub fn get_connection_pool(
    configuration: &DatabaseSettings
) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy_with(configuration.with_db())
}

pub struct ApplicationBaseUrl(pub String);

pub async fn run(db_pool: PgPool, listener: TcpListener, _base_url: String, _redis_uri: Secret<String>, hmac_secret: Secret<String>, tera: Arc<Tera>, email_settings: EmailSettings) -> Result<(), anyhow::Error> {
    // Session layer.
    //
    // This uses `tower-sessions` to establish a layer that will provide the session
    // as a request extension.
    let session_store = PostgresStore::new(db_pool.clone());
    session_store.migrate().await?;
    let deletion_task = tokio::task::spawn(
        session_store
        .clone()
        .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
    );

    // Generate a cryptographic key to sign the session cookie.
    // let key = Key::generate();
    // TODO: Need to secure cookie
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(time::Duration::days(1)));

    // Auth service.
    //
    // This combines the session layer with our backend to establish the auth
    // service which will provide the auth session as a request extension.
    let backend = Backend::new(db_pool.clone());
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let app = api_router()
        .layer(TraceLayer::new_for_http())
        .layer(
            Extension(
                AppState {
                    db: db_pool,
                    hmac_secret,
                    tera,
                    email_settings,
                }
            )
        )
        .layer(MessagesManagerLayer)
        .layer(auth_layer);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
        .await?;

    deletion_task.await??;
    Ok(())
}

fn api_router() -> Router {
    // The ServeDir directory will allow the application to access these files and its
    // subdirectories
    let service = ServeDir::new("public")
        .fallback(ServeFile::new("public/file_not_found.html"));

    Router::new()
        .nest_service("/public", service)
        .merge(health_check_routes())
        .merge(homepage_routes())
        .merge(protected_routes())
        .merge(auth_routes())
}

fn compile_scss_to_css(scss_dir: &str, css_dir: &str) {
    // Create the CSS directory if it doesn't exist
    fs::create_dir_all(css_dir).unwrap();

    // Compile SCSS files to CSS
    for entry in fs::read_dir(scss_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("scss") {
            let css = grass::from_path(&path, &grass::Options::default()).expect(strings::FAILED_TO_COMPILE_SCSS);

            let css_path = Path::new(css_dir).join(path.with_extension("css").file_name().unwrap());
            fs::write(css_path, css).expect(strings::FAILED_TO_WRITE_SCSS);
        }
    }
}

async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { deletion_task_abort_handle.abort() },
        _ = terminate => { deletion_task_abort_handle.abort() },
    }
}

