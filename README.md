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
  "tax_id": 12312312312,
  "bank_accounts": [
    {
      "name": "Some Account Name",
      "iban": "DE10 1010 1010 1010 1010",
      "url": "https://banking.bank.com"
    }
  ]
}
```

Add the region code of your social security number in `social_security_numbers.rs` in the enum and also in its validation `ReyFrom<&str> for RegionCode`.

Finally, the application can be bundled and installed:

```
cargo bundle --release
mv ./target/release/bundle/osx/Stammdaten.app /Applications/
```

## Tests

```
cargo test
```
