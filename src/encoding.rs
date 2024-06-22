use std::{io::Write, str::FromStr};

use flate2::{write::GzEncoder, Compression};

use crate::error::ServerError;

pub const ACCEPTED_ENCODINGS: [&str; 1] = ["gzip"];

pub enum AvailableEncodings {
    GZIP,
}

impl FromStr for AvailableEncodings {
    type Err = ServerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gzip" => Ok(AvailableEncodings::GZIP),
            _ => Err(ServerError::IncorrectEncodingError),
        }
    }
}

pub fn encode(data: &str, encoding: AvailableEncodings) -> Result<Vec<u8>, ServerError> {
    match encoding {
        AvailableEncodings::GZIP => {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder
                .write_all(data.as_bytes())
                .map_err(|_| ServerError::EncodingError)?;
            Ok(encoder.finish().map_err(|_| ServerError::EncodingError)?)
        }
    }
}
