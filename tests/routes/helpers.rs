use sqlx::{PgConnection, Executor, Connection};
use tradesalsa::configuration::{get_configuration, DatabaseSettings};
use tradesalsa::telemetry::{get_subscriber, init_subscriber};
use tradesalsa::startup::Application;
use sqlx::PgPool;
use once_cell::sync::Lazy;
use uuid::Uuid;
use fake::faker::internet::en::SafeEmail;
use fake::Fake;
use rand::Rng;
use rand::seq::SliceRandom;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub struct TestUser {
    pub user_id: Uuid,
    pub email: String,
    pub password: String,
}

impl TestUser {
    pub fn generate() -> Self {
        Self {
            user_id: Uuid::new_v4(),
            email: SafeEmail().fake::<String>(),
            password: Uuid::new_v4().to_string(),
        }
    }

    /// This function will store the built test user into the db pool passed in
    async fn store(&self, pool: &PgPool) {
        let email = self.email.clone();
        let password_hash = password_auth::generate_hash(self.password.clone());
        sqlx::query!(
            "INSERT INTO users (id, email, password_hash)
            VALUES ($1, $2, $3)",
            self.user_id,
            email,
            password_hash,
        )
        .execute(pool)
        .await
        .expect("Failed to store test user.");
    }

}

pub struct TestApp {
    pub address: String,
    pub _port: u16,
    pub db_pool: PgPool,
    pub api_client: reqwest::Client,
    pub test_user: TestUser,
    pub _db_settings: DatabaseSettings,
}

impl TestApp {
    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize
    {
        let url = format!("{}/login", &self.address);
        self.api_client
            .post(&url)
            .form(&body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_register<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize
    {
        self.api_client
            .post(&format!("{}/register", &self.address))
            .form(&body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_register(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/register", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_homepage_html(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to get homepage")
    }

    pub async fn get_health_check(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/health", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_login(&self, query_params: Option<&[(&str, &str)]>) -> reqwest::Response {
        let mut url = format!("{}/login", &self.address);

        if let Some(params) = query_params {
            let query_string = serde_urlencoded::to_string(params).expect("Failed to serialize query params");
            url = format!("{}?{}", url, query_string);
        }

        self.api_client
            .get(&url)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_protected(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/protected", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub async fn spawn_app() -> TestApp {
    /*
     * The first time 'initialize is invoked the code in 'TRACING' is executed.
     * All other invocations will instead skip execution (so init_subscriber() is only called once)
     */
    Lazy::force(&TRACING);
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration");
        // Use a different database for each test case
        c.database.database_name = Uuid::new_v4().to_string();
        // Use a random OS port
        c.application.port = 0;
        c
    };

    /* Session */
    let db_pool = configure_database(&configuration.database).await;

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application");

    let application_port = application.port();
    let address = format!("http://127.0.0.1:{}", application_port);

    let _ = tokio::spawn(application.run_until_stopped());
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();
    let mut test_app = TestApp {
        address,
        db_pool,
        _port: application_port,
        test_user: TestUser::generate(),
        api_client: client,
        _db_settings: configuration.database
    };
    test_app.test_user.store(&mut test_app.db_pool).await;
    test_app
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    /* Create database to use for testing */
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

pub fn assert_is_redirect_to(response: &reqwest::Response, location: &str) {
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), location);
}

pub fn fake_email() -> String {
    SafeEmail().fake::<String>()
}

pub fn rand_special_char() -> char {
    let mut rng = rand::thread_rng();
    let special_chars: Vec<char> = "!@#$%^&*()_+-=[]{}|;:,.<>/?".chars().collect();
    *special_chars.choose(&mut rng).unwrap()
}

pub fn rand_digit() -> char {
    let mut rng = rand::thread_rng();
    rng.gen_range(b'0'..=b'9' + 1) as char
}

pub fn rand_lowercase() -> char {
    let mut rng = rand::thread_rng();
    rng.gen_range(b'a'..=b'z' + 1) as char
}

pub fn rand_uppercase() -> char {
    let mut rng = rand::thread_rng();
    rng.gen_range(b'A'..=b'Z' + 1) as char
}

