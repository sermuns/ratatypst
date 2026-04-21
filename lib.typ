#let core = plugin(
  "./target/wasm32-unknown-unknown/release/ratatypst_core.wasm",
)

#let hello = str(core.run())
