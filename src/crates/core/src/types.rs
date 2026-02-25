/* Core types for the AetherFlow orchestration engine.
   This file contains shared structures and enums.
*/

use tokio::process::{Child, ChildStdin, ChildStdout};

/* The Runtime enum defines how an agent should be invoked.
   It covers local scripts, binaries, and remote services.
*/
#[derive(Debug, Clone)]
pub enum Runtime {
    Python3,
    NodeJS,
    Native,
    /* RemoteApi: External services reachable via HTTP */
    RemoteApi { endpoint: String, method: String },
}

/* LiveProcess holds the physical handles to a running agent.
   We keep this in memory to avoid cold starts.
*/
pub struct LiveProcess {
    pub handle: Child,
    pub stdin: ChildStdin,
    pub stdout: ChildStdout,
}
