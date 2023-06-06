use oxhttp::model::{Method, Request, Status};
use oxhttp::Client;
use serde::{Deserialize, Serialize};

use crate::core::config::Config;
use crate::core::errors::Error;
use crate::core::task::Task;
use crate::core::utils::formatters::{format_seconds, string_to_date};
use crate::integrations::traits::Integration;

#[derive(Serialize, Deserialize, Debug)]
struct RedmineResponse {
    errors: Vec<String>,
}

#[derive(Debug)]
pub struct Redmine {}

impl Integration for Redmine {
    fn report_task<'a>(&self, config: &'a Config, task: &Task) -> Result<(), Error> {
        if config.redmine_token.is_none() || config.redmine_url.is_none() {
            return Err(Error::MissingIntegrationParams);
        }

        let url = format!("{}time_entries.json", &config.redmine_url.as_ref().unwrap());
        let body = serde_json::json!({
            "time_entry": {
                "issue_id": task.external_id,
                "hours": format_seconds(&task.duration()),
                "comments": task.desc,
                "spent_on": string_to_date(&task.start),
            }
        });

        let client = Client::new();
        let response = client
            .request(Self::prepare_request(
                &url,
                &body.to_string(),
                &config.redmine_token.as_ref().unwrap(),
            ))
            .unwrap();

        if response.status() != Status::CREATED {
            let redmine_response: RedmineResponse =
                serde_json::from_str(&response.into_body().to_string().unwrap()).unwrap();
            return Err(Error::TaskCannotBeenReported(String::from(
                redmine_response.errors.join(", "),
            )));
        }

        Ok(())
    }
}

impl Redmine {
    pub fn new() -> Self {
        Self {}
    }

    fn prepare_request(url: &str, body: &str, token: &str) -> Request {
        let mut request =
            Request::builder(Method::POST, url.parse().unwrap()).with_body(body.to_string());
        request
            .append_header("Content-Type", "application/json")
            .unwrap();
        request.append_header("X-Redmine-API-Key", token).unwrap();
        request
    }
}
