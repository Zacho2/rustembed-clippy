Short example to reproduce rust-embed clippy warning on [same_name_method](https://rust-lang.github.io/rust-clippy/master/#same_name_method).

From the repo root:
```
$ cargo clippy -- -W clippy::same_name_method
    Checking rustembed-clippy v0.1.0 (/home/Documents/src/rustembed-clippy)
warning: method's name is the same as an existing method in a trait
 --> src/main.rs:3:10
  |
3 | #[derive(RustEmbed)]
  |          ^^^^^^^^^
  |
note: existing `get` defined here
 --> src/main.rs:3:10
  |
3 | #[derive(RustEmbed)]
  |          ^^^^^^^^^
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#same_name_method
  = note: requested on the command line with `-W clippy::same-name-method`
  = note: this warning originates in the derive macro `RustEmbed` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: `rustembed-clippy` (bin "rustembed-clippy") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
```
