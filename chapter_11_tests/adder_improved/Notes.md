# Notes

This is how you structure a library's tests.
Everything under the tests/ directory is for integration testing only,
whereas everything in lib.rs is for unit testing.
A good way to separate things is to test private functions, structs, enums, etc. in lib.rs
and test all public functions, structs, enums, etc in the tests/ directory.

Running `cargo test` in the root dir will automatically run everything.

We also add submodules in the tests/ dir:

``` bash
├── Cargo.lock
├── Cargo.toml
├── Notes.md
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

These submodules are meant to implement some kind of reusable code, maybe returns a hashmap whose values are specific versions of generic typed structs.

In our example, common has a setup() function that returns a HashMap of the enum Cases.
We use .get().unwrap_or_else(|| panic!) to try various implementations of the Example struct.