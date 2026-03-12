// Copyright (c) 2026 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

//! Parser for the Rekordbox XML file format for playlists sharing.
//!
//! The XML format includes all playlists information.
//!
//! # References
//!
//! - <https://rekordbox.com/en/support/developer/>
//! - <https://cdn.rekordbox.com/files/20200410160904/xml_format_list.pdf>
//! - <https://pyrekordbox.readthedocs.io/en/stable/formats/xml.html>
type NaiveDate = String; //Replace with "use chrono::naive::NaiveDate;"
use serde::{de::Error, ser::Serializer, Deserialize, Serialize};
use std::borrow::Cow;

/// The XML root element of a rekordbox XML file.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename = "DJ_PLAYLISTS")]
pub struct Document {
    /// Version of the XML format for share the playlists.
    ///
    /// The latest version is 1,0,0.
    #[serde(rename = "@Version")]
    version: String,
    #[serde(rename = "PRODUCT")]
    product: Product,
    #[serde(rename = "COLLECTION")]
    collection: Collection,
    #[serde(rename = "PLAYLISTS")]
    playlists: Playlists,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Product {
    /// Name of product
    ///
    /// This name will be displayed in each application software.
    #[serde(rename = "@Name")]
    name: String,
    /// Version of application
    #[serde(rename = "@Version")]
    version: String,
    /// Name of company
    #[serde(rename = "@Company")]
    company: String,
}

/// The information of the tracks who are not included in any playlist are unnecessary.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Collection {
    /// Number of TRACK in COLLECTION
    #[serde(rename = "@Entries")]
    entries: i32,
    #[serde(rename = "TRACK")]
    track: Vec<Track>,
}

/// "Location" is essential for each track ;
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Track {
    /// Identification of track
    #[serde(rename = "@TrackID")]
    trackid: i32,
    /// Name of track
    #[serde(rename = "@Name")]
    name: Option<String>,
    /// Name of artist
    #[serde(rename = "@Artist")]
    artist: Option<String>,
    /// Name of composer (or producer)
    #[serde(rename = "@Composer")]
    composer: Option<String>,
    /// Name of Album
    #[serde(rename = "@Album")]
    album: Option<String>,
    /// Name of goupe
    #[serde(rename = "@Grouping")]
    grouping: Option<String>,
    /// Name of genre
    #[serde(rename = "@Genre")]
    genre: Option<String>,
    /// Type of audio file
    #[serde(rename = "@Kind")]
    kind: Option<String>,
    /// Size of audio file
    /// Unit : Octet
    #[serde(rename = "@Size")]
    size: Option<i64>,
    /// Duration of track
    /// Unit : Second (without decimal numbers)
    #[serde(rename = "@TotalTime")]
    totaltime: Option<f64>,
    /// Order number of the disc of the album
    #[serde(rename = "@DiscNumber")]
    discnumber: Option<i32>,
    /// Order number of the track in the album
    #[serde(rename = "@TrackNumber")]
    tracknumber: Option<i32>,
    /// Year of release
    #[serde(rename = "@Year")]
    year: Option<i32>,
    /// Value of average BPM
    /// Unit : Second (with decimal numbers)
    #[serde(rename = "@AverageBpm")]
    averagebpm: Option<f64>,
    /// Date of last modification
    /// Format : yyyy- mm- dd ; ex. : 2010- 08- 21
    #[serde(rename = "@DateModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    datemodified: Option<NaiveDate>,
    /// Date of addition
    /// Format : yyyy- mm- dd ; ex. : 2010- 08- 21
    #[serde(rename = "@DateAdded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dateadded: Option<NaiveDate>,
    /// Encoding bit rate
    /// Unit : Kbps
    #[serde(rename = "@BitRate")]
    bitrate: Option<i32>,
    /// Frequency of sampling
    /// Unit : Hertz
    #[serde(rename = "@SampleRate")]
    samplerate: Option<f64>,
    /// Comments
    #[serde(rename = "@Comments")]
    comments: Option<String>,
    /// Play count of the track
    #[serde(rename = "@PlayCount")]
    playcount: Option<i32>,
    /// Date of last playing
    /// Format : yyyy- mm- dd ; ex. : 2010- 08- 21
    #[serde(rename = "@LastPlayed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    lastplayed: Option<NaiveDate>,
    /// Rating of the track
    /// 0 star = "@0", 1 star = "51", 2 stars = "102", 3 stars = "153", 4 stars = "204", 5 stars = "255"
    #[serde(rename = "@Rating")]
    rating: Option<i32>, // TODO: Use StarRating type here
    /// Location of the file
    /// includes the file name (URI formatted)
    #[serde(rename = "@Location")]
    location: String,
    /// Name of remixer
    #[serde(rename = "@Remixer")]
    remixer: Option<String>,
    /// Tonality (Kind of musical key)
    #[serde(rename = "@Tonality")]
    tonality: Option<String>,
    /// Name of record label
    #[serde(rename = "@Label")]
    label: Option<String>,
    /// Name of mix
    #[serde(rename = "@Mix")]
    mix: Option<String>,
    /// Colour for track grouping
    /// RGB format (3 bytes) ; rekordbox : Rose(0xFF007F), Red(0xFF0000), Orange(0xFFA500), Lemon(0xFFFF00), Green(0x00FF00), Turquoise(0x25FDE9),  Blue(0x0000FF), Violet(0x660099)
    #[serde(rename = "@Colour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    colour: Option<String>,
    #[serde(rename = "TEMPO")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    tempos: Vec<Tempo>,
    #[serde(rename = "POSITION_MARK")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    position_marks: Vec<PositionMark>,
}

/// 0 star = "@0", 1 star = "51", 2 stars = "102", 3 stars = "153", 4 stars = "204", 5 stars = "255"
#[expect(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
enum StarRating {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Unknown(i32),
}

/// For BeatGrid; More than two "TEMPO" can exist for each track
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Tempo {
    /// Start position of BeatGrid
    /// Unit : Second (with decimal numbers)
    #[serde(rename = "@Inizio")]
    inizio: f64,
    /// Value of BPM
    /// Unit : Second (with decimal numbers)
    #[serde(rename = "@Bpm")]
    bpm: f64,
    /// Kind of musical meter (formatted)
    /// ex. 3/ 4, 4/ 4, 7/ 8…
    #[serde(rename = "@Metro")]
    metro: String,
    /// Beat number in the bar
    /// If the value of "Metro" is 4/ 4, the value should be 1, 2, 3 or 4.
    #[serde(rename = "@Battito")]
    battito: i32,
}

/// More than two "POSITION MARK" can exist for each track
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PositionMark {
    /// Name of position mark
    #[serde(rename = "@Name")]
    name: String,
    /// Type of position mark
    /// Cue = "@0", Fade- In = "1", Fade- Out = "2", Load = "3",  Loop = " 4"
    #[serde(rename = "@Type")]
    mark_type: i32,
    /// Start position of position mark
    /// Unit : Second (with decimal numbers)
    #[serde(rename = "@Start")]
    start: f64,
    /// End position of position mark
    /// Unit : Second (with decimal numbers)
    #[serde(rename = "@End")]
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<f64>,
    /// Number for identification of the position mark
    /// rekordbox : Hot Cue A,  B,  C : "0", "1", "2"; Memory Cue : "- 1"
    #[serde(rename = "@Num")]
    num: i32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Playlists {
    #[serde(rename = "NODE")]
    node: PlaylistFolderNode,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
#[serde(tag = "@Type")]
enum PlaylistGenericNode {
    #[serde(rename = "0")]
    Folder(PlaylistFolderNode),
    #[serde(rename = "1")]
    Playlist(PlaylistPlaylistNode),
}

impl<'de> Deserialize<'de> for PlaylistGenericNode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PlaylistGenericNodeVisitor;

        impl<'de> serde::de::Visitor<'de> for PlaylistGenericNodeVisitor {
            type Value = PlaylistGenericNode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct PlaylistGenericNode")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut node_type = None;
                let mut name = None;
                let mut count = None;
                let mut key_type = None;
                let mut entries = None;

                while let Some(key) = map.next_key::<Cow<'_, str>>()? {
                    match key.as_ref() {
                        "@Name" => name = map.next_value::<Cow<'_, str>>()?.into(),
                        "@Type" => node_type = map.next_value::<Cow<'_, str>>()?.into(),
                        "@Count" => count = map.next_value::<usize>()?.into(),
                        "@KeyType" => key_type = map.next_value::<Cow<'_, str>>()?.into(),
                        "@Entries" => entries = map.next_value::<usize>()?.into(),
                        unknown => {
                            return Err(A::Error::unknown_field(
                                unknown,
                                &["@Name", "@Type", "@Count", "@KeyType", "@Entries"],
                            ));
                        }
                    }

                    match node_type.as_deref() {
                        Some("0") => {
                            if let (Some(n), Some(_c)) = (&name, count) {
                                let nodes = {
                                    // Create anonymous type
                                    #[derive(serde::Deserialize)]
                                    struct Nodes {
                                        #[serde(rename = "NODE")]
                                        content: Vec<PlaylistGenericNode>,
                                    }
                                    let de = serde::de::value::MapAccessDeserializer::new(map);
                                    Nodes::deserialize(de)?.content
                                };
                                // FIXME: Should we check if nodes.len() == count here?
                                return Ok(PlaylistGenericNode::Folder(PlaylistFolderNode {
                                    name: n.to_string(),
                                    nodes,
                                }));
                            }
                        }
                        Some("1") => {
                            if let (Some(n), Some(_c), Some(t)) = (&name, entries, &key_type) {
                                let tracks = {
                                    // Create anonymous type
                                    #[derive(serde::Deserialize)]
                                    struct Tracks {
                                        #[serde(rename = "TRACK")]
                                        content: Vec<PlaylistTrack>,
                                    }
                                    let de = serde::de::value::MapAccessDeserializer::new(map);
                                    Tracks::deserialize(de)?.content
                                };
                                // FIXME: Should we check if nodes.len() == count here?
                                return Ok(PlaylistGenericNode::Playlist(PlaylistPlaylistNode {
                                    name: n.to_string(),
                                    keytype: t.to_string(),
                                    tracks,
                                }));
                            }
                        }
                        Some(unknown) => {
                            return Err(A::Error::unknown_variant(unknown, &["0", "1"]))
                        }
                        None => (),
                    }
                }

                match node_type.as_deref() {
                    Some("0") => {
                        if name.is_none() {
                            Err(A::Error::missing_field("@Name"))
                        } else {
                            Err(A::Error::missing_field("@Count"))
                        }
                    }
                    Some("1") => {
                        if name.is_none() {
                            Err(A::Error::missing_field("@Name"))
                        } else if entries.is_none() {
                            Err(A::Error::missing_field("@Entries"))
                        } else {
                            Err(A::Error::missing_field("@KeyType"))
                        }
                    }
                    _ => Err(A::Error::missing_field("@Type")),
                }
            }
        }

        deserializer.deserialize_map(PlaylistGenericNodeVisitor)
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
struct PlaylistFolderNode {
    /// Name of NODE
    #[serde(rename = "@Name")]
    name: String,
    // The "Count" attribute that contains the "Number of NODE in NODE" is omitted here, because we
    // can just take the number of elements in the `tracks` vector instead.
    /// Nodes
    #[serde(rename = "NODE")]
    nodes: Vec<PlaylistGenericNode>,
}

impl Serialize for PlaylistFolderNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Value<'a> {
            /// Name of NODE
            #[serde(rename = "@Name")]
            name: &'a String,
            /// Count
            #[serde(rename = "@Count")]
            count: usize,
            /// Nodes
            #[serde(rename = "NODE")]
            nodes: &'a Vec<PlaylistGenericNode>,
        }

        let value = Value {
            name: &self.name,
            count: self.nodes.len(),
            nodes: &self.nodes,
        };

        value.serialize(serializer)
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
struct PlaylistPlaylistNode {
    /// Name of NODE
    #[serde(rename = "@Name")]
    name: String,
    // The "Entries" attribute that contains the "Number of TRACK in PLAYLIST" is omitted here,
    // because we can just take the number of elements in the `tracks` vector instead.
    /// Kind of identification
    /// "0" (Track ID) or "1"(Location)
    #[serde(rename = "@KeyType")]
    keytype: String,
    #[serde(rename = "TRACK")]
    tracks: Vec<PlaylistTrack>,
}

impl Serialize for PlaylistPlaylistNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Value<'a> {
            /// Name of NODE
            #[serde(rename = "@Name")]
            name: &'a String,
            /// Number of TRACK in PLAYLIST
            #[serde(rename = "@Entries")]
            entries: usize,
            /// Kind of identification
            /// "0" (Track ID) or "1"(Location)
            #[serde(rename = "@KeyType")]
            keytype: &'a String,
            #[serde(rename = "TRACK")]
            tracks: &'a Vec<PlaylistTrack>,
        }

        let value = Value {
            name: &self.name,
            entries: self.tracks.len(),
            keytype: &self.keytype,
            tracks: &self.tracks,
        };

        value.serialize(serializer)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PlaylistTrack {
    /// Identification of track
    /// "Track ID" or "Location" in "COLLECTION"
    #[serde(rename = "@Key")]
    key: i32,
}

/// Convert a PDB file path and Windows base path into a rekordbox file:// URI.
fn make_location_uri(base_path: &str, pdb_file_path: &str) -> String {
    // Normalise base to forward slashes, strip trailing slash
    let base = base_path.replace('\\', "/").trim_end_matches('/').to_string();
    // PDB paths start with '/', e.g. /Contents/Artist/file.mp3
    let rel = pdb_file_path.trim_start_matches('/');
    let full = format!("{}/{}", base, rel);
    // Percent-encode spaces (and a minimal set of unsafe chars)
    let encoded = full
        .chars()
        .flat_map(|c| match c {
            ' ' => vec!['%', '2', '0'],
            '#' => vec!['%', '2', '3'],
            '%' => vec!['%', '2', '5'],
            '?' => vec!['%', '3', 'F'],
            _ => vec![c],
        })
        .collect::<String>();
    format!("file://localhost/{}", encoded)
}

fn pdb_node_to_xml(
    node: crate::device::PlaylistNode,
    pdb: &crate::device::Pdb,
) -> crate::Result<PlaylistGenericNode> {
    use crate::device::PlaylistNode;
    match node {
        PlaylistNode::Folder(folder) => {
            let children = folder
                .children
                .into_iter()
                .map(|child| pdb_node_to_xml(child, pdb))
                .collect::<crate::Result<Vec<_>>>()?;
            Ok(PlaylistGenericNode::Folder(PlaylistFolderNode {
                name: folder.name,
                nodes: children,
            }))
        }
        PlaylistNode::Playlist(playlist) => {
            let mut entries: Vec<(u32, crate::pdb::TrackId)> =
                pdb.get_playlist_entries(playlist.id).collect();
            entries.sort_by_key(|(idx, _)| *idx);
            let tracks = entries
                .into_iter()
                .map(|(_, track_id)| PlaylistTrack {
                    key: track_id.0 as i32,
                })
                .collect();
            Ok(PlaylistGenericNode::Playlist(PlaylistPlaylistNode {
                name: playlist.name,
                keytype: "0".to_string(),
                tracks,
            }))
        }
    }
}

/// Build a rekordbox XML `Document` from a parsed PDB and write it to `writer`.
///
/// `base_path` is the Windows path prefix for tracks, e.g. `O:\PIONEER\ESD USB N Drive`.
pub fn write_from_pdb<W: std::io::Write>(
    pdb: &crate::device::Pdb,
    base_path: &str,
    writer: &mut W,
) -> crate::Result<()> {
    use std::collections::HashMap;

    let track_map: HashMap<_, _> = pdb.get_tracks().map(|t| (t.id, t)).collect();

    let xml_tracks: Vec<Track> = track_map
        .values()
        .filter_map(|t| {
            let file_path = t.offsets.inner.file_path.clone().into_string().ok()?;
            let title = t.offsets.inner.title.clone().into_string().ok()?;
            let location = make_location_uri(base_path, &file_path);

            let opt_str = |s: crate::pdb::string::DeviceSQLString| -> Option<String> {
                s.into_string().ok().filter(|v| !v.is_empty())
            };

            Some(Track {
                trackid: t.id.0 as i32,
                name: Some(title),
                artist: None,
                composer: None,
                album: None,
                grouping: None,
                genre: None,
                kind: None,
                size: Some(t.file_size as i64),
                totaltime: Some(t.duration as f64),
                discnumber: Some(t.disc_number as i32),
                tracknumber: Some(t.track_number as i32),
                year: Some(t.year as i32),
                averagebpm: if t.tempo > 0 {
                    Some(t.tempo as f64 / 100.0)
                } else {
                    None
                },
                datemodified: None,
                dateadded: opt_str(t.offsets.inner.date_added.clone()),
                bitrate: Some(t.bitrate as i32),
                samplerate: Some(t.sample_rate as f64),
                comments: opt_str(t.offsets.inner.comment.clone()),
                playcount: Some(t.play_count as i32),
                lastplayed: None,
                rating: Some(t.rating as i32),
                location,
                remixer: None,
                tonality: None,
                label: None,
                mix: opt_str(t.offsets.inner.mix_name.clone()),
                colour: None,
                tempos: vec![],
                position_marks: vec![],
            })
        })
        .collect();

    let xml_nodes = pdb
        .get_playlists()?
        .into_iter()
        .map(|n| pdb_node_to_xml(n, pdb))
        .collect::<crate::Result<Vec<_>>>()?;

    let doc = Document {
        version: "1.0.0".to_string(),
        product: Product {
            name: "rekordbox".to_string(),
            version: "6.0.0".to_string(),
            company: "AlphaTheta".to_string(),
        },
        collection: Collection {
            entries: xml_tracks.len() as i32,
            track: xml_tracks,
        },
        playlists: Playlists {
            node: PlaylistFolderNode {
                name: "ROOT".to_string(),
                nodes: xml_nodes,
            },
        },
    };

    writer.write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")?;
    let xml_str = quick_xml::se::to_string(&doc)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    writer.write_all(xml_str.as_bytes())?;
    Ok(())
}
