#import plugin(
  "./target/wasm32-unknown-unknown/release/ratatypst_core.wasm",
): run

#let _to-le-bytes-u16(num) = num.to-bytes(
  endian: "little",
  size: 2,
)

#let tui(input, width: none, height: none) = {
  if width == none {
    panic("width must be provided")
  }

  if height == none {
    panic("height must be provided")
  }

  raw(
    str(
      run(
        _to-le-bytes-u16(width),
        _to-le-bytes-u16(height),
        bytes(input),
      ),
    ),
  )
}
