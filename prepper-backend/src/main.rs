use std::env;
use actix_files::NamedFile;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use anyhow::{Context, Result, anyhow};
use chrono::{Days, Duration, Utc};
use diesel::{PgConnection, r2d2::{ConnectionManager, Pool}};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use dotenvy::dotenv;
use ics::{
    Alarm, Event, ICalendar,
    properties::{Action, Description, DtEnd, DtStart, Name, Summary, Trigger},
};
use crate::{api::{health::health, products::new_product_page}, server::ServerState};

const FILE_PATH: &'static str = "./static/calendar.ics";
const DTSTAMP_FORMAT: &'static str = "%Y%m%dT%H%M%SZ";
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

pub mod api;

pub mod server;

#[get("/")]
async fn calendar() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open(FILE_PATH)?)
}

fn required_env(name: &str) -> Result<String> {
    env::var(name).with_context(|| format!("Environment variable {name} has to be set"))
}


#[actix_web::main]
async fn main() -> Result<()> {
    if let Err(_err) = dotenv() {
        #[cfg(debug_assertions)]
        panic!("{_err:?}");
    }

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let db_url = required_env("DATABASE_URL")?;
    let secret = required_env("JWT_SECRET")?;

    let pool = Pool::builder()
        .max_size(8)
        .connection_timeout(std::time::Duration::from_secs(5))
        .build(ConnectionManager::<PgConnection>::new(&db_url))
        .context("Failed to connect to the database")?;

    let mut conn = pool.get()?;
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|err| anyhow!("migrations failed: {err}"))?;


    let state = web::Data::new(ServerState { pool, secret });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(calendar)
            .service(new_product_page)
            .service(health)
    })
    .bind(("0.0.0.0", 8888))?
    .run()
    .await?;

    Ok(())
}

#[test]
fn create_calendar() -> Result<()> {
    let now = Utc::now();
    let start = now
        .checked_add_days(Days::new(1))
        .expect("Should always succeed");
    let end = start + Duration::minutes(30);

    let mut event = Event::new(
        uuid::Uuid::new_v4().to_string(),
        now.format(DTSTAMP_FORMAT).to_string(),
    );
    event.push(Summary::new("Testing calendar!! Summary"));
    event.push(Description::new("Super test! Description"));
    event.push(DtStart::new(start.format(DTSTAMP_FORMAT).to_string()));
    event.push(DtEnd::new(end.format(DTSTAMP_FORMAT).to_string()));

    let mut alarm = Alarm::new(Action::display(), Trigger::new("-PT15M"));
    alarm.push(Description::new("Påminnelse"));
    event.add_alarm(alarm);

    let mut cld = ICalendar::new("2.0", "-//prepper-app//EN");
    cld.add_event(event);
    cld.push(Name::new("Test calendar name"));

    cld.save_file(FILE_PATH)?;
    Ok(())
}
