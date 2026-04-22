#import plugin("release/panels.wasm"): run

#let _to-le-bytes-u16(num) = num.to-bytes(
  endian: "little",
  size: 2,
)

#show raw: set text(
  font: "JetBrains Mono",
)

#(
  eval(
    str(
      run(
        _to-le-bytes-u16(80),
        _to-le-bytes-u16(60),
        bytes("kl"),
      ),
    ),
    scope: (
      Reset: black,
    ),
  )
)
