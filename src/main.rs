use actix_web::{post, web,web::Json,App, HttpServer, Responder, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::Read;
use std::fs::File;
use reqwest;
use serde_json;

#[derive(Debug, Deserialize, Serialize)]
pub struct SlackEventCallback {
    pub token: String,
    #[serde(rename = "team_id")]
    pub team_id: String,
    #[serde(rename = "api_app_id")]
    pub api_app_id: String,
    pub event: SlackEvent,
    #[serde(rename = "type")]
    pub type_field: String,
    pub authed_users: Vec<String>,
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(rename = "event_time")]
    pub event_time: u64,
    pub event_ts: Option<String>,
    pub channel_type: Option<String>
}

//https://api.slack.com/events
#[derive(Debug, Deserialize, Serialize)]
pub struct SlackEvent {
    #[serde(rename = "type")]
    pub type_field: String,
    pub user: String,
    pub text: String,
    pub ts: String,
    pub channel: String,
    pub subtype: Option<String>,
    pub event_ts: String,
    pub channel_type: String,
    pub files: Option<String>
}

#[derive(Debug, Deserialize)]
struct SlackChallenge {
    token: String,
    challenge: String,
    #[serde(rename = "type")]
    type_field: String,
}

#[derive(Deserialize, Debug)]
struct SlackPayload {
    event: SlackEventCallback,
    channel: String,
}

#[derive(Serialize, Debug)]
struct SlackResponse {
    text: String,
}

#[derive(Debug)]
enum MyError {
    DeserializeError(serde_json::Error),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::DeserializeError(ref err) => write!(f, "Deserialization Error: {}", err),
        }
    }
}

impl ResponseError for MyError {}

impl From<serde_json::Error> for MyError {
    fn from(err: serde_json::Error) -> MyError {
        MyError::DeserializeError(err)
    }
}

async fn upload_file(channel: String, file_path: &str, bot_token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    let mut file = File::open(file_path)?;
    file.read_to_end(&mut buffer)?;

    let part = reqwest::multipart::Part::bytes(buffer);
    let form = reqwest::multipart::Form::new().part("file", part);

    let client = reqwest::Client::new();
    let res = client.post("https://slack.com/api/files.upload")
        .bearer_auth(bot_token)
        .multipart(form)
        .send().await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                Ok("File sent.".to_string())
            } else {
                Ok(format!("File upload failed: {:?}", response.text().await?))
            }
        }
        Err(e) => {
            Err(e.into())
        }
    }
}

async fn slack_challenge_handler(challenge: Json<SlackChallenge>) -> Result<impl Responder, MyError> {
    Ok(HttpResponse::Ok().body(challenge.challenge.clone()))
}

async fn slack_event_handler(payload: Json<SlackEventCallback>,file_path: &str, bot_token: &str) -> impl Responder {
    //Payload内のeventオブジェクト内のtype,subtype,filesで分岐
    //Payload内のeventオブジェクト内のtype,subtype,filesで分岐
    match (payload.event.type_field.as_str(), payload.event.subtype.as_deref(), payload.event.files.as_ref()) {
        //eventがmessage,subtypeがapp_mention,filesがある場合
        ("message", Some("app_mention"), Some(files)) if !files.is_empty() => {
            let response_text = match upload_file(payload.event.channel.clone(), file_path, bot_token).await {
                Ok(msg) => msg,
                Err(e) => format!("Error: {:?}", e),
            };

            HttpResponse::Ok().json(SlackResponse {
                text: response_text,
            })
        }
        ("message", Some("app_mention"), _) => {
            HttpResponse::Ok().json(SlackResponse {
                text: "OK".to_string(),
            })
        }
        _ => {
            HttpResponse::Ok().json(SlackResponse {
                text: "NG".to_string(),
            })
        }
    }
}

#[post("/slack/events")]
async fn slack_events(req_body: String) -> Result<impl Responder, MyError> {
    // try to deserialize as a challenge
    match serde_json::from_str::<SlackChallenge>(&req_body) {
        Ok(challenge) => {
            slack_challenge_handler(Json(challenge)).await
        }
        Err(_) => {
            // if not a challenge, try as an event
            match serde_json::from_str::<SlackEventCallback>(&req_body) {
                Ok(event) => {
                    slack_event_handler(Json(event), "path_to_your_file", "your_bot_token").await
                }
                Err(_) => {
                    Err(MyError::DeserializeError(serde_json::Error::from_str("Bad request")))
                }
            }
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::{self, header::ContentType},
        test,
    };

    #[actix_web::test]
    async fn test_index_ok() {
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_http_request();
        let resp = index(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_index_not_ok() {
        let req = test::TestRequest::default().to_http_request();
        let resp = index(req).await;
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }
}