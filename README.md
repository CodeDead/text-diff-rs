# text-diff

![GitHub release (latest by date)](https://img.shields.io/github/v/release/CodeDead/text-diff-rs)
![GitHub](https://img.shields.io/badge/language-Rust-green)
![GitHub](https://img.shields.io/github/license/CodeDead/text-diff-rs)

![text-diff](https://i.imgur.com/VrcSyMD.png)

`text-diff` can be used to compare two text files and output the difference between them. `text-diff` was written in `Rust` and offers a simple and intuitive user-interface for Windows, Linux and macOS.

## Building

In order to build `text-diff`, you can run the following command:

```shell
cargo build
```

A `release` build with extra optimizations can be built by issuing the following command:
```shell
cargo build --release
```

### Tips

On `Linux` you can further decrease the binary size by issuing a `strip` command, which will 'strip' symbols from the object file:
```shell
strip filename
```

## Running

You can run `text-diff` directly by issuing the following command:
```shell
cargo run
```

## Credits

* [Rust](https://www.rust-lang.org/)
* [iced](https://iced.rs/)
* [native-dialog](https://github.com/balthild/native-dialog-rs)
* [serde](https://serde.rs/)
* [serde_json](https://serde.rs/)

## About

This library is maintained by CodeDead. You can find more about us using the following links:

* [Website](https://codedead.com)
* [Twitter](https://twitter.com/C0DEDEAD)
* [Facebook](https://facebook.com/deadlinecodedead)

Copyright © 2022 CodeDead
