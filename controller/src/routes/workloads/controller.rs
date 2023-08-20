use axum::response::Html;
use serde_json::{self, Value};
use crate::client::{Client, scheduler::{SchedulingRequest, Workload, workload::{Type, Resources}}};

pub async fn get_workloads(body: String) -> Html<String> {
    tokio::spawn(async move {
        // TODO: Implement => retrieve data from db
    });
    Html(format!("Hello, {}!", {"body"}))
}


pub async fn post_workload(body: String) -> Html<String> {
    tokio::spawn(async move {
        let mut client = Client::new().await.unwrap();
        let json_body: Value = serde_json::from_str(&body).unwrap();

        let mut environment = Vec::new();
        if let Some(environment_values) = json_body.get("environment").and_then(|e| e.as_array()) {
            for value in environment_values {
                if let Some(env_str) = value.as_str() {
                    environment.push(env_str.to_string());
                }
            }
        }
        
        let workload = Workload {
            name: json_body["name"].as_str().unwrap_or("DefaultName").to_string(),
            r#type: Type::Container.into(),
            image: json_body["image"].as_str().unwrap_or("DefaultImage").to_string(),
            environment: environment,
            resource_limits: Some(Resources{
                cpu: Some(json_body["cpu"].as_i64().unwrap_or(1) as i32),
                memory: Some(json_body["memory"].as_i64().unwrap_or(1) as i32),
                disk: Some(json_body["disk"].as_i64().unwrap_or(1) as i32),
            }),
        };

        print!("Here is the workload {:?}", workload);
        
        let request = SchedulingRequest {
            workload: Some(workload),
        };

        client.schedule_workload(request).await.unwrap();
        // TODO check response , if OK save data and send response to cli
    });
    Html(format!("{}!", {"body"}))
}
