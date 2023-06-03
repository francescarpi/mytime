use oxhttp::model::{Method, Request, Status};
use oxhttp::Client;
use serde_json::json;

use crate::core::config::Config;
use crate::core::errors::Error;
use crate::core::task::Task;
use crate::integrations::traits::Integration;
use crate::core::utils::formatters::format_seconds;

#[derive(Debug)]
pub struct Redmine {}

impl Integration for Redmine {
    fn report_task<'a>(config: &'a Config, task: &Task) -> Result<(), Error> {
        if config.redmine_token.is_none() || config.redmine_url.is_none() {
            return Err(Error::MissingIntegrationParams);
        }

        let url = format!("{}time_entries.json", &config.redmine_url.clone().unwrap());
        let body = json!({
            "time_entry": {
                "issue_id": task.external_id,
                "hours": format_seconds(&task.duration()),
                "comments": task.desc,
            }
        });

        let client = Client::new();
        let response = client
            .request(Self::prepare_request(
                &url,
                &body.to_string(),
                &config.redmine_token.clone().unwrap(),
            ))
            .unwrap();

        if response.status() != Status::CREATED {
            return Err(Error::TaskCannotBeenReported);
        }

        Ok(())
    }
}

impl Redmine {
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
