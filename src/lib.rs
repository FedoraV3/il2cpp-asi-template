//! ASI plugin template for Unity IL2CPP games.
//!
//! `InitializeASI` is the loader entry point. It seeds the IL2CPP export cache
//! from `GameAssembly.dll` and only then starts the bridge, so the metadata
//! cache is hydrated before any mod code runs.
//!
//! See `README.md` for the parts you need to change per project.

mod il2cpp_symbols;
mod mods;

use il2cpp_bridge_rs::logger;

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
unsafe extern "C" fn InitializeASI() {
    match il2cpp_symbols::preload_il2cpp_symbols() {
        Ok(cached) => {
            logger::info(&format!("Cached {cached} IL2CPP export(s)"));

            il2cpp_bridge_rs::init(il2cpp_symbols::GAME_ASSEMBLY, || {
                logger::info("IL2CPP ready");
                mods::init();
            });
        }
        Err(error) => logger::error(&format!("IL2CPP bootstrap failed: {error:#}")),
    }
}
