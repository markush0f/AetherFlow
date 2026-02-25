/*
   The Director acts as the main router. It holds the communication channels
   to all active workers and spawns new ones if they don't exist.
*/

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, RwLock};

use crate::messages::{DirectorCommand, WorkerCommand};
use crate::types::Runtime;

// tx: transmitter, rx: receiver
#[derive(Clone)]
pub struct Director {
    /* The routing table: Maps an Agent ID to its specific Walkie-Talkie transmitter.
       We use RwLock to allow multiple concurrent reads (fast routing).
    */
    routing_table: Arc<RwLock<HashMap<String, mpsc::Sender<WorkerCommand>>>>,

    /* Transmitter to send lifecycle messages to the Director's own background loop */
    director_tx: mpsc::Sender<DirectorCommand>,
}

impl Director {
    pub fn new() -> Self {
        let routing_table = Arc::new(RwLock::new(HashMap::new()));
        let (director_tx, mut director_rx) = mpsc::channel::<DirectorCommand>(100);

        let table_clone = routing_table.clone();

        /* The Director's Background Maintenance Loop.
           It listens for workers that have terminated themselves due to inactivity
           and removes them from the routing table.
        */
        tokio::spawn(async move {
            while let Some(command) = director_rx.recv().await {
                match command {
                    DirectorCommand::WorkerTerminated { id } => {
                        let mut table = table_clone.write().await;
                        table.remove(&id);
                        // Agent is now officially forgotten by the system
                    }
                }
            }
        });

        Self {
            routing_table,
            director_tx,
        }
    }

    /* Main entry point for the Web Server.
       Routes a payload to an agent, spawning it if it's not currently active (Cold Start).
    */
    pub async fn execute_task(
        &self,
        id: String,
        runtime: Runtime,
        entrypoint: String,
        workdir: String,
        payload: String,
    ) -> Result<String, String> {
        let worker_tx = {
            // 1. Read-only lock: Super fast, allows concurrent access.
            let table = self.routing_table.read().await;
            table.get(&id).cloned()
        };

        let tx = match worker_tx {
            Some(sender) => sender,
            None => {
                // 2. Cold Start: The agent is not running. We need to spawn it.
                // We drop the read lock and acquire the write lock.
                let mut table = self.routing_table.write().await;

                // Double-check pattern in case another thread spawned it while we were waiting
                if let Some(sender) = table.get(&id) {
                    sender.clone()
                } else {
                    // Spawn the new worker actor (logic to be implemented in worker.rs)
                    // let new_tx = Worker::spawn(id.clone(), runtime, entrypoint, workdir, self.director_tx.clone()).await?;

                    // Mocking the channel creation until we build the Worker
                    let (new_tx, _new_rx) = mpsc::channel(32);

                    table.insert(id.clone(), new_tx.clone());
                    new_tx
                }
            }
        };

        // 3. Create the return envelope (oneshot channel)
        let (reply_tx, reply_rx) = oneshot::channel();

        // 4. Send the command to the Worker asynchronously
        tx.send(WorkerCommand::Execute {
            input: payload,
            reply_channel: reply_tx,
        })
        .await
        .map_err(|_| format!("Failed to send message to worker {}", id))?;

        // 5. Wait for the specific Worker to process and return the result
        match reply_rx.await {
            Ok(Ok(response)) => Ok(response),
            Ok(Err(worker_err)) => Err(worker_err),
            Err(_) => Err(format!(
                "Worker {} dropped the channel before responding",
                id
            )),
        }
    }
}
