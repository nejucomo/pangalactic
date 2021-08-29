use crate::store::PgLink;

#[derive(Debug)]
pub struct LinkArg {
    pub link: PgLink,
}

impl std::str::FromStr for LinkArg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use pangalactic_codec::{decode_string, DecodeStringError::*};

        let link = decode_string(s).map_err(|e| match e {
            Base64(_) => String::from("Malformed base64 encoding."),
            Bytes(_) => String::from("Malformed byte encoding."),
        })?;
        Ok(LinkArg { link })
    }
}

impl ToString for LinkArg {
    fn to_string(&self) -> String {
        pangalactic_codec::encode_string(&self.link)
    }
}
