use serde::{Deserialize, Serialize};

use crate::types::{MimeWrapper, PhotoSize};

/// This object represents a general file (as opposed to [photos], [voice
/// messages] and [audio files]).
///
/// [The official docs](https://core.telegram.org/bots/api#document).
///
/// [photos]: https://core.telegram.org/bots/api#photosize
/// [voice messages]: https://core.telegram.org/bots/api#voice
/// [audio files]: https://core.telegram.org/bots/api#audio
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Document {
    /// An identifier for this file.
    pub file_id: String,

    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,

    /// A document thumbnail as defined by a sender.
    pub thumb: Option<PhotoSize>,

    /// An original filename as defined by a sender.
    pub file_name: Option<String>,

    /// A MIME type of the file as defined by a sender.
    pub mime_type: Option<MimeWrapper>,

    /// A size of a file.
    pub file_size: Option<u32>,
}

impl Document {
    pub fn new<S1, S2>(file_id: S1, file_unique_id: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            thumb: None,
            file_name: None,
            mime_type: None,
            file_size: None,
        }
    }

    pub fn file_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.file_id = val.into();
        self
    }

    pub fn file_unique_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.file_unique_id = val.into();
        self
    }

    pub fn thumb(mut self, val: PhotoSize) -> Self {
        self.thumb = Some(val);
        self
    }

    pub fn file_name<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.file_name = Some(val.into());
        self
    }

    pub fn mime_type(mut self, val: MimeWrapper) -> Self {
        self.mime_type = Some(val);
        self
    }

    pub fn file_size(mut self, val: u32) -> Self {
        self.file_size = Some(val);
        self
    }
}
