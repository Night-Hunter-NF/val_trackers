use reqwest::Client;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

use crate::TRN_API_KEY;
use crate::BASE_URL;

pub async fn get_matchs(name: String, tag: String, client: Client) -> Result<Matchs, reqwest::Error> {
    Ok(client
        .get(format!("{}/matches/riot/{}%23{}", BASE_URL, name, tag))
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
    pub data: Matchs,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Matchs {
    pub matches: Vec<Match>,
    pub metadata: Metadata46,
    pub pagination_type: String,
    pub requesting_player_attributes: RequestingPlayerAttributes,
    pub expiry_date: String,
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
    pub map_id: String,
    pub mode_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub mode_key: String,
    pub mode_name: String,
    pub mode_image_url: String,
    pub mode_max_rounds: i64,
    pub is_available: bool,
    pub timestamp: String,
    pub result: String,
    pub map: String,
    pub map_name: String,
    pub map_image_url: String,
    pub season_name: String,
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
    pub platform_slug: String,
    pub platform_user_identifier: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata2 {
    pub platform_user_handle: String,
    pub has_won: bool,
    pub result: String,
    pub agent: String,
    pub agent_name: String,
    pub agent_color: String,
    pub agent_image_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub playtime: Playtime,
    pub rounds_played: RoundsPlayed,
    pub rounds_won: RoundsWon,
    pub rounds_lost: RoundsLost,
    pub rounds_disconnected: RoundsDisconnected,
    pub placement: Placement,
    pub score: Score,
    pub kills: Kills,
    pub deaths: Deaths,
    pub assists: Assists,
    pub damage: Damage,
    pub damage_received: DamageReceived,
    pub headshots: Headshots,
    pub grenade_casts: GrenadeCasts,
    #[serde(rename = "ability1Casts")]
    pub ability1casts: Ability1Casts,
    #[serde(rename = "ability2Casts")]
    pub ability2casts: Ability2Casts,
    pub ultimate_casts: UltimateCasts,
    pub dealt_headshots: DealtHeadshots,
    pub dealt_bodyshots: DealtBodyshots,
    pub dealt_legshots: DealtLegshots,
    pub econ_rating: EconRating,
    pub suicides: Suicides,
    pub revived: Revived,
    pub first_bloods: FirstBloods,
    pub first_deaths: FirstDeaths,
    pub last_deaths: LastDeaths,
    pub survived: Survived,
    pub traded: Traded,
    pub kasted: Kasted,
    #[serde(rename = "kAST")]
    pub k_ast: KAst,
    pub flawless: Flawless,
    pub thrifty: Thrifty,
    pub aces: Aces,
    pub team_aces: TeamAces,
    pub clutches: Clutches,
    pub clutches_lost: ClutchesLost,
    pub plants: Plants,
    pub defuses: Defuses,
    pub kd_ratio: KdRatio,
    pub score_per_round: ScorePerRound,
    pub damage_per_round: DamagePerRound,
    pub headshots_percentage: HeadshotsPercentage,
    pub rank: Rank,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playtime {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata3,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata3 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundsPlayed {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata4,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata4 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundsWon {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata5,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata5 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundsLost {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata6,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata6 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundsDisconnected {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata7,
    pub value: Value,
    pub display_value: Value,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata7 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Placement {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata8,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata8 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata9,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata9 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kills {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata10,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata10 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deaths {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata11,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata11 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assists {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata12,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata12 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Damage {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata13,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata13 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageReceived {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata14,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata14 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headshots {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata15,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata15 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrenadeCasts {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata16,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata16 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability1Casts {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata17,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata17 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability2Casts {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata18,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata18 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UltimateCasts {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata19,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata19 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DealtHeadshots {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata20,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata20 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DealtBodyshots {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata21,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata21 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DealtLegshots {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata22,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata22 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EconRating {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata23,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata23 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Suicides {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata24,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata24 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Revived {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata25,
    pub value: Value,
    pub display_value: Value,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata25 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirstBloods {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata26,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata26 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirstDeaths {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata27,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata27 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastDeaths {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata28,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata28 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Survived {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata29,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata29 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Traded {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata30,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata30 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kasted {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata31,
    pub value: Value,
    pub display_value: Value,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata31 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KAst {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata32,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata32 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flawless {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata33,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata33 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thrifty {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata34,
    pub value: Value,
    pub display_value: Value,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata34 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aces {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata35,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata35 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamAces {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata36,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata36 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clutches {
    pub rank: Value,
    // pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata37,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata37 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClutchesLost {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata38,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata38 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plants {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata39,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata39 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Defuses {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata40,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata40 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KdRatio {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata41,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata41 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScorePerRound {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata42,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata42 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamagePerRound {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata43,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata43 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadshotsPercentage {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata44,
    pub value: Option<f64>,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata44 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rank {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: String,
    pub metadata: Metadata45,
    pub value: Value,
    pub display_value: Option<String>,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata45 {
    pub icon_url: String,
    pub tier_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata46 {
    pub schema: String,
    pub next: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestingPlayerAttributes {
    pub platform_slug: String,
    pub platform_user_identifier: String,
}
