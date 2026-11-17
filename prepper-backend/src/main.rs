use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files::NamedFile;
use anyhow::{Result, anyhow};
use chrono::{Days, Duration, Utc};
use ics::{Alarm, Event, ICalendar, properties::{Action, Description, DtEnd, DtStart, Name, Summary, Trigger}};

const FILE_PATH: &'static str = "./static/calendar.ics";
const DTSTAMP_FORMAT: &'static str = "%Y%m%dT%H%M%SZ";

#[get("/")]
async fn calendar() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open(FILE_PATH)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(calendar)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[test]
fn create_calendar() -> Result<()> {
    let now = Utc::now();
    let start = now.checked_add_days(Days::new(1)).expect("Should always succeed");
    let end = start + Duration::minutes(30);

    let mut event = Event::new(uuid::Uuid::new_v4().to_string(), now.format(DTSTAMP_FORMAT).to_string());
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