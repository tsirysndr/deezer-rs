use deezer_rs::Deezer;

#[tokio::main]
async fn main() {
  let client = Deezer::new();
  let album = client.album.get("302127").await;
  let fans = client.album.get_fans("302127").await;
  let tracks = client.album.get_tracks("302127").await;
  let artist = client.artist.get("27").await;
  let top = client.artist.get_top_five("27").await;
  let artist_albums = client.artist.get_albums("27").await;
  let artist_playlists = client.artist.get_playlists("27").await;
  let artist_fans = client.artist.get_fans("27").await;
  let artist_related = client.artist.get_related("27").await;
  let artist_radio = client.artist.get_radio("27").await;

  println!("{:#?}\n", album.unwrap());
  println!("{:#?}\n", fans.unwrap());
  println!("{:#?}\n", tracks.unwrap());
  println!("{:#?}\n", artist.unwrap());
  println!("{:#?}\n", top.unwrap());
  println!("{:#?}\n", artist_albums.unwrap());
  println!("{:#?}\n", artist_playlists.unwrap());
  println!("{:#?}\n", artist_fans.unwrap());
  println!("{:#?}\n", artist_related.unwrap());
  println!("{:#?}\n", artist_radio.unwrap());
}
