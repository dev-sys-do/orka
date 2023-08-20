// use std::env;
// use std::io::Write;
// use std::process::{Command, Stdio};

// const CNI_PATH: String = env::var("CNI_PATH").unwrap();

// fn exec_ipam(input: &str, r#type: &str) -> String {
//     let exec_path = format!("{}/{}", CNI_PATH, r#type);

//     let mut ipam_proc = Command::new(&exec_path)
//         .stdin(Stdio::piped())
//         .stdout(Stdio::piped())
//         .spawn()
//         .unwrap();

//     ipam_proc
//         .stdin
//         .as_mut()
//         .unwrap()
//         .write_all(input.as_bytes())
//         .unwrap();

//     let output = ipam_proc.wait_with_output().unwrap();
//     String::from_utf8_lossy(&output.stdout).to_string()
// }
