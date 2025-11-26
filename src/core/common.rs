use std::{process::{Command, Stdio}, str::FromStr};
use actix_web::{HttpRequest, HttpResponse, Responder, post, web::{self, Bytes}};
use log::info;
use serde::{Deserialize, Serialize};
use crate::{cli::Provider, core::{github::Github, gitlab::Gitlab}};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    Package,
    Ping,
    Push,
    Tag,
}

impl FromStr for EventType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "package" => Ok(EventType::Package),
            "push" => Ok(EventType::Push),
            "ping" => Ok(EventType::Ping),
            "tag" => Ok(EventType::Tag),
            _ => Err("Invalid event type"),
        }
    }
}

pub struct Headers {
    pub event_type: EventType,
    pub signature: String,
}

pub trait GitProvider {
    fn webhook(&self, http_request: HttpRequest, req_body: &Bytes) -> HttpResponse;
}

pub async fn calling_script_shell(provider: String, event_type: EventType, package_name: Option<String>) {
    match event_type {
        EventType::Package | EventType::Tag => {
            let program_name = format!("./{}-package.sh", provider);
            let child = Command::new(program_name)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .arg(serde_json::to_string(&event_type).unwrap())
                .arg(package_name.unwrap_or_default())
                .spawn()
                .expect("Failed to execute package.sh");

            let output = child.wait_with_output().expect("Failed to read stdout");
            info!("Package event received");
            if !output.stdout.is_empty() {
                info!("{}", String::from_utf8_lossy(&output.stdout));
            }
        },
        EventType::Ping => {
            info!("Ping event received");
        },
        _ => {
            info!("Nothing to do unknown event");
        }
    }
}

#[post("/webhook")]
pub async fn webhook_request(data: web::Data<Provider>, http_request: HttpRequest, req_body: Bytes) -> impl Responder {
    let proviver_enabled = data.as_ref();
    let github = Github;
    let gitlab = Gitlab;
    match proviver_enabled {
        Provider::Github => github.webhook(http_request, &req_body),
        Provider::Gitlab => gitlab.webhook(http_request, &req_body),
        Provider::Both => {
            let user_agent = http_request.headers().get("User-Agent");
            match user_agent {
                Some(ua) => {
                    let str_ua = ua.to_str().unwrap();
                    if str_ua.contains("GitLab") {
                        return gitlab.webhook(http_request, &req_body);
                    } else if str_ua.contains("GitHub") {
                        return github.webhook(http_request, &req_body);
                    } else {
                        return HttpResponse::BadRequest().body("Unknown User-Agent header");
                    }
                },
                None => HttpResponse::BadRequest().body("Missing User-Agent header"),
            }
        },
    }
}
