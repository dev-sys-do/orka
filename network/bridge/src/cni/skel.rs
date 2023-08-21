// skel provides skeleton code for a CNI plugin.
// In particular, it implements argument parsing and validation.
use super::types::{Code, Error};
use std::{
    collections::HashMap,
    env::{self, VarError},
    io::{self, Read, Write},
};

pub struct CmdArgs {
    container_id: String,   // ContainerID
    netns: String,          // Netns
    if_name: String,        // IfName
    args: String,           // Args
    path: String,           // Path
    netns_override: String, // NetnsOverride
    stdin_data: Vec<u8>,    // StdinData []byte
}

struct Dispatcher<I: Read, O: Write, E: Write> {
    getenv: fn(String) -> Result<String, VarError>,
    stdin: I,
    stdout: O,
    stderr: E,
}

impl<I: Read, O: Write, E: Write> Dispatcher<I, O, E> {
    fn get_cmd_args_from_env(&mut self) -> Result<(String, CmdArgs), Error> {
        let mut cmd: String = String::new();
        let mut container_id: String = String::new();
        let mut netns: String = String::new();
        let mut if_name: String = String::new();
        let mut args: String = String::new();
        let mut path: String = String::new();
        let mut netns_override: String = String::new();

        let req_for_cmd: HashMap<String, bool> = [
            ("ADD".to_string(), true),
            ("CHECK".to_string(), true),
            ("DEL".to_string(), true),
        ]
        .iter()
        .cloned()
        .collect();

        let cmd_before_loop = cmd.clone();

        let vars: [(&str, &mut String, &HashMap<String, bool>); 7] = [
            ("CNI_COMMAND", &mut cmd, &req_for_cmd),
            ("CNI_CONTAINERID", &mut container_id, &req_for_cmd),
            ("CNI_NETNS", &mut netns, &req_for_cmd),
            ("CNI_IFNAME", &mut if_name, &req_for_cmd),
            ("CNI_ARGS", &mut args, &HashMap::new()),
            ("CNI_PATH", &mut path, &req_for_cmd),
            ("CNI_NETNS_OVERRIDE", &mut netns_override, &HashMap::new()),
        ];

        let mut args_missing = Vec::new();
        for (name, val, req_for_cmd_entry) in vars.into_iter() {
            *val = (self.getenv)(name.to_string()).unwrap();
            if (*val).is_empty() {
                if *req_for_cmd_entry.get(&cmd_before_loop).unwrap_or(&false)
                    || name == "CNI_COMMAND"
                {
                    args_missing.push(name);
                }
            }
        }

        if !args_missing.is_empty() {
            let joined: String = args_missing.join(",");
            return Err(Error::new(
                Code::ErrInvalidEnvironmentVariables,
                format!("required env variables {} missing", joined),
                "".to_string(),
            ))?;
        }

        // Handle VERSION command
        // if cmd == "VERSION" {
        //     // Update stdin to be an empty reader
        //     self.stdin = std::io::empty();
        // }

        let mut stdin_data = Vec::new();
        self.stdin.read_to_end(&mut stdin_data).unwrap();

        let cmd_args: CmdArgs = CmdArgs {
            container_id,
            netns,
            if_name,
            args,
            path,
            stdin_data,
            netns_override,
        };

        Ok((cmd, cmd_args))
    }

    fn plugin_main(
        &mut self,
        cmd_add: fn(&CmdArgs),
        cmd_check: fn(&CmdArgs),
        cmd_del: fn(&CmdArgs),
        // version_info: PluginInfo,
        about: &str,
    ) {
        let (cmd, cmd_args) = self.get_cmd_args_from_env().unwrap();

        match cmd.as_str() {
            "ADD" => {
                cmd_add(&cmd_args);
            }
            "CHECK" => {
                cmd_check(&cmd_args);
            }
            "DEL" => {
                cmd_del(&cmd_args);
            }
            "VERSION" => (),
            _ => {
                panic!("unknown CNI_COMMAND: {}", cmd);
            }
        }
    }
}

fn run_plugin_main(
    cmd_add: fn(&CmdArgs),
    cmd_check: fn(&CmdArgs),
    cmd_del: fn(&CmdArgs),
    about: &str,
) {
    let mut dispatcher: Dispatcher<io::Stdin, io::Stdout, io::Stderr> = Dispatcher {
        getenv: env::var::<String>,
        stdin: io::stdin(),
        stdout: io::stdout(),
        stderr: io::stderr(),
    };

    dispatcher.plugin_main(cmd_add, cmd_check, cmd_del, about);
}
