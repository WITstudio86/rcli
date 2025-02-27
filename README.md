# Rust Generate Template

## preparatory

```shell
$ cargo install cargo-generate
```

## use

- first , install cargo make

```shell
$ cargo install cargo-make
$ cargo install cargp-release
```

- edit `Makefile.toml` add what crate you need ,default auto push 

- replace your repo url in `Cargo.toml` line 56 

- run : `cargo make commit` auto check code and addd commit
- run : `cargo make patch` -> `v[major][minor]>patch<` and push
- run : `cargo make minor` -> `v[major]>minor<[patch]` and push
- run : `cargo make major` -> `v>major<[minor][patch]` and push
