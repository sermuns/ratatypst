#set page(
  width: 1500pt,
  height: 300pt,
  fill: luma(10%),
  margin: 5%,
)
#set align(center + horizon)

#stack(
  dir: ltr,
  spacing: 2%,
  box(
    clip: true,
    width: auto,
    inset: (
      left: -35%,
      right: -80%,
      bottom: 35%,
    ),
    image("ratatui.png"),
  ),
  image("typst.svg", height: 80%),
)
