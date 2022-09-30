use crate::*;

/// @dev The event struct format
#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct EventData {
    pub standard: String,
    pub version: String,
    pub event: String,
    pub data: Option<HashMap<String, String>>,
}

impl Default for EventData {
    fn default() -> Self {
        Self {
            standard: "nep333".into(),
            version: "0.1.0".into(),
            event: String::default(),
            data: None,
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Participant {
    pub account: AccountId,
    pub amount: U128,
}