use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // read this from clap
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
        let album = match tag.album() {
            Some(v) => v,
            None    => {
                println!("no album for {}", file);
                continue;
            }
        };
        let title = match tag.title() {
            Some(v) => v,
            None    => {
                println!("no title for {}", file);
                continue;
            }
        };

        println!("'{}' — '{}' — '{}'", artist, album, title);
    }

    Ok(())
}
