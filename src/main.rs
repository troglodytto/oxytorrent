extern crate serde;
extern crate serde_bencode;
#[macro_use]
extern crate serde_derive;
extern crate serde_bytes;
use serde_bencode::de;
use serde_bytes::ByteBuf;
use std::{fmt::Display, fs::File, io::Result};

#[derive(Debug, Deserialize)]
struct Node(String, i64);

#[derive(Debug, Deserialize)]
struct TorrentFile {
    path: Vec<String>,
    length: i64,
    #[serde(default)]
    md5sum: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Info {
    name: String,
    pieces: ByteBuf,
    #[serde(rename = "piece length")]
    piece_length: i64,
    #[serde(default)]
    md5sum: Option<String>,
    #[serde(default)]
    length: Option<i64>,
    #[serde(default)]
    files: Option<Vec<TorrentFile>>,
    #[serde(default)]
    private: Option<u8>,
    #[serde(default)]
    path: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "root hash")]
    root_hash: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Torrent {
    info: Info,
    #[serde(default)]
    announce: Option<String>,
    #[serde(default)]
    nodes: Option<Vec<Node>>,
    #[serde(default)]
    encoding: Option<String>,
    #[serde(default)]
    httpseeds: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "announce-list")]
    announce_list: Option<Vec<Vec<String>>>,
    #[serde(default)]
    #[serde(rename = "creation date")]
    creation_date: Option<i64>,
    #[serde(rename = "comment")]
    comment: Option<String>,
    #[serde(default)]
    #[serde(rename = "created by")]
    created_by: Option<String>,
}

impl Display for Torrent {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(formatter, "name:\t\t{}", self.info.name)?;
        writeln!(formatter, "announce:\t{:?}", self.announce)?;
        writeln!(formatter, "nodes:\t\t{:?}", self.nodes)?;

        if let Some(al) = &self.announce_list {
            for a in al {
                writeln!(formatter, "announce list:\t{:?}", a)?;
            }
        }

        writeln!(formatter, "httpseeds:\t{:?}", self.httpseeds)?;
        writeln!(formatter, "creation date:\t{:?}", self.creation_date)?;
        writeln!(formatter, "comment:\t{:?}", self.comment)?;
        writeln!(formatter, "created by:\t{:?}", self.created_by)?;
        writeln!(formatter, "encoding:\t{:?}", self.encoding)?;
        writeln!(formatter, "piece length:\t{:?}", self.info.piece_length)?;
        writeln!(formatter, "private:\t{:?}", self.info.private)?;
        writeln!(formatter, "root hash:\t{:?}", self.info.root_hash)?;
        writeln!(formatter, "md5sum:\t\t{:?}", self.info.md5sum)?;
        writeln!(formatter, "path:\t\t{:?}", self.info.path)?;

        if let Some(files) = &self.info.files {
            for file in files {
                writeln!(formatter, "file path:\t{:?}", file.path)?;
                writeln!(formatter, "file length:\t{}", file.length)?;
                writeln!(formatter, "file md5sum:\t{:?}", file.md5sum)?;
            }
        }

        Ok(())
    }
}

impl Torrent {
    fn new(buffer: Vec<u8>) -> Result<Torrent> {
        let torrent = match de::from_bytes::<Torrent>(&buffer) {
            Ok(torrent) => torrent,
            Err(error) => todo!("Error"),
        };

        Ok(torrent)
    }

    fn download(&self) -> Result<File> {
        let file = File::create("./result.txt")?;
        Ok(file)
    }
}

fn main() -> Result<()> {
    let file_buffer = match std::fs::read("./kali.torrent") {
        Ok(buffer) => buffer,
        Err(error) => panic!("Failed to open file due to following error {}", error),
    };

    let torrent = match Torrent::new(file_buffer) {
        Ok(torrent) => torrent,
        Err(error) => panic!("Failed to read file due to following error {}", error),
    };

    let download = match torrent.download() {
        Ok(download) => download,
        Err(error) => panic!("Download failed due to following error {}", error),
    };

    Ok(())
}
