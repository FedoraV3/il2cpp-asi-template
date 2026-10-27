# il2cpp-asi-template

A starting point for Unity IL2CPP mods as an ASI plugin, using
[`il2cpp-bridge-rs`](https://crates.io/crates/il2cpp-bridge-rs).

## Why this exists

`il2cpp-bridge-rs` resolves its IL2CPP function table through
`memory::symbol::resolve_symbol`. On Windows that does:

```rust
let handle = GetModuleHandleA(std::ptr::null());   // the .exe
GetProcAddress(handle, symbol)
```

A null module handle is the *host executable*, but the IL2CPP runtime lives in
`GameAssembly.dll`. So every `il2cpp_*` lookup misses and `init` fails with a
list of unresolved symbols.

`resolve_symbol` checks an internal cache before it ever reaches that platform
path, and `memory::symbol::cache_symbol` is public. `src/il2cpp_symbols.rs`
walks `GameAssembly.dll`'s export directory and seeds every `il2cpp_*` export
into that cache. `InitializeASI` then calls `init`.

**Ordering is the whole trick.** Seeding must finish before `init` spawns its
worker, or `api::load` runs against an empty cache.

## Using it

1. Copy the folder, then rename the package in `Cargo.toml`.
2. Write your mod in `src/mods/`. `mods::init()` runs after the metadata cache
   is hydrated.
3. `cargo build --release`, then rename `target/release/<crate>.dll` to `.asi`
   and drop it next to the game executable.

Set `RUST_LOG=debug` to see the bridge's own logging.

## Things that will bite you

- **`GAME_ASSEMBLY` must keep the `.dll` extension.** The crate derives method
  RVAs by subtracting that module's base from each function pointer. A bare
  `"GameAssembly"` makes `GetModuleHandleW` miss and every RVA comes out `0`.
- **Threads you spawn need `api::Thread::attach(true)`.** Only the bridge's own
  worker is attached for you.
- **This needs `GameAssembly.dll` to still export its symbols by name.** If a
  build strips or renames them, this approach dies and you need signature
  scanning instead.
- **`crate-type = ["cdylib"]` is required.** Without it you get an `.rlib` the
  game cannot load.

## Layout

| Path | Purpose |
| --- | --- |
| `src/lib.rs` | `InitializeASI` entry point, bootstrap ordering |
| `src/il2cpp_symbols.rs` | Export-table walker that seeds the symbol cache |
| `src/mods/mod.rs` | Your code goes here |
