<div align="center">

<a href="https://github.com/sermuns/ratatypst">
  <img alt="ratatypst banner" src="media/banner.svg">
</a>

_finally.. `ratatui` in Typst.._

[![Built With Ratatui](https://img.shields.io/badge/Built_With_Ratatui-000?logo=ratatui&logoColor=fff)](https://ratatui.rs/)

</div>

![demo](media/demo.avif)

This is a terribly barebones and shitty proof-of-concept for writing Ratatui apps for Typst.

Inspired by [`soviet-matrix`](https://typst.app/universe/package/soviet-matrix/)

## Try it out!

1. Clone this repo.

2. Compile the Rust code:

   ```sh
   just build
   ```

3. Try out the example, either:
   - Edit the file with [`typst-preview.nvim`](https://github.com/chomosuke/typst-preview.nvim) which can re-render a preview on every keystroke.

   - Use [`typst` CLI](https://repology.org/project/typst/versions) (**not as cool**)

     ```sh
     typst watch --root . examples/hello.typ
     ```

     which automatically recompiles the PDF `examples/hello.pdf` everytime the `examples/hello.typ` is saved with modifications.

## Disclaimer

This code is 100% certified human-slop. **No artificial intelligence was used in the making of this.**

<a href="https://brainmade.org/">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://brainmade.org/white-logo.svg">
  <source media="(prefers-color-scheme: light)" srcset="https://brainmade.org/black-logo.svg">
  <img alt="brainmade" src="https://brainmade.org/white-logo.svg">
</picture>
</a>
