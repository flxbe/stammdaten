# stammdaten

A simple GUI based on [`druid`](https://github.com/linebender/druid) to display basic personal data.

## Installation

First, `cargo-bundle` must be installed:

```
cargo install cargo-bundle
```

Then, the file `data.json` must be created in the project root folder:

```json
{
  "first_name": "First Name",
  "last_name": "Last Name",
  "social_security_number": "some_value",
  "tax_id": 12312312312
}
```

Finally, the application can be bundled and installed:

```
cargo bundle --release
mv ./target/release/osx/Stammdaten.app /Applications/
```

## Tests

```
cargo test
```
