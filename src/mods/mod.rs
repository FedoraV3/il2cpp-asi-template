//! Put your mod code here.
//!
//! `init` runs on the bridge's worker thread once the metadata cache is
//! hydrated, so the runtime is safe to touch from it. Any thread you spawn
//! yourself must be attached first:
//!
//! ```ignore
//! std::thread::spawn(|| {
//!     let _thread = il2cpp_bridge_rs::api::Thread::attach(true);
//!     // ... runtime work ...
//! });
//! ```

use il2cpp_bridge_rs::{api, logger};

pub fn init() {
    let Some(assembly) = api::cache::assembly("Assembly-CSharp") else {
        logger::error("Assembly-CSharp not found");
        return;
    };

    logger::info(&format!("{} loaded, {} classes", assembly.name, assembly.classes.len()));
}
