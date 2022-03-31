//! The server library: provides the REST API and live game
//! protocol over WebSockets to manage state for the app. Data
//! is stored in an sqlite database.

// Produce a compiler warning for missing documentation.
#![warn(missing_docs)]

use error::Result;
use lettre::{
    message::Mailbox, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};
use scrabble::util::fsm::FastFsm;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::{convert::Infallible, net::SocketAddr};
use std::{env, sync::Arc};
use warp::{http::Method, Filter};

pub mod auth;
pub mod error;
pub mod models;
pub mod routes;

/// Alias type for the database pool.
type Db = PgPool;

/// Used to send emails asynchronously.
#[derive(Clone)]
pub struct Mailer {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    from_mailbox: Arc<Mailbox>,
}
impl Mailer {
    /// Sends an email message.
    pub async fn send(&self, to: &str, subject: &str, body: String) -> Result<()> {
        let from = (*self.from_mailbox).clone();
        let msg = Message::builder()
            .from(from)
            .to(to.parse()?)
            .subject(subject)
            .body(body)?;

        self.mailer.send(msg).await?;

        Ok(())
    }
}

/// Provides a request handler with access to the database connection pool.
pub fn with_db(db: &Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    let db = db.clone();
    warp::any().map(move || db.clone())
}

/// Provides a request handler with access to the smtp mailer.
pub fn with_mailer(
    mailer: &Mailer,
) -> impl Filter<Extract = (Mailer,), Error = Infallible> + Clone {
    let mailer = mailer.clone();
    warp::any().map(move || mailer.clone())
}

/// Starts the server on the given address.
pub async fn serve(addr: impl Into<SocketAddr>) -> Result<()> {
    // load TLS certificate and private key.
    let cert_path = env::var("CERT_PATH").expect("`CERT_PATH` env variable");
    let key_path = env::var("KEY_PATH").expect("`KEY_PATH` env variable");

    // connect to the email server
    let mailer = connect_email()?;

    // load the finite state machine and store in an atomic reference counter.
    let fsm = load_fast_fsm()?;
    let fsm = Arc::new(fsm);

    // connect to the database.
    let db = connect_db().await?;

    // CORS settings, which set allowed origins, headers and methods.
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[
            Method::GET,
            Method::OPTIONS,
            Method::POST,
            Method::DELETE,
            Method::PUT,
        ])
        .allow_headers(vec!["authorization", "content-type"]);

    // serve on `addr`.
    warp::serve(routes::all(&db, &mailer, fsm).with(cors))
        .tls()
        .cert_path(cert_path)
        .key_path(key_path)
        .run(addr)
        .await;

    Ok(())
}

/// Connects to the SMTP email server.
fn connect_email() -> Result<Mailer> {
    let smtp_server = env::var("EMAIL_SMTP_SERVER").expect("`EMAIL_SMTP_SERVER` env variable");
    let email_addr = env::var("EMAIL_ADDRESS").expect("`EMAIL_ADDRESS` env variable");
    let email_pwd = env::var("EMAIL_PASSWORD").expect("`EMAIL_PASSWORD` env variable");

    let from_mailbox = email_addr.parse::<Mailbox>()?;
    let credentials = Credentials::new(email_addr, email_pwd);
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_server)?
        .credentials(credentials)
        .build();

    Ok(Mailer {
        mailer,
        from_mailbox: Arc::new(from_mailbox),
    })
}

/// Loads the `FastFsm` from the binary file.
fn load_fast_fsm() -> Result<FastFsm> {
    let fsm_path = env::var("FAST_FSM_BIN").expect("`FAST_FSM_BIN` env variable");

    log::info!("loading fast fsm: {fsm_path}");
    let file = std::fs::File::open(&fsm_path)?;
    let rdr = std::io::BufReader::new(file);
    let fast_fsm: FastFsm = bincode::deserialize_from(rdr)?;

    Ok(fast_fsm)
}

/// Connects to the database at $DATABASE_URL.
async fn connect_db() -> Result<PgPool> {
    let db_url = env::var("DATABASE_URL").expect("`DATABASE_URL` env variable");

    log::info!("connecting to database: {db_url}");
    Ok(PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await?)
}
