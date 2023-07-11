use slack_morphism::prelude::*;
use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SlackEvent {
    pub token: String,
    #[serde(rename = "team_id")]
    pub team_id: String,
    #[serde(rename = "api_app_id")]
    pub api_app_id: String,
    pub event: SlackInnerEvent,
    #[serde(rename = "type")]
    pub type_field: String,
    pub authed_users: Vec<String>,
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(rename = "event_time")]
    pub event_time: u64,
}

//https://api.slack.com/events
#[derive(Debug, Deserialize, Serialize)]
pub struct SlackInnerEvent {
    #[serde(rename = "type")]
    pub type_field: String,
    pub user: String,
    pub text: String,
    pub ts: String,
    pub channel: String,
    #[serde(rename = "event_ts")]
    pub event_ts: String,
    #[serde(rename = "channel_type")]
    pub channel_type: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(slack_events))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn slack_events() -> impl Responder{
    ""
}

