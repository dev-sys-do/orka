use std::path::Path;

use crate::types;
use crate::allocator;


pub fn exec(command: &types::CNICommand) -> Result<(), ()> {
    let init_result = allocator::init_datadir(&command.data_dir.clone(), &command.cni_version.clone());
    if let Err(()) = init_result {
        return Err(());
    }

    let find_result = allocator::find_container_id(&command.data_dir.clone(), &command.container_id.clone(), &command.cni_version.clone());
    if find_result.is_none() {
        return Err(());
    }

    let data_dir = command.data_dir.clone();
    let ip_addr = find_result.unwrap();
    let file_path_str = data_dir + "/" + &ip_addr;
    let file_path = Path::new(&file_path_str);
    
    let remove_result = allocator::remove_file(file_path, &command.cni_version.clone());
    if let Err(()) = remove_result {
        return Err(());
    }

    Ok(())
}

