#let core = plugin(
  "./target/wasm32-unknown-unknown/release/ratatypst_core.wasm",
)

#let _to-le-bytes-u16(num) = num.to-bytes(
  endian: "little",
  size: 2,
)

#let run(width, height) = raw(
  str(
    core.run(
      _to-le-bytes-u16(width),
      _to-le-bytes-u16(height),
    ),
  ),
)
