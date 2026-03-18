use std::sync::Arc;

use async_trait::async_trait;
use serde_json::{Value, json};

use super::{YouTubeClient, core};
use crate::{
    common::types::AnyResult,
    protocol::tracks::Track,
    sources::youtube::{
        cipher::YouTubeCipherManager,
        clients::common::ClientConfig,
        oauth::YouTubeOAuth,
    },
};

const CLIENT_NAME: &str = "TVHTML5_SIMPLY";
const CLIENT_VERSION: &str = "1.0";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36";

pub struct TvSimplyClient {
    http: Arc<reqwest::Client>,
}

impl TvSimplyClient {
    pub fn new(http: Arc<reqwest::Client>) -> Self {
        Self {
            http,
        }
    }

    fn config(&self) -> ClientConfig<'static> {
        ClientConfig {
            client_name: CLIENT_NAME,
            client_version: CLIENT_VERSION,
            client_id: "TVHTML5_SIMPLY",
            user_agent: USER_AGENT,
            attestation_request: Some(json!({ "omitBotguardData": true })),
            ..Default::default()
        }
    }

    async fn fetch_encrypted_host_flags(&self, video_id: &str) -> Option<String> {
        let url = format!("https://www.youtube.com/embed/{}", video_id);
        let res = self
            .http
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .send()
            .await
            .ok()?;

        let html = res.text().await.ok()?;
        let re = regex::Regex::new(r#""encryptedHostFlags":"([^"]+)""#).ok()?;
        re.captures(&html).map(|caps| caps[1].to_string())
    }
}

#[async_trait]
impl YouTubeClient for TvSimplyClient {
    fn name(&self) -> &str {
        "TvSimply"
    }

    fn client_name(&self) -> &str {
        CLIENT_NAME
    }

    fn client_version(&self) -> &str {
        CLIENT_VERSION
    }

    fn user_agent(&self) -> &str {
        USER_AGENT
    }

    async fn search(
        &self,
        query: &str,
        context: &Value,
        oauth: Arc<YouTubeOAuth>,
    ) -> AnyResult<Vec<Track>> {
        core::standard_search(self, &self.http, query, context, oauth, || self.config()).await
    }

    async fn get_track_info(
        &self,
        track_id: &str,
        context: &Value,
        oauth: Arc<YouTubeOAuth>,
    ) -> AnyResult<Option<Track>> {
        let encrypted_host_flags = self.fetch_encrypted_host_flags(track_id).await;
        core::standard_get_track_info(
            self,
            core::StandardPlayerOptions {
                http: &self.http,
                track_id,
                context,
                oauth,
                signature_timestamp: None,
                encrypted_host_flags,
                config_builder: || self.config(),
            },
        )
        .await
    }

    async fn get_playlist(
        &self,
        playlist_id: &str,
        context: &Value,
        oauth: Arc<YouTubeOAuth>,
    ) -> AnyResult<Option<(Vec<Track>, String)>> {
        core::standard_get_playlist(self, &self.http, playlist_id, context, oauth, || self.config()).await
    }

    async fn resolve_url(
        &self,
        _url: &str,
        _context: &Value,
        _oauth: Arc<YouTubeOAuth>,
    ) -> AnyResult<Option<Track>> {
        Ok(None)
    }

    async fn get_track_url(
        &self,
        track_id: &str,
        context: &Value,
        cipher_manager: Arc<YouTubeCipherManager>,
        oauth: Arc<YouTubeOAuth>,
    ) -> AnyResult<Option<String>> {
        let signature_timestamp = cipher_manager.get_signature_timestamp().await.ok();
        let encrypted_host_flags = self.fetch_encrypted_host_flags(track_id).await;
        core::standard_get_track_url(
            self,
            core::StandardUrlOptions {
                http: &self.http,
                track_id,
                context,
                cipher_manager,
                oauth,
                signature_timestamp,
                encrypted_host_flags,
                config_builder: || self.config(),
            },
        )
        .await
    }

    async fn get_player_body(
        &self,
        track_id: &str,
        visitor_data: Option<&str>,
        _oauth: Arc<YouTubeOAuth>,
    ) -> Option<serde_json::Value> {
        let encrypted_host_flags = self.fetch_encrypted_host_flags(track_id).await;
        crate::sources::youtube::clients::common::make_player_request(
            crate::sources::youtube::clients::common::PlayerRequestOptions {
                http: &self.http,
                config: &self.config(),
                video_id: track_id,
                params: Some("2AMB"),
                visitor_data,
                signature_timestamp: None,
                auth_header: None,
                referer: None,
                origin: Some("https://www.youtube.com"),
                po_token: None,
                encrypted_host_flags,
                attestation_request: Some(json!({ "omitBotguardData": true })),
                serialized_third_party_embed_config: false,
            },
        )
        .await
        .ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::sources::YouTubeCipherConfig, sources::youtube::cipher::YouTubeCipherManager,
    };

    #[tokio::test]
    async fn test_search() {
        let http = Arc::new(reqwest::Client::new());
        let _cipher = Arc::new(YouTubeCipherManager::new(YouTubeCipherConfig::default()));
        let client = TvSimplyClient::new(http);
        let oauth = Arc::new(YouTubeOAuth::new(vec![]));

        let result = client.search("test", &json!({}), oauth).await.unwrap();
        assert!(!result.is_empty(), "Search should return tracks");
    }

    #[tokio::test]
    async fn test_playlist() {
        let http = Arc::new(reqwest::Client::new());
        let _cipher = Arc::new(YouTubeCipherManager::new(YouTubeCipherConfig::default()));
        let client = TvSimplyClient::new(http);
        let oauth = Arc::new(YouTubeOAuth::new(vec![]));

        // Use a known playlist ID
        let result = client
            .get_playlist("PLFsQleAWXsj_4yDeebiIADdH5FMayBiJo", &json!({}), oauth)
            .await
            .unwrap();
        assert!(result.is_some(), "Playlist should return tracks");
        assert!(
            !result.unwrap().0.is_empty(),
            "Playlist should not be empty"
        );
    }
}

#[cfg(test)]
mod get_track_tests {
    use super::*;
    use crate::{
        config::sources::YouTubeCipherConfig, sources::youtube::cipher::YouTubeCipherManager,
    };

    #[tokio::test]
    async fn test_get_track_url() {
        let http = Arc::new(reqwest::Client::new());
        let _cipher = Arc::new(YouTubeCipherManager::new(YouTubeCipherConfig::default()));
        let client = TvSimplyClient::new(http);
        //let oauth = Arc::new(YouTubeOAuth::new(vec![]));

        let body = client
            .get_player_body("3Z_x7vBqr6E", None, Arc::new(YouTubeOAuth::new(vec![])))
            .await;
        assert!(body.is_some());
        println!("Body: {}", serde_json::to_string_pretty(&body.unwrap()).unwrap());
    }
}
