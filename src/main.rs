use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};
use std::time::{SystemTime, UNIX_EPOCH};

use rspotify::spotify::client::Spotify;
use rspotify::spotify::util::get_token;
use rspotify::spotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};

fn main() -> std::io::Result<()> {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-read-private playlist-modify-private")
        .build();
    let token_info = get_token(&mut oauth).unwrap();
    let client_credential = SpotifyClientCredentials::default()
        .token_info(token_info)
        .build();
    // Or set client_id and client_secret explictly
    // let client_credential = SpotifyClientCredentials::default()
    //     .client_id("this-is-my-client-id")
    //     .client_secret("this-is-my-client-secret")
    //     .build();
    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();

    // create playlist
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    println!("{:?}", now);
    let user_id = "utrch63s7u3bha3i01phwkujg";
    let playlist_name = format!("playlist {}", now);
    let playlist = spotify.user_playlist_create(user_id, &playlist_name, false, None).unwrap();

    let mut file = File::open("/home/mark/Documents/music/good.m3u")?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    for file in buffer.lines() {
        if file.starts_with("#") {
            continue;
        }
        let file = format!("/home/mark/Music/{}", file);
        let tag = match id3::Tag::read_from_path(&file) {
            Ok(tag) => tag,
            Err(e)  => {
                println!("error [{}] reading tags from {}", e, file);
                continue;
            },
        };

        let artist = match tag.artist() {
            Some(v) => v,
            None    => {
                println!("no artist for {}", file);
                continue;
            }
        };
        // let album = match tag.album() {
        //     Some(v) => v,
        //     None    => {
        //         println!("no album for {}", file);
        //         continue;
        //     }
        // };
        let title = match tag.title() {
            Some(v) => v,
            None    => {
                println!("no title for {}", file);
                continue;
            }
        };

        let query = format!("artist:\"{}\" AND track:\"{}\"", artist, title);
        let result = spotify.search_track(query.as_str(), 10, 0, None);
        if let Ok(result) = result {
            if let Some(track) = result.tracks.items.get(0) {
                // println!("{}", query);
                // println!("search result: {}", track.uri);
                if let Some(track_id) = track.id.clone() {
                    spotify.user_playlist_add_tracks(user_id, &playlist.id, &[track_id], None).unwrap();
                }
                else {
                    println!("failed to add {} - {} to playlist", query, track.uri)
                }
            }
        }
        else {
            println!("nothing found for {}", query);
        };

        //sleep so we don't get rate limited
        thread::sleep(time::Duration::from_secs(2));
    }

    Ok(())
}


