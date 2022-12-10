use reqwest::Client;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

use crate::BASE_URL;
use crate::TRN_API_KEY;

pub async fn get_match(match_uuid: String, client: Client) -> Result<Match, reqwest::Error> {
    Ok(client
        .get(format!("{}/matches/{}", BASE_URL, match_uuid))
        .query(&[("source", "overwolf")])
        .header("TRN-API-Key", TRN_API_KEY)
        .send()
        .await?
        .json::<Root>()
        .await?
        .data)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Match,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Match {
    pub attributes: Attributes,
    pub metadata: Metadata,
    pub segments: Vec<Segment>,
    pub streams: Value,
    pub expiry_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub mode_key: String,
    pub mode_name: String,
    pub mode_image_url: String,
    pub mode_max_rounds: i64,
    pub duration: i64,
    pub date_started: String,
    pub rounds: i64,
    pub is_ranked: bool,
    pub queue_id: String,
    pub map: String,
    pub map_name: String,
    pub map_image_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    #[serde(rename = "type")]
    pub type_field: String,
    pub attributes: Attributes2,
    pub metadata: Metadata2,
    pub expiry_date: String,
    pub stats: Stats,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes2 {
    pub team_id: Option<String>,
    pub round: Option<i64>,
    pub platform_slug: Option<String>,
    pub platform_user_identifier: Option<String>,
    pub opponent_platform_slug: Option<String>,
    pub opponent_platform_user_identifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata2 {
    pub name: Option<String>,
    pub has_won: Option<bool>,
    pub team_id: Option<String>,
    pub team_side: Option<String>,
    pub agent_key: Option<String>,
    pub platform_info: Option<PlatformInfo>,
    pub opponent_platform_info: Option<OpponentPlatformInfo>,
    pub party_id: Option<String>,
    pub agent_name: Option<String>,
    pub agent_color: Option<String>,
    pub agent_image_url: Option<String>,
    pub agent_portrait_url: Option<String>,
    pub country_code: Option<String>,
    pub opponent_location: Option<OpponentLocation>,
    #[serde(default)]
    pub player_locations: Vec<PlayerLocation>,
    #[serde(default)]
    pub assistants: Vec<Assistant>,
    pub finishing_damage: Option<FinishingDamage>,
    pub weapon_image_url: Option<String>,
    pub weapon_name: Option<String>,
    pub weapon_category: Option<String>,
    pub game_time: Option<i64>,
    pub round_time: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformInfo {
    pub platform_slug: String,
    pub platform_user_id: Value,
    pub platform_user_handle: String,
    pub platform_user_identifier: String,
    pub avatar_url: Value,
    pub additional_parameters: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpponentPlatformInfo {
    pub platform_slug: String,
    pub platform_user_id: Value,
    pub platform_user_handle: String,
    pub platform_user_identifier: String,
    pub avatar_url: Value,
    pub additional_parameters: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpponentLocation {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerLocation {
    pub puuid: String,
    pub view_radians: f64,
    pub location: Location,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assistant {
    pub platform_slug: String,
    pub platform_user_id: Value,
    pub platform_user_handle: String,
    pub platform_user_identifier: String,
    pub avatar_url: Value,
    pub additional_parameters: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinishingDamage {
    pub damage_type: String,
    pub damage_item: String,
    pub is_secondary_fire_mode: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub rounds_won: Option<RoundsWon>,
    pub rounds_lost: Option<RoundsLost>,
    pub score: Option<Score>,
    pub kills: Option<Kills>,
    pub deaths: Option<Deaths>,
    pub assists: Option<Assists>,
    pub damage: Option<Damage>,
    pub round_result: Option<RoundResult>,
    pub winning_team: Option<WinningTeam>,
    pub kd_ratio: Option<KdRatio>,
    pub loadout_value: Option<LoadoutValue>,
    pub remaining_credits: Option<RemainingCredits>,
    pub spent_credits: Option<SpentCredits>,
    pub legshots: Option<Legshots>,
    pub bodyshots: Option<Bodyshots>,
    pub headshots: Option<Headshots>,
    pub rank: Option<Rank>,
    pub curr_rank: Option<CurrRank>,
    pub score_per_round: Option<ScorePerRound>,
    pub kills_per_round: Option<KillsPerRound>,
    pub damage_per_round: Option<DamagePerRound>,
    pub single_kills: Option<SingleKills>,
    pub double_kills: Option<DoubleKills>,
    pub triple_kills: Option<TripleKills>,
    pub quadra_kills: Option<QuadraKills>,
    pub penta_kills: Option<PentaKills>,
    pub multi_kills: Option<MultiKills>,
    pub grenade_casts: Option<GrenadeCasts>,
    #[serde(rename = "ability1Casts")]
    pub ability1casts: Option<Ability1Casts>,
    #[serde(rename = "ability2Casts")]
    pub ability2casts: Option<Ability2Casts>,
    pub ultimate_casts: Option<UltimateCasts>,
    pub grenade_casts_per_round: Option<GrenadeCastsPerRound>,
    #[serde(rename = "ability1CastsPerRound")]
    pub ability1casts_per_round: Option<Ability1CastsPerRound>,
    #[serde(rename = "ability2CastsPerRound")]
    pub ability2casts_per_round: Option<Ability2CastsPerRound>,
    pub ultimate_casts_per_round: Option<UltimateCastsPerRound>,
    pub plants: Option<Plants>,
    pub defuses: Option<Defuses>,
    pub first_kills: Option<FirstKills>,
    pub first_deaths: Option<FirstDeaths>,
    pub esr: Option<Esr>,
    pub first_kills_per_round: Option<FirstKillsPerRound>,
    pub first_deaths_per_round: Option<FirstDeathsPerRound>,
    pub econ_rating: Option<EconRating>,
    pub hs_accuracy: Option<HsAccuracy>,
    pub kast: Option<Kast>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundsWon {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata3,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata3 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundsLost {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata4,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata4 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Option<String>,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata5,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata5 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kills {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Option<String>,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata6,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata6 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deaths {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Option<String>,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata7,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata7 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assists {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Option<String>,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata8,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata8 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Damage {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Option<String>,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata9,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata9 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundResult {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata10,
    pub value: String,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata10 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WinningTeam {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata11,
    pub value: String,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata11 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KdRatio {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata12,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata12 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadoutValue {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata13,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata13 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemainingCredits {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata14,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata14 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpentCredits {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata15,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata15 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Legshots {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata16,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata16 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bodyshots {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata17,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata17 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headshots {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata18,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata18 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rank {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: String,
    pub metadata: Metadata19,
    pub value: String,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata19 {
    pub icon_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrRank {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: String,
    pub metadata: Metadata20,
    pub value: String,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata20 {
    pub icon_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScorePerRound {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata21,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata21 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KillsPerRound {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata22,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata22 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamagePerRound {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata23,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata23 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleKills {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata24,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata24 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DoubleKills {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata25,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata25 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TripleKills {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata26,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata26 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuadraKills {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata27,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata27 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PentaKills {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata28,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata28 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiKills {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata29,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata29 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrenadeCasts {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata30,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata30 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability1Casts {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata31,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata31 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability2Casts {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata32,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata32 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UltimateCasts {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata33,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata33 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrenadeCastsPerRound {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata34,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata34 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability1CastsPerRound {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata35,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata35 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability2CastsPerRound {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata36,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata36 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UltimateCastsPerRound {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata37,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata37 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plants {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata38,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata38 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Defuses {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata39,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata39 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirstKills {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata40,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata40 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirstDeaths {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata41,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata41 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Esr {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata42,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata42 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirstKillsPerRound {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata43,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata43 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirstDeathsPerRound {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata44,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata44 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EconRating {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata45,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata45 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HsAccuracy {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata46,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata46 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kast {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata47,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata47 {}
