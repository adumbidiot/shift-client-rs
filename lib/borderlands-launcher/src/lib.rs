/// Generated protobuf structs
mod protos {
    include!("generated/launcher.rs");
}

pub use self::protos::*;
use async_compression::tokio::bufread::GzipDecoder;
use bytes::BytesMut;
use futures_util::TryStreamExt;
use prost::Message;
use tokio::io::AsyncReadExt;
use tokio_util::io::StreamReader;

/// Library error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Reqwest Error
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    /// I/O Error
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Protobuf decode error
    #[error(transparent)]
    ProstDecode(#[from] prost::DecodeError),
}

/// The game type.
///
/// Borderlands 3 does not follow this protocol, so it is not included.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Game {
    Borderlands,
    Borderlands2,
    BorderlandsPresequel,
}

/// An API client for the borderlands launcher apis
#[derive(Debug, Clone)]
pub struct Client {
    /// The http client
    pub client: reqwest::Client,
}

impl Client {
    /// Make a new client
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Get launcher content.
    ///
    /// # The `lang_code` parameter
    /// `lang_code` must be lowercase.
    ///
    /// Valid options are:
    /// * "int" for english. This is the default.
    /// * "deu" for germany
    /// * "esn" for spanish
    /// * "fra" for france
    /// * "ita" for italy
    /// * "jpn" for japan
    ///
    ///
    /// Specifically for Borderlands 2,
    /// * "twn" for chinese. Note that the chinese code, "cht", is mapped to "twn", or taiwan. Cute.
    /// * "kor" for korean
    ///
    /// Specifically for Borderlands Presequel,
    /// * "rus" for russian
    /// * "kor" for korean
    ///
    /// # The `is_qa` parameter
    /// This argument with toggle whether a request will be made to a different url, presumable the Q/A server.
    /// This server is not active or accessible, so this parameter should be false.
    /// This parameter has no effect for Borderlands.
    ///
    /// # Supported Games
    /// Currently, only the Borderlands 2 game api still is functional.
    pub async fn get_launcher_content(
        &self,
        game: Game,
        is_qa: bool,
        lang_code: &str,
    ) -> Result<AvailableData, Error> {
        let qa_str = if is_qa { "-qa" } else { "" };

        let url = match game {
            Game::Borderlands => {
                format!("http://gbx.parnic.com/launcher/LauncherContent.{lang_code}.wpb")
            }
            Game::Borderlands2 => {
                format!("https://cdn{qa_str}.services.gearboxsoftware.com/sparktms/willow2/pc/steam/launcher/LauncherContent.{lang_code}.wpb")
            }
            Game::BorderlandsPresequel => {
                format!("http://cdn{qa_str}.services.gearboxsoftware.com/sparktms/cork/pc/steam/launcher/LauncherContent.{lang_code}.wpb")
            }
        };

        let response = self.client.get(url).send().await?.error_for_status()?;
        let stream = response
            .bytes_stream()
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error));
        let reader = StreamReader::new(stream);
        let mut decoder = GzipDecoder::new(reader);

        let mut buffer = BytesMut::new();
        while decoder.read_buf(&mut buffer).await? != 0 {}

        Ok(AvailableData::decode(buffer)?)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const LANG_CODES: &[&str] = &["int", "deu", "esn", "ita", "jpn", "kor", "twn", "rus"];

    /// Appears to have been taken down
    #[tokio::test]
    #[ignore]
    async fn get_launcher_content_bl() {
        let client = Client::new();
        for lang_code in LANG_CODES {
            dbg!(&lang_code);
            let available_data = client
                .get_launcher_content(Game::Borderlands, false, lang_code)
                .await
                .expect("failed to get launcher content");

            dbg!(available_data);
        }
    }

    #[tokio::test]
    async fn get_launcher_content_bl2() {
        let client = Client::new();

        for lang_code in LANG_CODES {
            dbg!(&lang_code);

            if *lang_code == "rus" {
                continue;
            }

            let available_data = client
                .get_launcher_content(Game::Borderlands2, false, lang_code)
                .await
                .expect("failed to get launcher content");
            dbg!(available_data);
        }
    }

    /// Appears to have been taken down
    #[tokio::test]
    #[ignore]
    async fn get_launcher_content_blps() {
        let client = Client::new();
        for lang_code in LANG_CODES {
            dbg!(&lang_code);
            let available_data = client
                .get_launcher_content(Game::BorderlandsPresequel, false, lang_code)
                .await
                .expect("failed to get launcher content");

            dbg!(available_data);
        }
    }
}
