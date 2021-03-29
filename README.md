# Skate


## Testing
```bash
# Runs unit tests

cargo test
# Runs e2e tests. Note that if these fail, it will not show
# but instead just change the file, which will show up in git.
# One day this should change
python test.py

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

