# Qilin - Tiny 2D Game Engine

**Qilin - 'Unicorn' of the ancient chinese Mythology**

## What is this?
Qilin is a super lightweight 2D Game Engine made with Rust.\
The focus of this Project is to develop a Game Engine to make 2D Games fast, performant and without a bunch of bytes.

## Goals
- **Fun**: Qilin is not a serious engine, but rather a fun Project to make fun Games with fun.
- **Compile Times**: Developing Games in Rust can be very slow, due to compile times. Qilin tries to fix this by reducing features & dependencies.
- **Performance:** While having great compile times, Qilin still tries to use the features of Rust to optimize performance and memory usage.
- **Modular:** Qilin is tiny by default. You can however add more features and add additions to your game-flow.
- **Lightweight:** Qilin is designed to make games with, however if you like to you can build your own engine on top.

## Cargo Features
- `default`: Doesn't contain any other features, so you can 100% customize qilin.
- `minifb`: Exports internal minifb crate.
- `image`: Exports module to convert Images from the `image` crate to Qilin Images.
