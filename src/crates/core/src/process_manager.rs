/* ProcessManager logic for managing the lifecycle and communication
   of external agents.
*/

use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;
use tokio::sync::Mutex;

use crate::types::{LiveProcess, Runtime};

pub struct ProcessManager {
    pub registry: Arc<Mutex<HashMap<String, LiveProcess>>>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /* Original spawn logic, now correctly scoped */
    pub async fn spawn_agent(
        &self,
        id: String,
        runtime: Runtime,
        entrypoint: String,
        workdir: String,
    ) -> Result<(), String> {
        let (program, args) = runtime.build_command(&entrypoint);

        // Run the agent as a child process
        let mut child = Command::new(program)
            .args(args)
            .current_dir(workdir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn {}: {}", id, e))?;

        let stdin = child.stdin.take().ok_or("Failed to capture stdin")?;
        let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;

        // Lock the registry to insert the live process
        let mut registry = self.registry.lock().await;

        // Insert the live process into the registry
        registry.insert(
            id,
            LiveProcess {
                handle: child,
                stdin,
                stdout,
            },
        );

        Ok(())
    }

    // The Communication Bridge:
    // Sends data to an agent's stdin and waits for its stdout response.
    pub async fn send_to_agent(&self, id: &str, message: &str) -> Result<String, String> {
        let mut registry = self.registry.lock().await;

        // 1. Find the agent in our memory registry
        let agent = registry
            .get_mut(id)
            .ok_or_else(|| format!("Agent {} not found or not running", id))?;

        // 2. Write the message to the agent's 'ear' (stdin)
        // We add a newline because most scripts expect it to finish a 'read'
        let formatted_message = format!("{}\n", message);
        agent
            .stdin
            .write_all(formatted_message.as_bytes())
            .await
            .map_err(|e| format!("Failed to write to agent {}: {}", id, e))?;

        // 3. Read the response from the agent's 'mouth' (stdout)
        // Note: This is a simplified read. In a real scenario, we need a
        // delimiter to know when the agent finishes speaking.
        let mut buffer = [0; 1024];
        let n = agent
            .stdout
            .read(&mut buffer)
            .await
            .map_err(|e| format!("Failed to read from agent {}: {}", id, e))?;

        Ok(String::from_utf8_lossy(&buffer[..n]).trim().to_string())
    }
}
