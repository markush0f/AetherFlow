//Translating abstract Runtimes into concrete system commands.

use crate::Runtime;

impl Runtime {
    // Builds the executable and arguments based on the runtime type
    pub fn build_command(&self, entrypoint: &str) -> (String, Vec<String>) {
        match self {
            Runtime::Python3 => (
                "python3".to_string(),
                vec!["-u".to_string(), entrypoint.to_string()], // -u for unbuffered output
            ),
            Runtime::NodeJS => ("node".to_string(), vec![entrypoint.to_string()]),
            Runtime::Native => (entrypoint.to_string(), vec![]),
            // RemoteApi doesn't use system commands, it will be handled by a client
            Runtime::RemoteApi { .. } => ("network_call".to_string(), vec![]),
        }
    }
}
