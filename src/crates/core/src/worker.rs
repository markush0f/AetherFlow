/* 
   This module implements the Worker Actor.
   It manages the physical OS process, handles communication via pipes,
   and implements the 'Scale-to-Zero' logic by self-terminating after a period of inactivity.
*/

use std::process::Stdio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;
use tokio::sync::{mpsc, oneshot};
use tokio::time::{timeout, Duration};

use crate::messages::{DirectorCommand, WorkerCommand};
use crate::types::Runtime;

const IDLE_TIMEOUT_SECONDS: u64 = 60;
const DELIMITER: &str = "__AETHER_DONE__";

pub struct Worker;

impl Worker {
    /* Spawns a new Worker Actor in a detached background task.
       Returns the Transmitter (tx) so the Director can send messages to this new Worker.
    */
    pub async fn spawn(
        id: String,
        runtime: Runtime,
        entrypoint: String,
        workdir: String,
        director_tx: mpsc::Sender<DirectorCommand>,
    ) -> Result<mpsc::Sender<WorkerCommand>, String> {
        let (program, args) = runtime.build_command(&entrypoint);

        // Spawn the physical OS process
        let mut child = Command::new(program)
            .args(args)
            .current_dir(workdir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn {}: {}", id, e))?;

        let mut stdin = child.stdin.take().ok_or("Failed to capture stdin")?;
        let mut stdout = child.stdout.take().ok_or("Failed to capture stdout")?;

        // Create the specific Walkie-Talkie for this Worker
        let (worker_tx, mut worker_rx) = mpsc::channel::<WorkerCommand>(32);

        // Launch the isolated Actor loop in a background thread
        tokio::spawn(async move {
            loop {
                /* We use tokio::time::timeout to wait for a new message.
                   If 60 seconds pass without receiving anything, this returns an Err,
                   triggering our 'Scale-to-Zero' self-destruction.
                */
                let message_result =
                    timeout(Duration::from_secs(IDLE_TIMEOUT_SECONDS), worker_rx.recv()).await;

                match message_result {
                    Ok(Some(WorkerCommand::Execute {
                        input,
                        reply_channel,
                    })) => {
                        // Write data to the physical agent
                        let payload = format!("{}\n", input);
                        if let Err(_) = stdin.write_all(payload.as_bytes()).await {
                            let _ = reply_channel.send(Err("Failed to write to agent".to_string()));
                            continue;
                        }

                        // Read the response using a delimiter to prevent blocking forever
                        let mut response_accumulator = String::new();
                        let mut buffer = [0; 1024];
                        let mut successful_read = false;

                        while let Ok(bytes_read) = stdout.read(&mut buffer).await {
                            if bytes_read == 0 {
                                break;
                            }

                            let chunk = String::from_utf8_lossy(&buffer[..bytes_read]);
                            response_accumulator.push_str(&chunk);

                            if response_accumulator.contains(DELIMITER) {
                                successful_read = true;
                                break;
                            }
                        }

                        // Send the response back to the Director's oneshot channel
                        if successful_read {
                            let clean_response = response_accumulator.replace(DELIMITER, "");
                            let _ = reply_channel.send(Ok(clean_response.trim().to_string()));
                        } else {
                            let _ = reply_channel
                                .send(Err("Agent closed without sending delimiter".to_string()));
                            break;
                        }
                    }
                    Ok(Some(WorkerCommand::Shutdown)) => {
                        // Explicit kill requested by the Director
                        break;
                    }
                    Ok(None) => {
                        // The Director dropped the transmission channel
                        break;
                    }
                    Err(_) => {
                        // Timeout reached! No messages received in 60 seconds.
                        // We break the loop to initiate self-destruction.
                        break;
                    }
                }
            }

            /* 4. Cleanup Phase: The loop has ended (due to timeout, error, or shutdown request) */
            let _ = child.kill().await;

            // Notify the Director to remove us from the routing table
            let _ = director_tx
                .send(DirectorCommand::WorkerTerminated { id })
                .await;
        });

        // 5. Return the transmission channel to the Director
        Ok(worker_tx)
    }
}
