# SDL2 Rust Template

An SDL2 template for Rust, targeting Linux, MacOS, and Windows (x64 MSVC) builds.

[![rust-sdl2 documentation](https://img.shields.io/badge/docs-rust--sdl2-blue)](https://docs.rs/sdl2/0.34.3/sdl2/)

## Project Generation

Ensure [`cargo-generate`](https://github.com/ashleygwilliams/cargo-generate) is installed:

```sh
cargo install cargo-generate
```

And generate with:

```sh
cargo generate --git hgalant/sdl2_rust_template
```

## Environment

> Note: The template's build script assumes the environment variable `CARGO_TARGET_DIR` is unset when building for Windows.

- For Linux/MacOS builds: install SDL2 and other necessary packages (e.g. SDL_image) via your package-manager (more details [here](https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries)).

- For Windows builds: library files are included for SDL2, SDL_image, and SDL_ttf in `lib/`.
You can remove unnecessary or add unincluded SDL dependencies (e.g. [SDL_mixer](https://www.libsdl.org/projects/SDL_mixer/)) as needed.