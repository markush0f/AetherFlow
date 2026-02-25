/* AetherFlow Core Library
   Entry point that exposes types, runtime logic, and process management.
*/

pub mod process_manager;
pub mod runtime;
pub mod types;

/* Re-exporting for convenience, so other crates can use
   oxide_core::Runtime instead of oxide_core::types::Runtime
*/
pub use types::*;
