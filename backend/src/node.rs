use std::{fs::File, os::unix::process::CommandExt, process::Child};

use serde::Serialize;

use crate::os::{OperatingSystem, ProcessInformation};

#[derive(Debug, Clone, Serialize)]
pub struct Parameters {
    dev: bool,
    chain: Option<String>,
    alice: bool,
    tmp: bool,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            dev: true,
            chain: None,
            alice: false,
            tmp: false,
        }
    }
}

impl Parameters {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(&self) -> Vec<String> {
        return vec![String::from("--dev")];
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Node {
    binary_name: String,
    binary_path: String,
    storage_path: String,
    proc: Option<ProcessInformation>,
    args: Parameters,
}

impl Node {
    pub fn new() -> Self {
        Self {
            binary_name: String::from("seed"),
            binary_path: String::from("/home/marko/Projects/centrality/seed/target/debug/seed"),
            ..Default::default()
        }
    }
    pub fn set_binary_name(&mut self, val: String) {
        self.binary_name = val;
    }

    pub fn set_binary_path(&mut self, val: String) {
        self.binary_path = val;
    }

    pub fn set_storage_path(&mut self, val: String) {
        self.storage_path = val;
    }

    pub fn refresh(&mut self, os: &OperatingSystem) {
        self.proc = os.process_information(&self.binary_name);
    }

    pub fn pid(&self) -> Option<usize> {
        self.proc.as_ref().and_then(|proc| Some(proc.pid))
    }

    pub fn path(&self) -> String {
        self.binary_path.clone()
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn args(&self) -> Vec<String> {
        self.args.build()
    }
}

#[derive(Debug, Default)]
pub struct NodeManager(Option<Child>);

impl NodeManager {
    pub fn stop_node(&mut self, node: &Node) -> Result<String, String> {
        let Some(pid) = node.pid() else {
            return Err(String::from("Cannot stop node. Node is not running."));
        };
        let Some(mut child) = self.0.take() else {
            return Err(String::from("Cannot stop node. Node is not running. 2"));
        };

        dbg!(child.kill());
        dbg!(child.wait());

        Ok(String::from("Node is killed"))
    }

    pub fn start_node(&mut self, node: &Node) -> Result<String, String> {
        if let Some(_) = node.pid() {
            return Err(String::from("Node is already running"));
        }

        let command = std::format!("{}", node.path());
        let args = node.args();
        let stdout_log =
            File::create("/home/marko/Projects/centrality/seed/stdout_log.txt").unwrap();
        let stderr_log =
            File::create("/home/marko/Projects/centrality/seed/stderr_log.txt").unwrap();

        let mut cmd = std::process::Command::new(command);
        let cmd = cmd.args(args);
        let cmd = cmd.stdout(stdout_log);
        let cmd = cmd.stderr(stderr_log);

        let res = cmd.spawn();
        match res {
            Ok(child) => {
                self.0 = Some(child);
                Ok(String::from("New process is created"))
            }
            Err(err) => Err(err.to_string()),
        }
    }
}
