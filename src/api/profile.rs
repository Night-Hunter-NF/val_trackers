use reqwest::Client;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

use crate::TRN_API_KEY;
use crate::BASE_URL;

pub async fn get_profile(name: String, tag: String, client: Client) -> Result<Profile, reqwest::Error> {
    Ok(client
        .get(format!("{}/profile/riot/{}%23{}", BASE_URL, name, tag))
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
    pub data: Profile,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub platform_info: PlatformInfo,
    pub user_info: UserInfo,
    pub metadata: Metadata,
    pub segments: Vec<Segment>,
    pub available_segments: Vec<Value>,
    pub expiry_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformInfo {
    pub platform_slug: String,
    pub platform_user_id: String,
    pub platform_user_handle: String,
    pub platform_user_identifier: String,
    pub avatar_url: String,
    pub additional_parameters: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub user_id: Option<i64>,
    pub is_premium: bool,
    pub is_verified: bool,
    pub is_influencer: bool,
    pub is_partner: bool,
    pub country_code: Option<String>,
    pub custom_avatar_url: Value,
    pub custom_hero_url: Value,
    pub social_accounts: Vec<Value>,
    pub pageviews: i64,
    pub is_suspicious: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub active_shard: String,
    pub schema: String,
    pub privacy: String,
    pub default_playlist: String,
    pub default_season: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    #[serde(rename = "type")]
    pub type_field: String,
    pub attributes: Attributes,
    pub metadata: Metadata2,
    pub expiry_date: String,
    pub stats: Box<Stats>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub key: Option<String>,
    pub playlist: String,
    pub season_id: Option<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata2 {
    pub name: String,
    pub schema: Option<String>,
    pub image_url: Option<String>,
    pub role: Option<String>,
    pub color: Option<String>,
    pub abilities: Option<Abilities>,
    pub match_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Abilities {
    #[serde(rename = "Ability2")]
    pub ability2: Ability2,
    #[serde(rename = "Ability1")]
    pub ability1: Ability1,
    #[serde(rename = "Grenade")]
    pub grenade: Grenade,
    #[serde(rename = "Ultimate")]
    pub ultimate: Ultimate,
    #[serde(rename = "Passive")]
    pub passive: Option<Passive>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability2 {
    pub name: String,
    pub image_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability1 {
    pub name: String,
    pub image_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Grenade {
    pub name: String,
    pub image_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ultimate {
    pub name: String,
    pub image_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Passive {
    pub name: String,
    pub image_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub matches_played: Option<MatchesPlayed>,
    pub matches_won: Option<MatchesWon>,
    pub matches_lost: Option<MatchesLost>,
    pub matches_tied: Option<MatchesTied>,
    pub matches_win_pct: Option<MatchesWinPct>,
    pub matches_duration: Option<MatchesDuration>,
    pub time_played: Option<TimePlayed>,
    pub rounds_played: Option<RoundsPlayed>,
    pub rounds_won: Option<RoundsWon>,
    pub rounds_lost: Option<RoundsLost>,
    pub rounds_win_pct: Option<RoundsWinPct>,
    pub rounds_duration: Option<RoundsDuration>,
    pub score: Option<Score>,
    pub score_per_match: Option<ScorePerMatch>,
    pub score_per_round: Option<ScorePerRound>,
    pub kills: Option<Kills>,
    pub kills_per_round: Option<KillsPerRound>,
    pub kills_per_match: Option<KillsPerMatch>,
    pub deaths: Option<Deaths>,
    pub deaths_per_round: Option<DeathsPerRound>,
    pub deaths_per_match: Option<DeathsPerMatch>,
    pub assists: Option<Assists>,
    pub assists_per_round: Option<AssistsPerRound>,
    pub assists_per_match: Option<AssistsPerMatch>,
    #[serde(rename = "kDRatio")]
    pub k_dratio: Option<KDratio>,
    #[serde(rename = "kDARatio")]
    pub k_daratio: Option<KDaratio>,
    #[serde(rename = "kADRatio")]
    pub k_adratio: Option<KAdratio>,
    pub damage: Option<Damage>,
    pub damage_per_round: Option<DamagePerRound>,
    pub damage_per_match: Option<DamagePerMatch>,
    pub damage_per_minute: Option<DamagePerMinute>,
    pub damage_received: Option<DamageReceived>,
    pub headshots: Option<Headshots>,
    pub headshots_per_round: Option<HeadshotsPerRound>,
    pub headshots_percentage: Option<HeadshotsPercentage>,
    pub grenade_casts: Option<GrenadeCasts>,
    pub grenade_casts_per_round: Option<GrenadeCastsPerRound>,
    pub grenade_casts_per_match: Option<GrenadeCastsPerMatch>,
    #[serde(rename = "ability1Casts")]
    pub ability1casts: Option<Ability1Casts>,
    #[serde(rename = "ability1CastsPerRound")]
    pub ability1casts_per_round: Option<Ability1CastsPerRound>,
    #[serde(rename = "ability1CastsPerMatch")]
    pub ability1casts_per_match: Option<Ability1CastsPerMatch>,
    #[serde(rename = "ability2Casts")]
    pub ability2casts: Option<Ability2Casts>,
    #[serde(rename = "ability2CastsPerRound")]
    pub ability2casts_per_round: Option<Ability2CastsPerRound>,
    #[serde(rename = "ability2CastsPerMatch")]
    pub ability2casts_per_match: Option<Ability2CastsPerMatch>,
    pub ultimate_casts: Option<UltimateCasts>,
    pub ultimate_casts_per_round: Option<UltimateCastsPerRound>,
    pub ultimate_casts_per_match: Option<UltimateCastsPerMatch>,
    pub dealt_headshots: Option<DealtHeadshots>,
    pub dealt_bodyshots: Option<DealtBodyshots>,
    pub dealt_legshots: Option<DealtLegshots>,
    pub received_headshots: Option<ReceivedHeadshots>,
    pub received_bodyshots: Option<ReceivedBodyshots>,
    pub received_legshots: Option<ReceivedLegshots>,
    pub econ_rating: Option<EconRating>,
    pub econ_rating_per_match: Option<EconRatingPerMatch>,
    pub econ_rating_per_round: Option<EconRatingPerRound>,
    pub suicides: Option<Suicides>,
    pub first_bloods: Option<FirstBloods>,
    pub first_bloods_per_match: Option<FirstBloodsPerMatch>,
    pub first_deaths: Option<FirstDeaths>,
    pub last_deaths: Option<LastDeaths>,
    pub survived: Option<Survived>,
    pub traded: Option<Traded>,
    #[serde(rename = "kAST")]
    pub k_ast: Option<KAst>,
    pub most_kills_in_match: Option<MostKillsInMatch>,
    pub flawless: Option<Flawless>,
    pub thrifty: Option<Thrifty>,
    pub aces: Option<Aces>,
    pub team_aces: Option<TeamAces>,
    pub clutches: Option<Clutches>,
    pub clutches_lost: Option<ClutchesLost>,
    pub clutches1v1: Option<Clutches1v1>,
    pub clutches1v2: Option<Clutches1v2>,
    pub clutches1v3: Option<Clutches1v3>,
    pub clutches1v4: Option<Clutches1v4>,
    pub clutches1v5: Option<Clutches1v5>,
    pub clutches_lost1v1: Option<ClutchesLost1v1>,
    pub clutches_lost1v2: Option<ClutchesLost1v2>,
    pub clutches_lost1v3: Option<ClutchesLost1v3>,
    pub clutches_lost1v4: Option<ClutchesLost1v4>,
    pub clutches_lost1v5: Option<ClutchesLost1v5>,
    #[serde(rename = "kills1K")]
    pub kills1k: Option<Kills1K>,
    #[serde(rename = "kills2K")]
    pub kills2k: Option<Kills2K>,
    #[serde(rename = "kills3K")]
    pub kills3k: Option<Kills3K>,
    #[serde(rename = "kills4K")]
    pub kills4k: Option<Kills4K>,
    #[serde(rename = "kills5K")]
    pub kills5k: Option<Kills5K>,
    #[serde(rename = "kills6K")]
    pub kills6k: Option<Kills6K>,
    pub plants: Option<Plants>,
    pub plants_per_match: Option<PlantsPerMatch>,
    pub plants_per_round: Option<PlantsPerRound>,
    pub attack_kills: Option<AttackKills>,
    pub attack_deaths: Option<AttackDeaths>,
    #[serde(rename = "attackKDRatio")]
    pub attack_kdratio: Option<AttackKdratio>,
    pub attack_assists: Option<AttackAssists>,
    pub attack_rounds_won: Option<AttackRoundsWon>,
    pub attack_rounds_lost: Option<AttackRoundsLost>,
    pub attack_rounds_win_pct: Option<AttackRoundsWinPct>,
    pub attack_score: Option<AttackScore>,
    pub attack_damage: Option<AttackDamage>,
    pub attack_headshots: Option<AttackHeadshots>,
    pub attack_traded: Option<AttackTraded>,
    pub attack_survived: Option<AttackSurvived>,
    pub attack_first_bloods: Option<AttackFirstBloods>,
    pub attack_first_deaths: Option<AttackFirstDeaths>,
    #[serde(rename = "attackKAST")]
    pub attack_kast: Option<AttackKast>,
    pub defuses: Option<Defuses>,
    pub defuses_per_match: Option<DefusesPerMatch>,
    pub defuses_per_round: Option<DefusesPerRound>,
    pub defense_kills: Option<DefenseKills>,
    pub defense_deaths: Option<DefenseDeaths>,
    #[serde(rename = "defenseKDRatio")]
    pub defense_kdratio: Option<DefenseKdratio>,
    pub defense_assists: Option<DefenseAssists>,
    pub defense_rounds_won: Option<DefenseRoundsWon>,
    pub defense_rounds_lost: Option<DefenseRoundsLost>,
    pub defense_rounds_win_pct: Option<DefenseRoundsWinPct>,
    pub defense_score: Option<DefenseScore>,
    pub defense_damage: Option<DefenseDamage>,
    pub defense_headshots: Option<DefenseHeadshots>,
    pub defense_traded: Option<DefenseTraded>,
    pub defense_survived: Option<DefenseSurvived>,
    pub defense_first_bloods: Option<DefenseFirstBloods>,
    pub defense_first_deaths: Option<DefenseFirstDeaths>,
    #[serde(rename = "defenseKAST")]
    pub defense_kast: Option<DefenseKast>,
    pub rank: Option<Rank>,
    pub peak_rank: Option<PeakRank>,
    #[serde(rename = "ability1Kills")]
    pub ability1kills: Option<Ability1Kills>,
    #[serde(rename = "ability1KillsPerMatch")]
    pub ability1kills_per_match: Option<Ability1KillsPerMatch>,
    #[serde(rename = "ability2Kills")]
    pub ability2kills: Option<Ability2Kills>,
    #[serde(rename = "ability2KillsPerMatch")]
    pub ability2kills_per_match: Option<Ability2KillsPerMatch>,
    pub grenade_kills: Option<GrenadeKills>,
    pub grenade_kills_per_match: Option<GrenadeKillsPerMatch>,
    pub primary_kills: Option<PrimaryKills>,
    pub primary_kills_per_match: Option<PrimaryKillsPerMatch>,
    pub ultimate_kills: Option<UltimateKills>,
    pub ultimate_kills_per_match: Option<UltimateKillsPerMatch>,
    pub head_pct: Option<HeadPct>,
    pub head_hits: Option<HeadHits>,
    pub body_pct: Option<BodyPct>,
    pub body_hits: Option<BodyHits>,
    pub legs_pct: Option<LegsPct>,
    pub legs_hits: Option<LegsHits>,
    pub matches: Option<Matches>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchesPlayed {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
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
pub struct MatchesWon {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
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
pub struct MatchesLost {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
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
pub struct MatchesTied {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
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
pub struct MatchesWinPct {
    pub rank: Value,
    pub percentile: Option<f64>,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata7,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata7 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchesDuration {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata8,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata8 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimePlayed {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
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
pub struct RoundsPlayed {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata10,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata10 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundsWon {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata11,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata11 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundsLost {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata12,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata12 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundsWinPct {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata13,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata13 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundsDuration {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata14,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata14 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
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
pub struct ScorePerMatch {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata16,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata16 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScorePerRound {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata17,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata17 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kills {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
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
pub struct KillsPerRound {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata19,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata19 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KillsPerMatch {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata20,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata20 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deaths {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata21,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata21 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeathsPerRound {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
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
pub struct DeathsPerMatch {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
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
pub struct Assists {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
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
pub struct AssistsPerRound {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata25,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata25 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssistsPerMatch {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata26,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata26 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KDratio {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata27,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata27 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KDaratio {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata28,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata28 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KAdratio {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata29,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata29 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Damage {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
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
pub struct DamagePerRound {
    pub rank: Value,
    pub percentile: Option<f64>,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata31,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata31 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamagePerMatch {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata32,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata32 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamagePerMinute {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata33,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata33 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageReceived {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata34,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata34 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headshots {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata35,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata35 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadshotsPerRound {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
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
pub struct HeadshotsPercentage {
    pub rank: Value,
    pub percentile: Option<f64>,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
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
pub struct GrenadeCasts {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
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
pub struct GrenadeCastsPerRound {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata39,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata39 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrenadeCastsPerMatch {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata40,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata40 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability1Casts {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
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
pub struct Ability1CastsPerRound {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
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
pub struct Ability1CastsPerMatch {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
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
pub struct Ability2Casts {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata44,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata44 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability2CastsPerRound {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata45,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata45 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability2CastsPerMatch {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
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
pub struct UltimateCasts {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata47,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata47 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UltimateCastsPerRound {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata48,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata48 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UltimateCastsPerMatch {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata49,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata49 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DealtHeadshots {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata50,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata50 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DealtBodyshots {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata51,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata51 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DealtLegshots {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata52,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata52 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceivedHeadshots {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata53,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata53 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceivedBodyshots {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata54,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata54 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceivedLegshots {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata55,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata55 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EconRating {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata56,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata56 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EconRatingPerMatch {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata57,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata57 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EconRatingPerRound {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata58,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata58 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Suicides {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata59,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata59 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirstBloods {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata60,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata60 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirstBloodsPerMatch {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata61,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata61 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirstDeaths {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata62,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata62 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastDeaths {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata63,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata63 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Survived {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata64,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata64 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Traded {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata65,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata65 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KAst {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata66,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata66 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MostKillsInMatch {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata67,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata67 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flawless {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata68,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata68 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thrifty {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata69,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata69 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aces {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata70,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata70 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamAces {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata71,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata71 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clutches {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata72,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata72 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClutchesLost {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata73,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata73 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clutches1v1 {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata74,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata74 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clutches1v2 {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata75,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata75 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clutches1v3 {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata76,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata76 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clutches1v4 {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata77,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata77 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clutches1v5 {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata78,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata78 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClutchesLost1v1 {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata79,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata79 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClutchesLost1v2 {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata80,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata80 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClutchesLost1v3 {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata81,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata81 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClutchesLost1v4 {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata82,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata82 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClutchesLost1v5 {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata83,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata83 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kills1K {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata84,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata84 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kills2K {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata85,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata85 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kills3K {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata86,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata86 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kills4K {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata87,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata87 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kills5K {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata88,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata88 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kills6K {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata89,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata89 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plants {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata90,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata90 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlantsPerMatch {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata91,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata91 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlantsPerRound {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata92,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata92 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackKills {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata93,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata93 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackDeaths {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata94,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata94 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackKdratio {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata95,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata95 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackAssists {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata96,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata96 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackRoundsWon {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata97,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata97 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackRoundsLost {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata98,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata98 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackRoundsWinPct {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata99,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata99 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackScore {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata100,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata100 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackDamage {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata101,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata101 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackHeadshots {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata102,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata102 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackTraded {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata103,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata103 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackSurvived {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata104,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata104 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackFirstBloods {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata105,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata105 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackFirstDeaths {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata106,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata106 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackKast {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata107,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata107 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Defuses {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata108,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata108 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefusesPerMatch {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata109,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata109 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefusesPerRound {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata110,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata110 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefenseKills {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata111,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata111 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefenseDeaths {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata112,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata112 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefenseKdratio {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata113,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata113 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefenseAssists {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata114,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata114 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefenseRoundsWon {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata115,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata115 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefenseRoundsLost {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata116,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata116 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefenseRoundsWinPct {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata117,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata117 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefenseScore {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata118,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata118 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefenseDamage {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata119,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata119 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefenseHeadshots {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata120,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata120 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefenseTraded {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata121,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata121 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefenseSurvived {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata122,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata122 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefenseFirstBloods {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata123,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata123 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefenseFirstDeaths {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata124,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata124 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefenseKast {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: String,
    pub category: String,
    pub metadata: Metadata125,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata125 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rank {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: String,
    pub metadata: Metadata126,
    pub value: Value,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata126 {
    pub icon_url: String,
    pub tier_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeakRank {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: String,
    pub metadata: Metadata127,
    pub value: Value,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata127 {
    pub icon_url: String,
    pub tier_name: String,
    pub act_id: String,
    pub act_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability1Kills {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata128,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata128 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability1KillsPerMatch {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata129,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata129 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability2Kills {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata130,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata130 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability2KillsPerMatch {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata131,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata131 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrenadeKills {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata132,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata132 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrenadeKillsPerMatch {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata133,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata133 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryKills {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata134,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata134 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryKillsPerMatch {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata135,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata135 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UltimateKills {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata136,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata136 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UltimateKillsPerMatch {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: String,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata137,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata137 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadPct {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata138,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata138 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadHits {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata139,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata139 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyPct {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata140,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata140 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyHits {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata141,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata141 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegsPct {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata142,
    pub value: f64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata142 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegsHits {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata143,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata143 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Matches {
    pub rank: Value,
    pub percentile: Value,
    pub display_name: Value,
    pub display_category: Value,
    pub category: Value,
    pub metadata: Metadata144,
    pub value: i64,
    pub display_value: String,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata144 {}
