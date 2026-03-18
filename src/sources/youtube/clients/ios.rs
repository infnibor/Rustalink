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

const CLIENT_NAME: &str = "IOS";
const CLIENT_VERSION: &str = "21.02.1";
const USER_AGENT: &str =
    "com.google.ios.youtube/21.02.1 (iPhone16,2; U; CPU iOS 18_2 like Mac OS X;)";

pub struct IosClient {
    http: Arc<reqwest::Client>,
}

impl IosClient {
    pub fn new(http: Arc<reqwest::Client>) -> Self {
        Self { http }
    }

    fn config(&self) -> ClientConfig<'static> {
        ClientConfig {
            client_name: CLIENT_NAME,
            client_version: CLIENT_VERSION,
            client_id: "5",
            user_agent: USER_AGENT,
            device_make: Some("Apple"),
            device_model: Some("iPhone16,2"),
            os_name: Some("iPhone"),
            os_version: Some("18.2.22C152"),
            utc_offset_minutes: Some(0),
            ..Default::default()
        }
    }
}

#[async_trait]
impl YouTubeClient for IosClient {
    fn name(&self) -> &str {
        "IOS"
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
        core::standard_get_track_info(
            self,
            core::StandardPlayerOptions {
                http: &self.http,
                track_id,
                context,
                oauth,
                signature_timestamp: None,
                encrypted_host_flags: None,
                config_builder: || self.config(),
            },
        )
        .await
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
        core::standard_get_track_url(
            self,
            core::StandardUrlOptions {
                http: &self.http,
                track_id,
                context,
                cipher_manager,
                oauth,
                signature_timestamp: None,
                encrypted_host_flags: None,
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
        crate::sources::youtube::clients::common::make_player_request(
            crate::sources::youtube::clients::common::PlayerRequestOptions {
                http: &self.http,
                config: &self.config(),
                video_id: track_id,
                params: None,
                visitor_data,
                signature_timestamp: None,
                auth_header: None,
                referer: None,
                origin: None,
                po_token: None,
                encrypted_host_flags: None,
                attestation_request: None,
                serialized_third_party_embed_config: false,
            },
        )
        .await
        .ok()
    }
}
