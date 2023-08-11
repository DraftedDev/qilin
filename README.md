# Qilin - Tiny 2D Game Engine

**Qilin - 'Unicorn' of the ancient chinese Mythology**

## What is this?

Qilin is a super lightweight 2D Game Engine made with Rust.\
The focus of this Project is to develop a Game Engine to make 2D Games fast, performant and without a bunch of bytes.

## Goals

- **Fun**: Qilin is not a serious engine, but rather a fun Project to make fun Games with fun.
- **Compile Times**: Developing Games in Rust can be very slow, due to compile times. Qilin tries to fix this by reducing features & dependencies.
- **Performance:** While having great compile times, Qilin still tries to use the features of Rust to optimize performance and memory usage.
- **Modular:** Qilin is tiny by default. You can however add more features and additions to your game-flow.
- **Lightweight:** Qilin is tiny and inspired by [MonoGame](https://en.wikipedia.org/wiki/MonoGame), so you can build your own engine on top, if you want.

## Cargo Features

- `text`: Contain the `text` module.
- `audio`: Contains the `audio` module.
- `minifb`: Exports internal minifb crate.
- `image`: Exports module to convert Images from the `image` crate to Qilin Images.
- `store`: Adds the `PlayerPrefs` struct to store game data.
- `serde`: Adds [serde](https://serde.rs) support for common types. Also enables the `serde` feature for the `mint` crate.
- `plugin-dev`: Adds plugin development support. You won't need this if you just want to register/use plugins.

## Submitting Issues

Make sure your issue isn't a duplicate and please respect the Code of Conduct.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Qilin is dual licensed under the MIT and Apache 2.0 License.\
Unless you explicitly state otherwise, your contributed code will also be dual licensed under these terms.
