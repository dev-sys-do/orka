use crate::args::{
    config::{GetConfig, SetConfig},
    crud::{
        CreateInstance, CreateWorkload, DeleteInstance, DeleteWorkload, GetInstance, GetWorkload,
    },
    OrkaCtlArgs,
};

pub fn get_config_value(args: GetConfig) {}

pub fn set_config_value(args: SetConfig) {}

pub async fn create_workload(args: CreateWorkload) {}

pub async fn create_instance(args: CreateInstance) {}

pub async fn get_workload(args: GetWorkload) {}

pub async fn get_instance(args: GetInstance) {}

pub async fn delete_workload(args: DeleteWorkload) {}

pub async fn delete_instance(args: DeleteInstance) {}
