use crate::cni_error;
use crate::types;
use crate::allocator;

pub fn exec(command: &types::CNICommand) -> Result<(), ()> {

    let init_result = allocator::init_datadir(&command.data_dir.clone(), &command.cni_version.clone());
    if let Err(()) = init_result {
        return Err(());
    }

    let find_result = allocator::find_container_id(
        &command.data_dir.clone(),
        &command.container_id.clone(),
        &command.cni_version.clone()
    );

    if find_result.is_none() {
        return Err(());
    }

    let container_ip = find_result.unwrap();
    if container_ip.is_empty() {
        cni_error::output_error(
            &"This container does not have registered ip address".to_string(),
            &"".to_string(),
            cni_error::CNIErrorCode::ContainerUnknownOrDoesntExist,
            &command.cni_version.clone()
        );
        return Err(());
    }

    Ok(())
}