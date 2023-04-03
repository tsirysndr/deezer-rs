<h1 align="left">deezer-rs</h1>
<p>
  <a href="#" target="_blank">
    <img alt="License: BSD" src="https://img.shields.io/badge/License-BSD-green.svg" />
  </a>
  <a href="https://github.com/tsirysndr/deezer-rs/commits/master">
    <img src="https://img.shields.io/github/last-commit/tsirysndr/deezer-rs.svg" target="_blank" />
  </a>
  <a href="https://twitter.com/tsiry_sndr" target="_blank">
    <img alt="Twitter: tsiry_sndr" src="https://img.shields.io/twitter/follow/tsiry_sndr.svg?style=social" />
  </a>
</p>

deezer-rs is a Rust client library for accessing the [Deezer API](https://developers.deezer.com/api)


## Install

```toml
[dependencies]
deezer-rs = { git = "https://github.com/tsirysndr/deezer-rs" }
```

## Usage

Construct a new Deezer client, then use the various services on the client to access different parts of the Deezer API. For example:

```rust
use deezer_rs::Deezer;

#[tokio::main]
async fn main() {
  let client = Deezer::new();
  let album = client.album().get("302127").await;
  let tracks = client.album().get_tracks("302127").await;
  let artist = client.artist().get("27").await;
  println!("{:#?}\n", album.unwrap());
  println!("{:#?}\n", tracks.unwrap());
  println!("{:#?}\n", artist.unwrap());
}
```

## Author

👤 **Tsiry Sandratraina**

* Website: https://tsiry-sandratraina.netlify.com
* Twitter: [@tsiry_sndr](https://twitter.com/tsiry_sndr)
* Github: [@tsirysndr](https://github.com/tsirysndr)

## Show your support

Give a ⭐️ if this project helped you!
