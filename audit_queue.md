# 🦀 Rust Audit Queue

> **Total: 181 files** | Statuses: `[ ]` pending · `[✓]` done · `[!]` critical · `[~]` skipped · `[🔄]` re-auditing

---

## 📦 audio/ — 50 files

| # | Status | File |
|---|--------|------|
| 1 | [✓] High:2 Med:2 | `src/audio/buffer/mod.rs` |
| 2 | [✓] High:1 Med:2 Low:2 | `src/audio/buffer/pool.rs` |
| 3 | [✓] High:1 Med:2 | `src/audio/buffer/ring.rs` |
| 4 | [✓] clean | `src/audio/codec/mod.rs` |
| 5 | [✓] Med:2 Low:2 | `src/audio/codec/opus_decoder.rs` |
| 6 | [✓] clean | `src/audio/codec/opus_encoder.rs` |
| 7 | [✓] Low:3 | `src/audio/constants.rs` |
| 8 | [✓] Med:1 | `src/audio/demux/format.rs` |
| 9 | [✓] Low:1 | `src/audio/demux/mod.rs` |
| 10 | [✓] Med:1 Low:1 | `src/audio/demux/webm_opus.rs` |
| 11 | [✓] Med:1 Low:2 | `src/audio/effects/crossfade.rs` |
| 12 | [✓] Low:2 | `src/audio/effects/fade.rs` |
| 13 | [✓] Med:1 Low:1 | `src/audio/effects/mod.rs` |
| 14 | [✓] High:1 Med:1 Low:2 | `src/audio/effects/tape.rs` |
| 15 | [✓] Med:1 Low:3 | `src/audio/effects/volume.rs` |
| 16 | [✓] clean | `src/audio/engine/encoder.rs` |
| 17 | [✓] clean | `src/audio/engine/mod.rs` |
| 18 | [✓] clean | `src/audio/engine/standard.rs` |
| 19 | [✓] clean | `src/audio/error.rs` |
| 20 | [✓] Low:1 | `src/audio/filters/biquad.rs` |
| 21 | [✓] Med:1 Low:1 | `src/audio/filters/channel_mix.rs` |
| 22 | [✓] Low:2 | `src/audio/filters/chorus.rs` |
| 23 | [✓] Med:1 | `src/audio/filters/compressor.rs` |
| 24 | [✓] clean | `src/audio/filters/delay_line.rs` |
| 25 | [✓] Med:1 Low:1 | `src/audio/filters/distortion.rs` |
| 26 | [✓] Low:1 | `src/audio/filters/echo.rs` |
| 27 | [✓] clean | `src/audio/filters/equalizer.rs` |
| 28 | [✓] Low:1 | `src/audio/filters/flanger.rs` |
| 29 | [✓] Low:2 | `src/audio/filters/high_pass.rs` |
| 30 | [✓] Med:1 Low:1 | `src/audio/filters/karaoke.rs` |
| 31 | [✓] Low:1 | `src/audio/filters/lfo.rs` |
| 32 | [✓] Low:1 | `src/audio/filters/low_pass.rs` |
| 33 | [✓] clean | `src/audio/filters/mod.rs` |
| 34 | [✓] Low:1 | `src/audio/filters/normalization.rs` |
| 35 | [✓] Low:1 | `src/audio/filters/phaser.rs` |
| 36 | [✓] Low:1 | `src/audio/filters/phonograph.rs` |
| 37 | [✓] Low:1 | `src/audio/filters/reverb.rs` |
| 38 | [✓] Low:1 | `src/audio/filters/rotation.rs` |
| 39 | [✓] Low:1 | `src/audio/filters/spatial.rs` |
| 40 | [✓] Low:1 | `src/audio/filters/timescale.rs` |
| 41 | [✓] clean | `src/audio/filters/tremolo.rs` |
| 42 | [✓] Low:1 | `src/audio/filters/vibrato.rs` |
| 43 | [✓] Low:1 | `src/audio/filters/volume.rs` |
| 44 | [✓] clean | `src/audio/flow/controller.rs` |
| 45 | [✓] clean | `src/audio/flow/mod.rs` |
| 46 | [✓] clean | `src/audio/frame.rs` |
| 47 | [✓] Low:1 | `src/audio/mix/layer.rs` |
| 48 | [✓] clean | `src/audio/mix/mixer.rs` |
| 49 | [✓] clean | `src/audio/mix/mod.rs` |
| 50 | [✓] clean | `src/audio/mod.rs` |

---

## 📦 common/ — 9 files

| # | Status | File |
|---|--------|------|
| 51 | [✓] clean | `src/common/banner.rs` |
| 52 | [✓] clean | `src/common/errors.rs` |
| 53 | [✓] clean | `src/common/http.rs` |
| 54 | [✓] Low:1 | `src/common/logger/formatter.rs` |
| 55 | [✓] clean | `src/common/logger/mod.rs` |
| 56 | [✓] clean | `src/common/logger/writer.rs` |
| 57 | [✓] clean | `src/common/mod.rs` |
| 58 | [✓] clean | `src/common/types.rs` |
| 59 | [✓] Low:1 | `src/common/utils.rs` |

---

## 📦 config/ — 34 files

| # | Status | File |
|---|--------|------|
| 60 | [✓] clean | `src/config/filters.rs` |
| 61 | [✓] clean | `src/config/lyrics.rs` |
| 62 | [✓] clean | `src/config/metrics.rs` |
| 63 | [✓] clean | `src/config/mod.rs` |
| 64 | [✓] clean | `src/config/player.rs` |
| 65 | [✓] clean | `src/config/server.rs` |
| 66 | [✓] clean | `src/config/sources/amazonmusic.rs` |
| 67 | [✓] clean | `src/config/sources/anghami.rs` |
| 68 | [✓] clean | `src/config/sources/applemusic.rs` |
| 69 | [✓] clean | `src/config/sources/audiomack.rs` |
| 70 | [✓] clean | `src/config/sources/audius.rs` |
| 71 | [✓] clean | `src/config/sources/bandcamp.rs` |
| 72 | [✓] clean | `src/config/sources/deezer.rs` |
| 73 | [✓] clean | `src/config/sources/flowery.rs` |
| 74 | [✓] clean | `src/config/sources/gaana.rs` |
| 75 | [✓] clean | `src/config/sources/google_tts.rs` |
| 76 | [✓] clean | `src/config/sources/http.rs` |
| 77 | [✓] clean | `src/config/sources/jiosaavn.rs` |
| 78 | [✓] clean | `src/config/sources/lastfm.rs` |
| 79 | [✓] clean | `src/config/sources/local.rs` |
| 80 | [✓] clean | `src/config/sources/mixcloud.rs` |
| 81 | [✓] clean | `src/config/sources/mod.rs` |
| 82 | [✓] clean | `src/config/sources/netease.rs` |
| 83 | [✓] clean | `src/config/sources/pandora.rs` |
| 84 | [✓] clean | `src/config/sources/qobuz.rs` |
| 85 | [✓] clean | `src/config/sources/reddit.rs` |
| 86 | [✓] clean | `src/config/sources/shazam.rs` |
| 87 | [✓] clean | `src/config/sources/soundcloud.rs` |
| 88 | [✓] clean | `src/config/sources/spotify.rs` |
| 89 | [✓] clean | `src/config/sources/tidal.rs` |
| 90 | [✓] clean | `src/config/sources/twitch.rs` |
| 91 | [✓] clean | `src/config/sources/vkmusic.rs` |
| 92 | [✓] clean | `src/config/sources/yandexmusic.rs` |
| 93 | [✓] clean | `src/config/sources/youtube.rs` |

---

## 📦 gateway/ — 13 files

| # | Status | File |
|---|--------|------|
| 94 | [✓] clean | `src/gateway/constants.rs` |
| 95 | [✓] Low:2 | `src/gateway/encryption.rs` |
| 96 | [✓] clean | `src/gateway/engine.rs` |
| 97 | [✓] clean | `src/gateway/mod.rs` |
| 98  | [✓] clean | `src/gateway/session/backoff.rs` |
| 99  | [✓] Low:2 | `src/gateway/session/handler.rs` |
| 100 | [✓] clean | `src/gateway/session/heartbeat.rs` |
| 101 | [✓] Low:1 | `src/gateway/session/mod.rs` |
| 102 | [✓] clean | `src/gateway/session/policy.rs` |
| 103 | [✓] clean | `src/gateway/session/protocol.rs` |
| 104 | [✓] clean | `src/gateway/session/types.rs` |
| 105 | [✓] Low:1 | `src/gateway/session/voice.rs` |
| 106 | [✓] clean | `src/gateway/udp_link.rs` |

---

## 📦 lyrics/ — 10 files

| # | Status | File |
|---|--------|------|
| 107 | [✓] Low:3  | `src/lyrics/deezer.rs` |
| 108 | [✓] Low:2  | `src/lyrics/genius.rs` |
| 109 | [✓] Low:1  | `src/lyrics/letrasmus.rs` |
| 110 | [✓] clean  | `src/lyrics/lrclib.rs` |
| 111 | [✓] clean  | `src/lyrics/mod.rs` |
| 112 | [✓] clean  | `src/lyrics/musixmatch.rs` |
| 113 | [✓] clean  | `src/lyrics/netease.rs` |
| 114 | [✓] Med:1  | `src/lyrics/utils.rs` |
| 115 | [✓] Low:1  | `src/lyrics/yandex.rs` |
| 116 | [✓] Low:1  | `src/lyrics/youtubemusic.rs` |

---

## 📦 monitoring/ — 3 files

| # | Status | File |
|---|--------|------|
| 117 | [✓] clean | `src/monitoring/mod.rs` |
| 118 | [✓] clean | `src/monitoring/prometheus.rs` |
| 119 | [✓] clean | `src/monitoring/stats_collector.rs` |

---

## 📦 player/ — 8 files

| # | Status | File |
|---|--------|------|
| 120 | [✓] clean | `src/player/context.rs` |
| 121 | [✓] Low:1 | `src/player/manager/error.rs` |
| 122 | [✓] clean | `src/player/manager/lyrics.rs` |
| 123 | [✓] clean | `src/player/manager/mod.rs` |
| 124 | [✓] Low:1 | `src/player/manager/monitor.rs` |
| 125 | [✓] clean | `src/player/manager/start.rs` |
| 126 | [✓] clean | `src/player/mod.rs` |
| 127 | [✓] clean | `src/player/state.rs` |

---

## 📦 protocol/ — 13 files

| # | Status | File |
|---|--------|------|
| 128 | [ ] | `src/protocol/codec/decode.rs` |
| 129 | [ ] | `src/protocol/codec/encode.rs` |
| 130 | [ ] | `src/protocol/codec/io.rs` |
| 131 | [ ] | `src/protocol/codec/mod.rs` |
| 132 | [ ] | `src/protocol/events.rs` |
| 133 | [ ] | `src/protocol/info.rs` |
| 134 | [ ] | `src/protocol/mod.rs` |
| 135 | [ ] | `src/protocol/models.rs` |
| 136 | [ ] | `src/protocol/opcodes.rs` |
| 137 | [ ] | `src/protocol/routeplanner.rs` |
| 138 | [ ] | `src/protocol/session.rs` |
| 139 | [ ] | `src/protocol/stats.rs` |
| 140 | [ ] | `src/protocol/tracks.rs` |

---

## 📦 rest/ — 13 files

| # | Status | File |
|---|--------|------|
| 141 | [ ] | `src/rest/middleware.rs` |
| 142 | [ ] | `src/rest/mod.rs` |
| 143 | [ ] | `src/rest/routes/lyrics.rs` |
| 144 | [ ] | `src/rest/routes/mod.rs` |
| 145 | [ ] | `src/rest/routes/player/destroy.rs` |
| 146 | [ ] | `src/rest/routes/player/get.rs` |
| 147 | [ ] | `src/rest/routes/player/mod.rs` |
| 148 | [ ] | `src/rest/routes/player/update.rs` |
| 149 | [ ] | `src/rest/routes/stats/info.rs` |
| 150 | [ ] | `src/rest/routes/stats/mod.rs` |
| 151 | [ ] | `src/rest/routes/stats/routeplanner.rs` |
| 152 | [ ] | `src/rest/routes/stats/track.rs` |
| 153 | [ ] | `src/rest/routes/youtube.rs` |

---

## 📦 routeplanner/ — 1 file

| # | Status | File |
|---|--------|------|
| 154 | [ ] | `src/routeplanner/mod.rs` |

---

## 📦 server/ — 4 files

| # | Status | File |
|---|--------|------|
| 155 | [ ] | `src/server/app_state.rs` |
| 156 | [ ] | `src/server/mod.rs` |
| 157 | [ ] | `src/server/session.rs` |
| 158 | [ ] | `src/server/voice.rs` |

---

## 📦 sources/ — 150 files

### sources/amazonmusic — 11 files
| # | Status | File |
|---|--------|------|
| 159 | [ ] | `src/sources/amazonmusic/api.rs` |
| 160 | [ ] | `src/sources/amazonmusic/crypt.rs` |
| 161 | [ ] | `src/sources/amazonmusic/direct.rs` |
| 162 | [ ] | `src/sources/amazonmusic/manager.rs` |
| 163 | [ ] | `src/sources/amazonmusic/mod.rs` |
| 164 | [ ] | `src/sources/amazonmusic/parsers.rs` |
| 165 | [ ] | `src/sources/amazonmusic/reader.rs` |
| 166 | [ ] | `src/sources/amazonmusic/region.rs` |
| 167 | [ ] | `src/sources/amazonmusic/streaming_reader.rs` |
| 168 | [ ] | `src/sources/amazonmusic/track.rs` |
| 169 | [ ] | `src/sources/amazonmusic/validators.rs` |

### sources/anghami — 3 files
| # | Status | File |
|---|--------|------|
| 170 | [ ] | `src/sources/anghami/manager.rs` |
| 171 | [ ] | `src/sources/anghami/mod.rs` |
| 172 | [ ] | `src/sources/anghami/reader.rs` |

### sources/applemusic — 6 files
| # | Status | File |
|---|--------|------|
| 173 | [ ] | `src/sources/applemusic/helpers.rs` |
| 174 | [ ] | `src/sources/applemusic/metadata.rs` |
| 175 | [ ] | `src/sources/applemusic/mod.rs` |
| 176 | [ ] | `src/sources/applemusic/parser.rs` |
| 177 | [ ] | `src/sources/applemusic/search.rs` |
| 178 | [ ] | `src/sources/applemusic/token.rs` |

### sources/audiomack — 4 files
| # | Status | File |
|---|--------|------|
| 179 | [ ] | `src/sources/audiomack/manager.rs` |
| 180 | [ ] | `src/sources/audiomack/mod.rs` |
| 181 | [ ] | `src/sources/audiomack/track.rs` |
| 182 | [ ] | `src/sources/audiomack/utils.rs` |

### sources/audius — 2 files
| # | Status | File |
|---|--------|------|
| 183 | [ ] | `src/sources/audius/mod.rs` |
| 184 | [ ] | `src/sources/audius/track.rs` |

### sources/bandcamp — 2 files
| # | Status | File |
|---|--------|------|
| 185 | [ ] | `src/sources/bandcamp/mod.rs` |
| 186 | [ ] | `src/sources/bandcamp/track.rs` |

### sources/deezer — 9 files
| # | Status | File |
|---|--------|------|
| 187 | [ ] | `src/sources/deezer/helpers.rs` |
| 188 | [ ] | `src/sources/deezer/metadata.rs` |
| 189 | [ ] | `src/sources/deezer/mod.rs` |
| 190 | [ ] | `src/sources/deezer/parser.rs` |
| 191 | [ ] | `src/sources/deezer/reader/crypt.rs` |
| 192 | [ ] | `src/sources/deezer/reader/mod.rs` |
| 193 | [ ] | `src/sources/deezer/reader/remote_reader.rs` |
| 194 | [ ] | `src/sources/deezer/recommendations.rs` |
| 195 | [ ] | `src/sources/deezer/search.rs` |
| 196 | [ ] | `src/sources/deezer/token.rs` |
| 197 | [ ] | `src/sources/deezer/track.rs` |

### sources/flowery — 1 file
| # | Status | File |
|---|--------|------|
| 198 | [ ] | `src/sources/flowery/mod.rs` |

### sources/gaana — 5 files
| # | Status | File |
|---|--------|------|
| 199 | [ ] | `src/sources/gaana/crypto.rs` |
| 200 | [ ] | `src/sources/gaana/manager.rs` |
| 201 | [ ] | `src/sources/gaana/mod.rs` |
| 202 | [ ] | `src/sources/gaana/reader.rs` |
| 203 | [ ] | `src/sources/gaana/track.rs` |

### sources/google_tts — 1 file
| # | Status | File |
|---|--------|------|
| 204 | [ ] | `src/sources/google_tts/mod.rs` |

### sources/http — 3 files
| # | Status | File |
|---|--------|------|
| 205 | [ ] | `src/sources/http/mod.rs` |
| 206 | [ ] | `src/sources/http/reader.rs` |
| 207 | [ ] | `src/sources/http/track.rs` |

### sources/jiosaavn — 8 files
| # | Status | File |
|---|--------|------|
| 208 | [ ] | `src/sources/jiosaavn/helpers.rs` |
| 209 | [ ] | `src/sources/jiosaavn/metadata.rs` |
| 210 | [ ] | `src/sources/jiosaavn/mod.rs` |
| 211 | [ ] | `src/sources/jiosaavn/parser.rs` |
| 212 | [ ] | `src/sources/jiosaavn/reader.rs` |
| 213 | [ ] | `src/sources/jiosaavn/recommendations.rs` |
| 214 | [ ] | `src/sources/jiosaavn/search.rs` |
| 215 | [ ] | `src/sources/jiosaavn/track.rs` |

### sources/lastfm — 4 files
| # | Status | File |
|---|--------|------|
| 216 | [ ] | `src/sources/lastfm/helpers.rs` |
| 217 | [ ] | `src/sources/lastfm/metadata.rs` |
| 218 | [ ] | `src/sources/lastfm/mod.rs` |
| 219 | [ ] | `src/sources/lastfm/search.rs` |

### sources/local — 2 files
| # | Status | File |
|---|--------|------|
| 220 | [ ] | `src/sources/local/mod.rs` |
| 221 | [ ] | `src/sources/local/track.rs` |

### sources/manager — 4 files
| # | Status | File |
|---|--------|------|
| 222 | [ ] | `src/sources/manager/best_match.rs` |
| 223 | [ ] | `src/sources/manager/mod.rs` |
| 224 | [ ] | `src/sources/manager/registration.rs` |
| 225 | [ ] | `src/sources/manager/resolver.rs` |

### sources/mixcloud — 3 files
| # | Status | File |
|---|--------|------|
| 226 | [ ] | `src/sources/mixcloud/mod.rs` |
| 227 | [ ] | `src/sources/mixcloud/reader.rs` |
| 228 | [ ] | `src/sources/mixcloud/track.rs` |

### sources/netease — 4 files
| # | Status | File |
|---|--------|------|
| 229 | [ ] | `src/sources/netease/api.rs` |
| 230 | [ ] | `src/sources/netease/manager.rs` |
| 231 | [ ] | `src/sources/netease/mod.rs` |
| 232 | [ ] | `src/sources/netease/track.rs` |

### sources/pandora — 3 files
| # | Status | File |
|---|--------|------|
| 233 | [ ] | `src/sources/pandora/manager.rs` |
| 234 | [ ] | `src/sources/pandora/mod.rs` |
| 235 | [ ] | `src/sources/pandora/token.rs` |

### sources/qobuz — 4 files
| # | Status | File |
|---|--------|------|
| 236 | [ ] | `src/sources/qobuz/manager.rs` |
| 237 | [ ] | `src/sources/qobuz/mod.rs` |
| 238 | [ ] | `src/sources/qobuz/token.rs` |
| 239 | [ ] | `src/sources/qobuz/track.rs` |

### sources/reddit — 3 files
| # | Status | File |
|---|--------|------|
| 240 | [ ] | `src/sources/reddit/manager.rs` |
| 241 | [ ] | `src/sources/reddit/mod.rs` |
| 242 | [ ] | `src/sources/reddit/track.rs` |

### sources/shazam — 1 file
| # | Status | File |
|---|--------|------|
| 243 | [ ] | `src/sources/shazam/mod.rs` |

### sources/soundcloud — 5 files
| # | Status | File |
|---|--------|------|
| 244 | [ ] | `src/sources/soundcloud/manager.rs` |
| 245 | [ ] | `src/sources/soundcloud/mod.rs` |
| 246 | [ ] | `src/sources/soundcloud/reader.rs` |
| 247 | [ ] | `src/sources/soundcloud/token.rs` |
| 248 | [ ] | `src/sources/soundcloud/track.rs` |

### sources/spotify — 7 files
| # | Status | File |
|---|--------|------|
| 249 | [ ] | `src/sources/spotify/helpers.rs` |
| 250 | [ ] | `src/sources/spotify/metadata.rs` |
| 251 | [ ] | `src/sources/spotify/mod.rs` |
| 252 | [ ] | `src/sources/spotify/parser.rs` |
| 253 | [ ] | `src/sources/spotify/recommendations.rs` |
| 254 | [ ] | `src/sources/spotify/search.rs` |
| 255 | [ ] | `src/sources/spotify/token.rs` |

### sources/tidal — 8 files
| # | Status | File |
|---|--------|------|
| 256 | [ ] | `src/sources/tidal/client.rs` |
| 257 | [ ] | `src/sources/tidal/error.rs` |
| 258 | [ ] | `src/sources/tidal/manager.rs` |
| 259 | [ ] | `src/sources/tidal/mod.rs` |
| 260 | [ ] | `src/sources/tidal/model.rs` |
| 261 | [ ] | `src/sources/tidal/oauth.rs` |
| 262 | [ ] | `src/sources/tidal/token.rs` |
| 263 | [ ] | `src/sources/tidal/track.rs` |

### sources/twitch — 4 files
| # | Status | File |
|---|--------|------|
| 264 | [ ] | `src/sources/twitch/api.rs` |
| 265 | [ ] | `src/sources/twitch/manager.rs` |
| 266 | [ ] | `src/sources/twitch/mod.rs` |
| 267 | [ ] | `src/sources/twitch/track.rs` |

### sources/vkmusic — 5 files
| # | Status | File |
|---|--------|------|
| 268 | [ ] | `src/sources/vkmusic/api.rs` |
| 269 | [ ] | `src/sources/vkmusic/manager.rs` |
| 270 | [ ] | `src/sources/vkmusic/mod.rs` |
| 271 | [ ] | `src/sources/vkmusic/track.rs` |
| 272 | [ ] | `src/sources/vkmusic/utils.rs` |

### sources/yandexmusic — 3 files
| # | Status | File |
|---|--------|------|
| 273 | [ ] | `src/sources/yandexmusic/mod.rs` |
| 274 | [ ] | `src/sources/yandexmusic/track.rs` |
| 275 | [ ] | `src/sources/yandexmusic/utils.rs` |

### sources/youtube — 30 files
| # | Status | File |
|---|--------|------|
| 276 | [ ] | `src/sources/youtube/cipher.rs` |
| 277 | [ ] | `src/sources/youtube/clients/android.rs` |
| 278 | [ ] | `src/sources/youtube/clients/android_vr.rs` |
| 279 | [ ] | `src/sources/youtube/clients/common.rs` |
| 280 | [ ] | `src/sources/youtube/clients/ios.rs` |
| 281 | [ ] | `src/sources/youtube/clients/mod.rs` |
| 282 | [ ] | `src/sources/youtube/clients/music_android.rs` |
| 283 | [ ] | `src/sources/youtube/clients/tv.rs` |
| 284 | [ ] | `src/sources/youtube/clients/tv_cast.rs` |
| 285 | [ ] | `src/sources/youtube/clients/tv_embedded.rs` |
| 286 | [ ] | `src/sources/youtube/clients/tv_simply.rs` |
| 287 | [ ] | `src/sources/youtube/clients/tv_unplugged.rs` |
| 288 | [ ] | `src/sources/youtube/clients/web.rs` |
| 289 | [ ] | `src/sources/youtube/clients/web_embedded.rs` |
| 290 | [ ] | `src/sources/youtube/clients/web_parent_tools.rs` |
| 291 | [ ] | `src/sources/youtube/clients/web_remix.rs` |
| 292 | [ ] | `src/sources/youtube/extractor.rs` |
| 293 | [ ] | `src/sources/youtube/hls/fetcher.rs` |
| 294 | [ ] | `src/sources/youtube/hls/mod.rs` |
| 295 | [ ] | `src/sources/youtube/hls/parser.rs` |
| 296 | [ ] | `src/sources/youtube/hls/resolver.rs` |
| 297 | [ ] | `src/sources/youtube/hls/ts_demux.rs` |
| 298 | [ ] | `src/sources/youtube/hls/types.rs` |
| 299 | [ ] | `src/sources/youtube/hls/utils.rs` |
| 300 | [ ] | `src/sources/youtube/mod.rs` |
| 301 | [ ] | `src/sources/youtube/oauth.rs` |
| 302 | [ ] | `src/sources/youtube/reader.rs` |
| 303 | [ ] | `src/sources/youtube/track.rs` |
| 304 | [ ] | `src/sources/youtube/ua.rs` |
| 305 | [ ] | `src/sources/youtube/utils.rs` |

---

## 📦 ws/ — 2 files

| # | Status | File |
|---|--------|------|
| 306 | [ ] | `src/ws/handler.rs` |
| 307 | [ ] | `src/ws/mod.rs` |

---

## 📦 root src/ — 2 files

| # | Status | File |
|---|--------|------|
| 308 | [ ] | `src/lib.rs` |
| 309 | [ ] | `src/main.rs` |

---

## 📊 Progress Summary

| Module | Files | Done | Critical | Skipped |
|--------|-------|------|----------|---------|
| audio/ | 50 | 50 ✓ | 0 | 0 |
| common/ | 9 | 9 ✓ | 0 | 0 |
| config/ | 34 | 34 ✓ | 0 | 0 |
| gateway/ | 13 | 13 ✓ | 0 | 0 |
| lyrics/ | 10 | 10 ✓ | 0 | 1 |
| monitoring/ | 3 | 3 ✓ | 0 | 0 |
| player/ | 8 | 8 ✓ | 0 | 0 |
| protocol/ | 13 | 0 | 0 | 0 |
| rest/ | 13 | 0 | 0 | 0 |
| routeplanner/ | 1 | 0 | 0 | 0 |
| server/ | 4 | 0 | 0 | 0 |
| sources/ | 150+ | 0 | 0 | 0 |
| ws/ | 2 | 0 | 0 | 0 |
| src/ (root) | 2 | 0 | 0 | 0 |
| **Total** | **309** | **0** | **0** | **0** |

---

*Last updated: 2026-03-17 — Type `start`, `next`, `goto <N>`, `skip`, `summary` to control the audit.*
