use deezer_rs::Deezer;

#[tokio::main]
async fn main() {
  let client = Deezer::new();
  let album = client.album.get("302127").await;
  let fans = client.album.get_fans("302127").await;
  let tracks = client.album.get_tracks("302127").await;
  println!("{:#?}\n", album.unwrap());
  println!("{:#?}\n", fans.unwrap());
  println!("{:#?}\n", tracks.unwrap());
}
