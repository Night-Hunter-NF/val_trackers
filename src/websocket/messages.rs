use serde::{Deserializer, Serializer};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

// pub static EVENTS: &'static [&str] = &[
//     "OnClientMinimize",
//     "OnClientFocus",
//     "OnJsonApiEvent_agent_v1_session",
//     "OnJsonApiEvent_agent_v1_requests",
//     "OnJsonApiEvent_chat_v1_session",
//    asdfasds "OnJsonApiEvent_chat_v4_presences",
//     "OnJsonApiEvent_chat_v4_friends",
//  ASDFASD   "OnJsonApiEvent_chat_v5_messages",
//   asdfadf  "OnJsonApiEvent_chat_v5_participants",
//asdfasd "OnJsonApiEvent_chat_v6_conversations",
//     "OnJsonApiEvent_chat_v6_friendrequests",
// "OnJsonApiEvent_riot-messaging-service_v1_message",
// ];

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Messages {
    ChatV6Conversations(ChatV6Conversations),
    ChatV5Messages(ChatV5Messages),
    ChatV4Presences(ChatV4Presences),
    ChatV5Participants(ChatV5Participants),
    ServiceMessage(ServiceMessage),
    None,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceMessage {
    pub data: ServiceMessageData,
    pub event_type: String,
    pub uri: String,
}

impl ServiceMessage {
    pub fn pre_match(self) -> Option<String> {
        if self.data.service == "pregame" {
            let string_parts = self.data.resource.split("/").collect::<Vec<&str>>();

            Some(string_parts[string_parts.len() - 1].to_string())
        } else {
            None
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceMessageData {
    pub ack_required: bool,
    pub id: String,
    pub payload: String,
    pub resource: String,
    pub service: String,
    pub timestamp: i64,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatV5Participants {
    pub data: ParticipantsData,
    pub event_type: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantsData {
    pub participants: Vec<Participant>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    pub active_platform: Value,
    pub cid: String,
    #[serde(rename = "game_name")]
    pub game_name: String,
    #[serde(rename = "game_tag")]
    pub game_tag: String,
    pub muted: bool,
    pub name: String,
    pub pid: String,
    pub puuid: String,
    pub region: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatV4Presences {
    pub data: PresencesData,
    pub event_type: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresencesData {
    pub presences: Vec<Presence>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Presence {
    pub actor: Value,
    pub basic: String,
    pub details: Value,
    #[serde(rename = "game_name")]
    pub game_name: String,
    #[serde(rename = "game_tag")]
    pub game_tag: String,
    pub location: Value,
    pub msg: Value,
    pub name: String,
    pub patchline: Value,
    pub pid: String,
    pub platform: Value,
    #[serde(deserialize_with = "from_base64")]
    pub private: Private,
    pub private_jwt: Value,
    pub product: String,
    pub puuid: String,
    pub region: String,
    pub resource: String,
    pub state: String,
    pub summary: String,
    pub time: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Private {
    pub is_valid: bool,
    pub session_loop_state: String,
    pub party_owner_session_loop_state: String,
    pub custom_game_name: String,
    pub custom_game_team: String,
    pub party_owner_match_map: String,
    pub party_owner_match_current_team: String,
    pub party_owner_match_score_ally_team: i64,
    pub party_owner_match_score_enemy_team: i64,
    pub party_owner_provisioning_flow: String,
    pub provisioning_flow: String,
    pub match_map: String,
    pub party_id: String,
    pub is_party_owner: bool,
    pub party_state: String,
    pub party_accessibility: String,
    pub max_party_size: i64,
    pub queue_id: String,
    #[serde(rename = "partyLFM")]
    pub party_lfm: bool,
    pub party_client_version: String,
    pub party_size: i64,
    pub tournament_id: String,
    pub roster_id: String,
    pub party_version: i64,
    pub queue_entry_time: String,
    pub player_card_id: String,
    pub player_title_id: String,
    pub preferred_level_border_id: String,
    pub account_level: i64,
    pub competitive_tier: i64,
    pub leaderboard_position: i64,
    pub is_idle: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatV5Messages {
    pub data: MessagesData,
    pub event_type: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagesData {
    pub messages: Vec<Message>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub body: String,
    pub cid: String,
    #[serde(rename = "game_name")]
    pub game_name: String,
    #[serde(rename = "game_tag")]
    pub game_tag: String,
    pub id: String,
    pub mid: String,
    pub name: String,
    pub pid: String,
    pub puuid: String,
    pub read: bool,
    pub region: String,
    pub time: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatV6Conversations {
    pub data: ConversationsData,
    pub event_type: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationsData {
    pub conversations: Vec<Conversation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Conversation {
    pub cid: String,
    #[serde(rename = "direct_messages")]
    pub direct_messages: bool,
    #[serde(rename = "global_readership")]
    pub global_readership: bool,
    #[serde(rename = "message_history")]
    pub message_history: bool,
    pub mid: String,
    pub muted: bool,
    pub muted_restriction: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub ui_state: UiState,
    #[serde(rename = "unread_count")]
    pub unread_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiState {
    pub changed_since_hidden: bool,
    pub hidden: bool,
}

fn from_base64<'de, D>(deserializer: D) -> Result<Private, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    use serde::Deserialize;
    String::deserialize(deserializer)
        .and_then(|string| base64::decode(&string).map_err(|err| Error::custom(err.to_string())))
        .map(|bytes| serde_json::from_slice::<Private>(&bytes))
        .and_then(|opt| opt.map_err(|_| Error::custom("failed to deserialize private")))
}
