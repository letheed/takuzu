# Takuzu

A library for solving takuzu (a.k.a. binairo) number puzzles and a binary using the library.

## Documentation

See the [documentation](https://docs.rs/takuzu) and the [example grids](https://github.com/letheed/takuzu/tree/master/grids).

## Import

Add this to your `Cargo.toml`:

```toml
[dependencies]
takuzu = "0.6"
```
and this to your crate root:

```rust
extern crate takuzu;
```

## Solver

The crate also provides a binary. To use the solver:

```bash
$ cargo install takuzu
$ takuzu [FILE]...
```

or

```bash
$ git clone https://github.com/letheed/takuzu.git
$ cd takuzu
$ cargo run --release [FILE]...
```

![solving grid2 screenshot](https://raw.githubusercontent.com/letheed/takuzu/master/img/solving_grid2.png)
