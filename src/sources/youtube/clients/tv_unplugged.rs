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

const CLIENT_NAME: &str = "TVHTML5_UNPLUGGED";
const CLIENT_VERSION: &str = "6.13";
const USER_AGENT: &str = "Mozilla/5.0 (Linux armeabi-v7a; Android 7.1.2; Fire OS 6.0) Cobalt/22.lts.3.306369-gold (unlike Gecko) v8/8.8.278.8-jit gles Starboard/13, Amazon_ATV_mediatek8695_2019/NS6294 (Amazon, AFTMM, Wireless) com.amazon.firetv.youtube/22.3.r2.v66.0";

pub struct TvUnpluggedClient {
    http: Arc<reqwest::Client>,
}

impl TvUnpluggedClient {
    pub fn new(http: Arc<reqwest::Client>) -> Self {
        Self { http }
    }

    fn config(&self) -> ClientConfig<'static> {
        ClientConfig {
            client_name: CLIENT_NAME,
            client_version: CLIENT_VERSION,
            user_agent: USER_AGENT,
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
impl YouTubeClient for TvUnpluggedClient {
    fn name(&self) -> &str {
        "TvUnplugged"
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

    fn can_handle_request(&self, identifier: &str) -> bool {
        if identifier.contains("list=") && !identifier.contains("list=RD") {
            return false;
        }
        true
    }

    async fn search(
        &self,
        _query: &str,
        _context: &Value,
        _oauth: Arc<YouTubeOAuth>,
    ) -> AnyResult<Vec<Track>> {
        Err("TvUnplugged client does not support search".into())
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
                config_builder: || {
                    let mut cfg = self.config();
                    cfg.client_screen = Some("EMBED");
                    cfg
                },
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
                config_builder: || {
                    let mut cfg = self.config();
                    cfg.client_screen = Some("EMBED");
                    cfg
                },
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
                config: &{
                    let mut cfg = self.config();
                    cfg.client_screen = Some("EMBED");
                    cfg
                },
                video_id: track_id,
                params: Some("2AMB"),
                visitor_data,
                signature_timestamp: None,
                auth_header: None,
                referer: None,
                origin: Some("https://www.youtube.com/"),
                po_token: None,
                encrypted_host_flags,
                attestation_request: None,
                serialized_third_party_embed_config: false,
            },
        )
        .await
        .ok()
    }
}
