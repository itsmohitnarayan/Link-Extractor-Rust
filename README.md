# 🚀 **Web Scraper in Rust** 🚀


This project is a simple web scraper implemented in Rust using the `reqwest` and `select` libraries. It fetches a webpage (in this case, [https://www.rust-lang.org/en-US/](https://www.rust-lang.org/en-US/)) and extracts all the hyperlinks (anchor tags) from the HTML.

## Features

- 🌐 Asynchronous web scraping using `tokio::main`.
- 🦀 Utilizes the `reqwest` library for making HTTP requests.
- 📑 HTML parsing with the `select` library.

## Installation

1. Make sure you have Rust installed. If not, you can [install Rust](https://www.rust-lang.org/learn/get-started).

2. Clone this repository:

   ```bash
   git clone https://github.com/itsmohitnarayan/Link-Extractor-Rust.git
   cd Link-Extractor-Rust
   ```

3. Build and run the project:

   ```bash
   cargo run
   ```

## Dependencies

- [`reqwest`](https://crates.io/crates/reqwest) - HTTP client for Rust.
- [`select`](https://crates.io/crates/select) - A Rust library for selecting and manipulating HTML.

```
error-chain = "0.12.4"
reqwest = "0.11.24"
select = "0.6.0"
tokio = { version = "1.36.0", features = ["full"] }
```

## Usage

The main functionality of the web scraper is demonstrated in the provided code snippet. It fetches the specified URL and prints all the hyperlinks found on the webpage.


## Contributing

If you'd like to contribute to this project, please follow the [Contribution Guidelines](CONTRIBUTING.md).

## License

This project is licensed under the [MIT License](LICENSE).


------------------------------------------------------------------------------------------------
