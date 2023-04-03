use deezer_rs::Deezer;

#[tokio::main]
async fn main() {
    let client = Deezer::new();
    // let album = client.album().get("302127").await;
    // let fans = client.album().get_fans("302127").await;
    // let tracks = client.album().get_tracks("302127").await;
    // let artist = client.artist().get("27").await;
    // let top = client.artist().get_top_five("27").await;
    // let artist_albums = client.artist().get_albums("27").await;
    // let artist_playlists = client.artist().get_playlists("27").await;
    // let artist_fans = client.artist().get_fans("27").await;
    // let artist_related = client.artist().get_related("27").await;
    // let artist_radio = client.artist().get_radio("27").await;
    // let chart_tracks = client.chart().get_tracks().await;
    // let chart_albums = client.chart().get_albums().await;
    // let chart_artists = client.chart().get_artists().await;
    // let chart_playlists = client.chart().get_playlists().await;
    // let genre = client.genre().get("0").await;
    // let genres = client.genre().list().await;
    // let genre_artists = client.genre().get_artists("0").await;
    // let radios_genres = client.radio().by_genre().await;
    // let radio_top = client.radio().get_top_radio().await;
    // let radios_tracks = client.radio().get_tracks("6").await;
    // let radios = client.radio().list().await;
    let search = client.search().get("eminem").await;
    let albums = client.search().get_albums("eminem").await;
    let artists = client.search().get_artists("eminem").await;
    let playlists = client.search().get_playlists("eminem").await;
    let radio_results = client.search().get_radio("eminem").await;
    let track_results = client.search().get_tracks("eminem").await;
    let track = client.track().get("3135556").await;

    // println!("{:#?}\n", album.unwrap());
    // println!("{:#?}\n", fans.unwrap());
    // println!("{:#?}\n", tracks.unwrap());
    // println!("{:#?}\n", artist.unwrap());
    // println!("{:#?}\n", top.unwrap());
    // println!("{:#?}\n", artist_albums.unwrap());
    // println!("{:#?}\n", artist_playlists.unwrap());
    // println!("{:#?}\n", artist_fans.unwrap());
    // println!("{:#?}\n", artist_related.unwrap());
    // println!("{:#?}\n", artist_radio.unwrap());
    // println!("{:#?}\n", chart_tracks.unwrap());
    // println!("{:#?}\n", chart_albums.unwrap());
    // println!("{:#?}\n", chart_artists.unwrap());
    // println!("{:#?}\n", chart_playlists.unwrap());
    // println!("{:#?}\n", genre.unwrap());
    // println!("{:#?}\n", genres.unwrap());
    // println!("{:#?}\n", genre_artists.unwrap());
    // println!("{:#?}\n", radios_genres.unwrap());
    // println!("{:#?}\n", radio_top.unwrap());
    // println!("{:#?}\n", radios_tracks.unwrap());
    // println!("{:#?}\n", radios.unwrap());
    println!("{:#?}\n", search.unwrap());
    println!("{:#?}\n", albums.unwrap());
    println!("{:#?}\n", artists.unwrap());
    println!("{:#?}\n", playlists.unwrap());
    println!("{:#?}\n", radio_results.unwrap());
    println!("{:#?}\n", track_results.unwrap());
    println!("{:#?}\n", track.unwrap());
}
