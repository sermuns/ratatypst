#import plugin("panels.wasm"): run

#let _to-le-bytes-u16(num) = num.to-bytes(
  endian: "little",
  size: 2,
)

#show raw: set text(
  font: "JetBrains Mono",
  size: 15pt,
)

#raw(
  str(
    run(
      _to-le-bytes-u16(50),
      _to-le-bytes-u16(20),
      bytes("kljkhlhlhjk"),
    ),
  ),
)
