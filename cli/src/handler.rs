use crate::workloads::file::read_file;
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::process::exit;

use crate::config::Config;
use crate::{
    args::{
        config::{ConfigResource::ApiFqdn, ConfigResource::ApiPort, GetConfig, SetConfig},
        crud::{
            CreateInstance, CreateWorkload, DeleteInstance, DeleteWorkload, GetInstance,
            GetWorkload,
        },
    },
    DISPLAY,
};

pub struct Handler {
    client: reqwest::Client,
}

impl Handler {
    pub fn new() -> Self {
        Handler {
            client: reqwest::Client::new(),
        }
    }

    pub fn get_config_value(&self, args: GetConfig) {
        let config = Config::get_config_lock();
        let value: String = match args.resource {
            ApiFqdn => config.orka_url.clone(),
            ApiPort => config.orka_port.clone().to_string(),
        };
        DISPLAY.print_log(&value);
    }

    pub fn set_config_value(&self, args: SetConfig) {
        let mut config = Config::get_config_lock();
        match args.resource {
            ApiFqdn => config.set_orka_url(&args.value),
            ApiPort => {
                let port: u16 = match args.value.parse::<u16>() {
                    Ok(port) => port,
                    Err(e) => {
                        DISPLAY.print_error(&format!("Error parsing port: {}", e));
                        exit(-1)
                    }
                };

                config.set_orka_port(port);
            }
        };
        config.save()
    }

    pub async fn create_workload(&self, args: CreateWorkload) {
        match read_file(args.file_path) {
            Ok(json) => {
                let res = self
                    .client
                    .post(Handler::get_url("workloads"))
                    .json(&json)
                    .send()
                    .await;

                let _ = self
                    .generic_response_handling::<serde_json::Value>(res)
                    .await;
            }
            Err(error) => println!("{:?}", error),
        }
    }

    pub async fn create_instance(&self, args: CreateInstance) {
        let instance = serde_json::to_string(&args).unwrap();
        match serde_json::from_str::<serde_json::Value>(&instance) {
            Ok(json) => {
                let res = self
                    .client
                    .post(Handler::get_url("instances"))
                    .json(&json)
                    .send()
                    .await;

                let _ = self
                    .generic_response_handling::<serde_json::Value>(res)
                    .await;
            }
            Err(_) => DISPLAY.print_error("Not a json object."),
        };
    }

    pub async fn get_workload(&self, args: GetWorkload) {
        let mut url = Handler::get_url("workloads");
        if args.workload_id.is_some() {
            url += &format!("/{}", &args.workload_id.unwrap());
        }
        let res = self.client.get(url).send().await;

        let _ = self
            .generic_response_handling::<serde_json::Value>(res)
            .await;
    }

    pub async fn get_instance(&self, args: GetInstance) {
        let mut url = Handler::get_url("instances");
        if args.instance_id.is_some() {
            url += &format!("/{}", &args.instance_id.unwrap());
        }
        let res = self.client.get(url).send().await;

        let _ = self
            .generic_response_handling::<serde_json::Value>(res)
            .await;
    }

    pub async fn delete_workload(&self, args: DeleteWorkload) {
        let url = format!("{}/{}", Handler::get_url("workloads"), args.workload_id);
        let res = self.client.delete(url).send().await;

        let _ = self
            .generic_response_handling::<serde_json::Value>(res)
            .await;
    }

    pub async fn delete_instance(&self, args: DeleteInstance) {
        let mut url = format!("{}/{}", Handler::get_url("instances"), args.instance_id);
        if args.force {
            url += "/force"
        }
        let res = self.client.delete(url).send().await;

        let _ = self
            .generic_response_handling::<serde_json::Value>(res)
            .await;
    }

    /// Wrapper to display common errors
    async fn generic_response_handling<T: DeserializeOwned + Serialize>(
        &self,
        response: Result<Response, reqwest::Error>,
    ) -> Option<T> {
        match response {
            Err(err) => DISPLAY.print_error(&format!("{:?}", err)),
            Ok(response) => {
                let response_status = response.status();
                let response_text = response.text().await.unwrap();

                if !response_status.is_success() {
                    DISPLAY.print_error(&format!(
                        "The server returned with error {}",
                        response_status
                    ));

                    let json_err: Result<serde_json::Value, serde_json::Error> =
                        serde_json::from_str(&response_text);

                    if let Ok(value) = json_err {
                        DISPLAY.print_error(&format!(
                            "Status: {} \n Message: {}",
                            value["status"], value["message"]
                        ));
                        return None;
                    }
                }

                let json: Result<T, serde_json::Error> = serde_json::from_str(&response_text);
                match json {
                    Err(_) => DISPLAY.print_error("The response is not a formatted json !"),
                    Ok(json) => {
                        DISPLAY
                            .print_log(&serde_json::to_string_pretty(&json).unwrap_or("".into()));
                        return Some(json);
                    }
                }
            }
        };

        None
    }

    fn get_url(endpoint: &str) -> String {
        let config = Config::get_config_lock();
        let url_with_port = Config::get_url_and_port(&config.orka_url, config.orka_port);
        format!("{}{}", url_with_port, endpoint)
    }
}
