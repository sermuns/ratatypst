#let core = plugin(
  "./target/wasm32-unknown-unknown/release/ratatypst_core.wasm",
)

#let run = raw(str(core.run()))
