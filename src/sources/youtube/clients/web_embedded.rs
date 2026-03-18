use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;

use super::{YouTubeClient, core};
use crate::{
    common::types::AnyResult,
    protocol::tracks::Track,
    sources::youtube::{
        cipher::YouTubeCipherManager, clients::common::ClientConfig, oauth::YouTubeOAuth,
    },
};

const CLIENT_NAME: &str = "WEB_EMBEDDED_PLAYER";
const CLIENT_ID: &str = "56";
const CLIENT_VERSION: &str = "1.20260128.01.00";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36,gzip(gfe)";

pub struct WebEmbeddedClient {
    http: Arc<reqwest::Client>,
}

impl WebEmbeddedClient {
    pub fn new(http: Arc<reqwest::Client>) -> Self {
        Self { http }
    }

    fn config(&self) -> ClientConfig<'static> {
        ClientConfig {
            client_name: CLIENT_NAME,
            client_version: CLIENT_VERSION,
            client_id: CLIENT_ID,
            user_agent: USER_AGENT,
            platform: Some("DESKTOP"),
            third_party_embed_url: Some("https://www.google.com/"),
            ..Default::default()
        }
    }

    async fn fetch_encrypted_host_flags(&self, video_id: &str) -> Option<String> {
        let url = format!("https://www.youtube.com/embed/{}", video_id);
        let res = self
            .http
            .get(&url)
            .header("Referer", "https://www.google.com")
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            )
            .send()
            .await
            .ok()?;

        if !res.status().is_success() {
            return None;
        }

        let body = res.text().await.ok()?;
        let re = regex::Regex::new(r#""encryptedHostFlags":"([^"]+)""#).ok()?;
        re.captures(&body)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
    }
}

#[async_trait]
impl YouTubeClient for WebEmbeddedClient {
    fn name(&self) -> &str {
        "WebEmbedded"
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

    fn is_embedded(&self) -> bool {
        true
    }

    fn can_handle_request(&self, identifier: &str) -> bool {
        !identifier.contains("list=")
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
        _track_id: &str,
        _context: &Value,
        _oauth: Arc<YouTubeOAuth>,
    ) -> AnyResult<Option<Track>> {
        Ok(None)
    }

    async fn get_playlist(
        &self,
        _playlist_id: &str,
        _context: &Value,
        _oauth: Arc<YouTubeOAuth>,
    ) -> AnyResult<Option<(Vec<Track>, String)>> {
        Ok(None)
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
                params: None,
                visitor_data,
                signature_timestamp: None,
                auth_header: None,
                referer: Some("https://www.youtube.com"),
                origin: None,
                po_token: None,
                encrypted_host_flags,
                attestation_request: None,
                serialized_third_party_embed_config: true,
            },
        )
        .await
        .ok()
    }
}
