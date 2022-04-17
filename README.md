vestalia
=========

This crate is an async wrapper for the Vestaboard API. It is a third-party developed crate
and has no relation to Vestaboard, Inc.

## [Change log](CHANGELOG.md)

## Features

- Supports Vestaboard API key pairs and auto-configuration of the subscription id
- Validation of text and vector character inputs
- Utilities such as converting text lines into vectors to support Vestaboard character mapping

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
vestalia = "0.1.0"
```

Then:

```rust
use vestalia::Vestaboard;

let client = Vestaboard::new(api_key, api_secret);
let response = client.text("Hello world!").await;
// If you'd like to handle API errors... Do something like this
match response {
    Ok(post) => println!("{:#?}", post),
    Err(error) => panic!("{}", error),
}
```

## License

Licensed under the MIT license:
 * MIT license ([LICENSE](LICENSE) or https://opensource.org/licenses/MIT)

### Contribution

If you are interested in contributing, feel free to send a PR! This is my first
foray into rust lang so always open to feedback and suggestions. If you have any problems
please feel free to open an issue as well.

While I can do what I can to help, I do not have control over the Vestaboard API itself,
so feature requests and support will be limited to what the official API can support.