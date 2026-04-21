<div align="center">

<a href="https://github.com/sermuns/meread">
  <img alt="MEREAD" src="media/banner.svg">
</a>

_finally.. `ratatui` in Typst.._

[![Built With Ratatui](https://img.shields.io/badge/Built_With_Ratatui-000?logo=ratatui&logoColor=fff)](https://ratatui.rs/)

</div>

![demo](media/demo.avif)

This is a terribly barebones and shitty proof-of-concept for writing Ratatui apps for Typst.

Inspired by [`soviet-matrix`](https://typst.app/universe/package/soviet-matrix/)

## Try it out!

Compile the Rust code:

```sh
just build
```

Include the function `tui` from `lib.typ` into your example code and try it out!

```typ
#import "../lib.typ": tui

#tui(
  "ljlhlkjhljh",
  width: 50,
  height: 20,
)
```

## Disclaimer

This code is 100% certified human-slop. **No artificial intelligence was used in the making of this.**

<a href="https://brainmade.org/">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://brainmade.org/white-logo.svg">
  <source media="(prefers-color-scheme: light)" srcset="https://brainmade.org/black-logo.svg">
  <img alt="brainmade" src="https://brainmade.org/white-logo.svg">
</picture>
</a>
