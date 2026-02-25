/*
   This module defines the communication protocol (the 'Walkie-Talkie' messages)
   between the Director and the Workers.
*/

use tokio::sync::oneshot;

/* --------------------------------------------------------------------------
Messages sent FROM the Director TO a specific Worker.
These are the commands a Worker actor can receive and process in its loop.
-------------------------------------------------------------------------- */
#[derive(Debug)]
pub enum WorkerCommand {
    /* Instruction to execute a task.
       - input: The string payload to send to the physical agent's stdin.
       - reply_channel: A one-time use channel to send the agent's stdout
         back to the caller without blocking the Worker's main loop.
    */
    Execute {
        input: String,
        reply_channel: oneshot::Sender<Result<String, String>>,
    },

    /* Instruction to force a physical process shutdown immediately.
       Useful if the user explicitly requests to kill the agent via API.
    */
    Shutdown,
}

/* --------------------------------------------------------------------------
Messages sent FROM a Worker TO the Director.
These are usually lifecycle events or status updates to keep the registry in sync.
-------------------------------------------------------------------------- */
#[derive(Debug)]
pub enum DirectorCommand {
    /* Notification that a Worker has cleanly terminated itself (e.g., due to
       the 60-second inactivity timeout).
       The Director must remove this agent's ID from its internal routing table
       to prevent future messages from being sent to a dead channel.
    */
    WorkerTerminated { id: String },
}
