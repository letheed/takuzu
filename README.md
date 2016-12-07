# Takuzu solver

Library for solving takuzu (a.k.a. binairo) number puzzles.


### Documentation

See the [documentation](https://docs.rs/takuzu) and the [example grids](https://github.com/letheed/takuzu/tree/master/grids).


### Import

Add this to your `Cargo.toml`:

```toml
[dependencies]
takuzu = "^0.5"
```

and this to your crate root:

```rust
extern crate takuzu;
```


### Solver

The crate also provides a binary. To use the solver:

```shell
$ git clone https://github.com/letheed/takuzu.git
$ cd takuzu
$ cargo run --release [FILE]...
```

or

```shell
$ cargo install takuzu
$ takuzu [FILE]...
```

![solving grid2 screenshot](https://raw.githubusercontent.com/letheed/takuzu/master/docs/solving_grid2.png)
