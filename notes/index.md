# TODO

- [x] Execute lua
- [x] Pass global variables
- [x] Mutate global variables and read them back in Rust
  - [x] Primitives
  - [x] Tables
- [x] Pass functions
- [x] Pass object with attributes and methods
- [x] Serialize/Deserialize into Rust struct
- [x] Handle `Result`
- [x] Handle user `require`s
- [x] Do a `require`able library (different crate, cdylib)
  - For some reason it fails when called from Rust, but it works from luajit...
