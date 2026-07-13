# Guessing Game

Started with the [Rust tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html),
just to try something new. Later, added some logics to make it more fun.

---

## Features

- Setting random number range
- Setting attempt limit
- Prompt attempt count
- Suggesting next best guess

---

## Compile vs Build

`rustc` vs `cargo`

### Compile

Process of translating Rust code into **machine code**.

- Check for errors
- Perform optimizations
- generate an executable or library

```sh
# compile
rustc main.rs -o main

# run
./main
```

### Build

Broader process that **includes** compilation, also everything
needed to produce the final artifact.

1. Read `Cargo.toml`
2. Resolve dependencies
3. Download missing `crates`
4. Compile dependencies
5. Compile your `crate`
6. Link everything
7. Store artifact to `target` directory
8. Rebuild only what has changed

```sh
cargo build

cargo run
```

---

#### Stay Rusty 🦀
