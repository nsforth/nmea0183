use heapless::String;

use crate::{common, Source};

/// Geographic latitude ang longitude sentence with time of fix and receiver state.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TXT {
    /// Navigational system.
    pub source: Source,
    /// Total number of sentences
    pub sentences: u8,
    /// Sentence number
    pub number: u8,
    /// Message type
    pub type_: MessageType,
    /// The message content
    pub text: String<69>,
}

impl TXT {
    pub(crate) fn parse<'a>(
        source: Source,
        fields: &mut core::str::Split<'a, char>,
    ) -> Result<Option<Self>, &'static str> {
        let mut fields = fields.peekable();
        let sentences = common::parse_u8(fields.next())?;
        let number = common::parse_u8(fields.next())?;
        let type_ = MessageType::parse(fields.next())?;
        let mut text = String::<69>::new();

        while let Some(part) = fields.next() {
            text.push_str(part).map_err(|_| "exceeded string size")?;

            if fields.peek().is_some() {
                text.push_str(",").map_err(|_| "exceeded string size")?;
            }
        }

        if let (Some(sentences), Some(number), Some(type_)) = (sentences, number, type_) {
            Ok(Some(TXT { source, sentences, number, type_, text }))
        }
        else {
            Ok(None)
        }
    }
}

/// The type of message
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MessageType {
    /// An error message
    Error,
    /// A warning message
    Warning,
    /// An informational message or notice
    Notice,
    /// An application specific message
    User,
}

impl MessageType {
    pub(crate) fn parse(input: Option<&str>) -> Result<Option<MessageType>, &'static str> {
        match input {
            Some("00") => Ok(Some(MessageType::Error)),
            Some("01") => Ok(Some(MessageType::Warning)),
            Some("02") => Ok(Some(MessageType::Notice)),
            Some("07") => Ok(Some(MessageType::User)),
            Some("") => Ok(None),
            None => Ok(None),
            _ => Err("Unsupported message kind!"),
        }
    }
}
