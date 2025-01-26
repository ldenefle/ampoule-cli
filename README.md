# Ampoule CLI 💡

Ampoule CLI is a Rust client for the [ampoule](https://github.com/ldenefle/ampoule) project. 

Right now it only supports blinking a LED over a serial device and is aimed at testing the implementation.

The device module is designed to be fairly extensible and can support any Transport that implements the traits Read, Write and Send.


## Building

Clone the repo and its submodules, the project's sole dependencies are `libudev` and `protoc`.

```
cargo build
```

or one can run the project directly with

```
RUST_LOG=trace cargo run -- -d <serial-port-ampoule-dev>
```

## Testing

The project comes with a set of tests.
 
```
cargo test
```

