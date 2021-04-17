use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tokio::fs;
use url::Url;

pub async fn main(url: &str) -> Result<()> {
    let resp = reqwest::get(url).await?.text().await?;
    fs::write("temp/zoo.html", &resp).await?;

    let re = Regex::new(r"ytInitialPlayerResponse\s*=\s*(\{.+?\});")?;

    let yt = re.captures(&resp).unwrap().get(1).unwrap().as_str();
    fs::write("temp/yt.json", yt).await?;

    let yt: YtInitialPlayerResponse = serde_json::from_str(yt)?;
    fs::write("temp/yt.pretty.json", serde_json::to_string_pretty(&yt)?).await?;

    for format in yt.streaming_data.adaptive_formats {
        match format {
            AdaptiveFormats::Video { url, .. } => {
                println!("Video url: {}", &url)
            }
            AdaptiveFormats::CipheredVideo {
                signature_cipher, ..
            } => {
                println!("Video cipher: {}", &signature_cipher)
            }
            AdaptiveFormats::Audio { url, .. } => {
                println!("Audio url: {}", &url)
            }
            AdaptiveFormats::CipheredAudio {
                signature_cipher, ..
            } => {
                println!("Audio cipher: {}", &signature_cipher)
            }
        }
    }

    // dbg!(&yt);

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct YtInitialPlayerResponse {
    #[serde(rename = "streamingData")]
    pub streaming_data: StreamingData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StreamingData {
    #[serde(rename = "expiresInSeconds")]
    pub expires_in_seconds: String,
    pub formats: Vec<Formats>,
    #[serde(rename = "adaptiveFormats")]
    pub adaptive_formats: Vec<AdaptiveFormats>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Formats {
    Format {
        itag: u32,
        url: String,
        #[serde(rename = "mimeType")]
        mime_type: String,
        bitrate: u32,
        width: u32,
        height: u32,
        #[serde(rename = "lastModified")]
        last_modified: String,
        #[serde(rename = "contentLength")]
        content_length: Option<String>,
        quality: String,
        fps: u32,
        #[serde(rename = "qualityLabel")]
        quality_label: String,
        #[serde(rename = "projectionType")]
        projection_type: String,
        #[serde(rename = "averageBitrate")]
        average_bitrate: Option<u32>,
        #[serde(rename = "audioQuality")]
        audio_quality: AudioQuality,
        #[serde(rename = "approxDurationMs")]
        approx_duration_ms: String,
        #[serde(rename = "audioSampleRate")]
        audio_sample_rate: String,
        #[serde(rename = "audioChannels")]
        audio_channels: u32,
    },
    CipheredFormat {
        itag: u32,
        #[serde(rename = "mimeType")]
        mime_type: String,
        bitrate: u32,
        width: u32,
        height: u32,
        #[serde(rename = "lastModified")]
        last_modified: String,
        #[serde(rename = "contentLength")]
        content_length: Option<String>,
        quality: String,
        fps: u32,
        #[serde(rename = "qualityLabel")]
        quality_label: String,
        #[serde(rename = "projectionType")]
        projection_type: String,
        #[serde(rename = "averageBitrate")]
        average_bitrate: Option<u32>,
        #[serde(rename = "audioQuality")]
        audio_quality: AudioQuality,
        #[serde(rename = "approxDurationMs")]
        approx_duration_ms: String,
        #[serde(rename = "audioSampleRate")]
        audio_sample_rate: String,
        #[serde(rename = "audioChannels")]
        audio_channels: u32,
        #[serde(rename = "signatureCipher")]
        signature_cipher: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AdaptiveFormats {
    Video {
        itag: u32,
        url: String,
        #[serde(rename = "mimeType")]
        mime_type: String,
        bitrate: u32,
        width: u32,
        height: u32,
        #[serde(rename = "initRange")]
        init_range: Range,
        #[serde(rename = "indexRange")]
        index_range: Range,
        #[serde(rename = "lastModified")]
        last_modified: String,
        #[serde(rename = "contentLength")]
        content_length: String,
        quality: String,
        fps: u32,
        #[serde(rename = "qualityLabel")]
        quality_label: String,
        #[serde(rename = "projectionType")]
        projection_type: String,
        #[serde(rename = "averageBitrate")]
        average_bitrate: u32,
        #[serde(rename = "colorInfo")]
        color_info: Option<ColorInfo>,
        #[serde(rename = "approxDurationMs")]
        approx_duration_ms: String,
    },
    CipheredVideo {
        itag: u32,
        #[serde(rename = "mimeType")]
        mime_type: String,
        bitrate: u32,
        width: u32,
        height: u32,
        #[serde(rename = "initRange")]
        init_range: Range,
        #[serde(rename = "indexRange")]
        index_range: Range,
        #[serde(rename = "lastModified")]
        last_modified: String,
        #[serde(rename = "contentLength")]
        content_length: String,
        quality: String,
        fps: u32,
        #[serde(rename = "qualityLabel")]
        quality_label: String,
        #[serde(rename = "projectionType")]
        projection_type: String,
        #[serde(rename = "averageBitrate")]
        average_bitrate: u32,
        #[serde(rename = "colorInfo")]
        color_info: Option<ColorInfo>,
        #[serde(rename = "approxDurationMs")]
        approx_duration_ms: String,
        #[serde(rename = "signatureCipher")]
        signature_cipher: String,
    },
    Audio {
        itag: u32,
        url: String,
        #[serde(rename = "mimeType")]
        mime_type: String,
        bitrate: u32,
        #[serde(rename = "initRange")]
        init_range: Range,
        #[serde(rename = "indexRange")]
        index_range: Range,
        #[serde(rename = "lastModified")]
        last_modified: String,
        #[serde(rename = "contentLength")]
        content_length: String,
        quality: String,
        #[serde(rename = "projectionType")]
        projection_type: String,
        #[serde(rename = "averageBitrate")]
        average_bitrate: u32,
        #[serde(rename = "highReplication")]
        high_replication: Option<bool>,
        #[serde(rename = "audioQuality")]
        audio_quality: AudioQuality,
        #[serde(rename = "approxDurationMs")]
        approx_duration_ms: String,
        #[serde(rename = "audioSampleRate")]
        audio_sample_rate: String,
        #[serde(rename = "audioChannels")]
        audio_channels: u32,
        #[serde(rename = "loudnessDb")]
        loudness_db: f32,
    },

    CipheredAudio {
        itag: u32,
        #[serde(rename = "mimeType")]
        mime_type: String,
        bitrate: u32,
        #[serde(rename = "initRange")]
        init_range: Range,
        #[serde(rename = "indexRange")]
        index_range: Range,
        #[serde(rename = "lastModified")]
        last_modified: String,
        #[serde(rename = "contentLength")]
        content_length: String,
        quality: String,
        #[serde(rename = "projectionType")]
        projection_type: String,
        #[serde(rename = "averageBitrate")]
        average_bitrate: u32,
        #[serde(rename = "highReplication")]
        high_replication: Option<bool>,
        #[serde(rename = "audioQuality")]
        audio_quality: AudioQuality,
        #[serde(rename = "approxDurationMs")]
        approx_duration_ms: String,
        #[serde(rename = "audioSampleRate")]
        audio_sample_rate: String,
        #[serde(rename = "audioChannels")]
        audio_channels: u32,
        #[serde(rename = "loudnessDb")]
        loudness_db: f32,
        #[serde(rename = "signatureCipher")]
        signature_cipher: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Range {
    pub start: String,
    pub end: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ColorInfo {
    pub primaries: Option<String>,
    #[serde(rename = "transferCharacteristics")]
    pub transfer_characteristics: Option<String>,
    #[serde(rename = "matrixCoefficients")]
    pub matrix_coefficients: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AudioQuality {
    #[serde(rename = "AUDIO_QUALITY_LOW")]
    AudioQualityLow,
    #[serde(rename = "AUDIO_QUALITY_MEDIUM")]
    AudioQualityMedium,
}
