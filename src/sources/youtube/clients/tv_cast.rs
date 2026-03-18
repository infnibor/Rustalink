use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;

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

const CLIENT_NAME_OVERRIDE: &str = "TVHTML5_CAST";
const CLIENT_VERSION: &str = "7.20190924";
const USER_AGENT: &str = "Mozilla/5.0 (Linux; Android) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36 CrKey/1.54.248666";

pub struct TvCastClient {
    http: Arc<reqwest::Client>,
}

impl TvCastClient {
    pub fn new(http: Arc<reqwest::Client>) -> Self {
        Self { http }
    }

    fn config(&self) -> ClientConfig<'static> {
        ClientConfig {
            client_name: CLIENT_NAME_OVERRIDE,
            client_version: CLIENT_VERSION,
            client_id: "7",
            user_agent: USER_AGENT,
            ..Default::default()
        }
    }
}

#[async_trait]
impl YouTubeClient for TvCastClient {
    fn name(&self) -> &str {
        "TV Cast"
    }

    fn client_name(&self) -> &str {
        CLIENT_NAME_OVERRIDE
    }

    fn client_version(&self) -> &str {
        CLIENT_VERSION
    }

    fn user_agent(&self) -> &str {
        USER_AGENT
    }

    fn can_handle_request(&self, _identifier: &str) -> bool {
        false
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
        core::standard_get_track_url(
            self,
            core::StandardUrlOptions {
                http: &self.http,
                track_id,
                context,
                cipher_manager,
                oauth,
                signature_timestamp,
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
