#let logo = stack(
  dir: ltr,
  spacing: 1.5%,
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
