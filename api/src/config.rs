use serde::Deserialize;
use url::Url;

/// Configuration parameters for the application
///
/// Config values are picked up from environment variables (e.g. `BASE_URL`)
/// See `.env.sample` in the repository root for details.
#[derive(Deserialize)]
pub struct Config {
    /// URL where this service is running
    ///
    /// Used for generating full URLs like OAuth Callback URLs)
    pub base_url: Url,

    pub port: u16,

    pub allow_origin: Option<Url>,

    /// Spice API for querying data and models
    pub spice_api: Url,
}
