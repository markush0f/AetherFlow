/* crates/core/src/lib.rs
   Entry point for the AetherFlow Core crate.
   This file defines the public API that the Web Server will consume.
   We explicitly hide internal mechanics (like the worker) to maintain a clean API.
*/

pub mod director;
pub mod messages;
pub mod runtime;
pub mod types;

/* We declare the worker module, but we do NOT make it public.
   The outside world should never spawn a worker manually;
   they must always go through the Director.
*/
mod worker;

/* Re-exporting the essential components so the Web Server can import them
   directly from 'aetherflow_core' without digging into submodules.
*/
pub use director::Director;
pub use types::Runtime;
