# Takuzu solver

Solver for takuzu (a.k.a. binairo) number puzzles.

### Usage

To use the solver:

```shell
$ git clone https://github.com/letheed/takuzu.git
$ cd takuzu
$ cargo run --release [FILE]...
```
or

```shell
$ cargo install takuzu
$ takuzu-solver [FILE]...
```

![solving grid2 screenshot](docs/solving_grid2.png)

### Documentation

For the `FILE` format, see the [library documentation](https://letheed.github.io/takuzu/takuzu) and the [example grids](grids).

For additional details on the binary, see the [binary documentation](https://letheed.github.io/takuzu/takuzu_solver).
