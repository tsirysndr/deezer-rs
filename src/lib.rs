use reqwest::Client;
use std::time::Duration;

pub mod album;
pub mod artist;
pub mod chart;
pub mod editorial;
pub mod genre;
pub mod infos;
pub mod options;
pub mod playlist;
pub mod radio;
pub mod search;
pub mod track;
pub mod user;

// TODO: Only have one client and derive Clone
pub struct Deezer {
    pub album: album::AlbumService,
    pub artist: artist::ArtistService,
    pub chart: chart::ChartService,
    pub editorial: editorial::EditorialService,
    pub genre: genre::GenreService,
    pub infos: infos::InfosService,
    pub options: options::OptionsService,
    pub playlist: playlist::PlaylistService,
    pub radio: radio::RadioService,
    pub search: search::SearchService,
    pub track: track::TrackService,
    pub user: user::UserService,
}

pub const BASE_URL: &str = "https://api.deezer.com/";

impl Deezer {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        Self {
            album: album::AlbumService::new(&client),
            artist: artist::ArtistService::new(&client),
            chart: chart::ChartService::new(&client),
            editorial: editorial::EditorialService::new(&client),
            genre: genre::GenreService::new(&client),
            infos: infos::InfosService::new(&client),
            options: options::OptionsService::new(&client),
            playlist: playlist::PlaylistService::new(&client),
            radio: radio::RadioService::new(&client),
            search: search::SearchService::new(&client),
            track: track::TrackService::new(&client),
            user: user::UserService::new(&client),
        }
    }
}

impl Default for Deezer {
    fn default() -> Self {
        Self::new()
    }
}
