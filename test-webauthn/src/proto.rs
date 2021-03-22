
use crate::base64_data::Base64UrlSafeData;

use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct PublicKeyCredential {
    pub id: String,
    #[serde(rename = "rawId")]
    pub raw_id: Base64UrlSafeData,
    pub response: AuthenticatorAssertionResponseRaw,
    #[serde(rename = "type")]
    pub type_: String,
}

impl PublicKeyCredential {
    pub fn get_user_handle(&self) -> Option<&[u8]> {
        self.response.user_handle.as_ref().map(|uh| uh.as_ref())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthenticatorAssertionResponseRaw {
    #[serde(rename = "authenticatorData")]
    pub authenticator_data: Base64UrlSafeData,

    #[serde(rename = "clientDataJSON")]
    pub client_data_json: Base64UrlSafeData,

    pub signature: Base64UrlSafeData,

    #[serde(rename = "userHandle")]
    pub user_handle: Option<Base64UrlSafeData>,
}
