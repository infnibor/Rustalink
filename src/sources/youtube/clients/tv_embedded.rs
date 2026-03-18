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

const CLIENT_NAME: &str = "TVHTML5_SIMPLY_EMBEDDED_PLAYER";
const CLIENT_ID: &str = "85";
const CLIENT_VERSION: &str = "2.0";
const USER_AGENT: &str = "Mozilla/5.0 (Linux armeabi-v7a; Android 7.1.2; Fire OS 6.0) Cobalt/22.lts.3.306369-gold (unlike Gecko) v8/8.8.278.8-jit gles Starboard/13, Amazon_ATV_mediatek8695_2019/NS6294 (Amazon, AFTMM, Wireless) com.amazon.firetv.youtube/22.3.r2.v66.0";

pub struct TvEmbeddedClient {
    http: Arc<reqwest::Client>,
}

impl TvEmbeddedClient {
    pub fn new(http: Arc<reqwest::Client>) -> Self {
        Self { http }
    }

    fn config(&self) -> ClientConfig<'static> {
        ClientConfig {
            client_name: CLIENT_NAME,
            client_version: CLIENT_VERSION,
            client_id: CLIENT_ID,
            user_agent: USER_AGENT,
            third_party_embed_url: Some("https://www.youtube.com/tv"),
            ..Default::default()
        }
    }
}

#[async_trait]
impl YouTubeClient for TvEmbeddedClient {
    fn name(&self) -> &str {
        "TvEmbedded"
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

    fn supports_oauth(&self) -> bool {
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
        core::standard_get_playlist(self, &self.http, playlist_id, context, oauth, || {
            self.config()
        })
        .await
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
        oauth: Arc<YouTubeOAuth>,
    ) -> Option<serde_json::Value> {
        crate::sources::youtube::clients::common::make_player_request(
            crate::sources::youtube::clients::common::PlayerRequestOptions {
                http: &self.http,
                config: &self.config(),
                video_id: track_id,
                params: Some("2AMB"),
                visitor_data,
                signature_timestamp: None,
                auth_header: oauth.get_auth_header().await,
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
