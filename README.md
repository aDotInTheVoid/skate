# Skate

an experimental programming language.

Check out the [playground](https://adotinthevoid.github.io/skate/)

## Status.

Function calls, variables and controll flow are implemented.

Its currently a tree walk interpriter, but it needs to be changed to bytecode.

All the words in [docs](docs) are lofty ambitions we're nowhere near.

For more see [tests/run-pass](tests/run-pass)


## Testing
```bash
# Runs unit tests
cargo test

# Runs e2e tests.
python test.py

# Bless e2e tests
python test.py -b

# Update snapshots
cargo insta test --accept --delete-unreferenced-snapshots
```

## Conduct

In all Skate-related forums, we follow the [Rust Code of Conduct]. For
escalation or moderation issues please contact me (nixon DOT emoony AT gmail)
instead of the Rust moderation team.

[Rust Code of Conduct]: https://www.rust-lang.org/policies/code-of-conduct

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>

