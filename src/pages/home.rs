use iced::widget::{column, row, text};

use crate::{
    api::{matchs::Matchs, profile::Profile},
    HomePages,
};

pub fn home_page(profile: &Profile, matches: &Matchs, subpage: &HomePages) -> crate::Element {
    let submenu = row![];

    let page = match subpage {
        HomePages::Competitive => {
            let profile = profile
                .segments
                .iter()
                .find(|i| i.attributes.playlist == "competitive")
                .unwrap()
                .clone();

            let match_wins = profile.stats.matches_win_pct.unwrap();
            let kills_data = profile.stats.kills.unwrap();
            let deaths_data = profile.stats.deaths.unwrap();
            let match_kd = kills_data.value as f32 / deaths_data.value as f32;
            let headshots = profile.stats.headshots_percentage.unwrap();
            let damage_per_round = profile.stats.damage_per_round.unwrap();

            let win_percent = column![
                text(match_wins.display_name),
                text(match_wins.display_value)
            ];
            let kd = column![text("K/D Ratio"), text(format!("{:.2}", match_kd))];
            let headshot_percent =
                column![text(headshots.display_name), text(headshots.display_value)];
            let damage_round = column![
                text(damage_per_round.display_name),
                text(damage_per_round.display_value)
            ];

            let row1 = row![win_percent, kd, headshot_percent, damage_round];

            let wins_data = profile.stats.matches_won.unwrap();
            let losses_data = profile.stats.matches_lost.unwrap();
            let assists_data = profile.stats.assists.unwrap();
            let headshots_data = profile.stats.headshots.unwrap();

            let wins = column![text(wins_data.display_name), text(wins_data.display_value)];
            let losses = column![
                text(losses_data.display_name),
                text(losses_data.display_value)
            ];
            let kills = column![
                text(kills_data.display_name),
                text(kills_data.display_value)
            ];
            let deaths = column![
                text(deaths_data.display_name),
                text(deaths_data.display_value)
            ];
            let assists = column![
                text(assists_data.display_name),
                text(assists_data.display_value)
            ];
            let headshots = column![
                text(headshots_data.display_name),
                text(headshots_data.display_value)
            ];

            let row2 = row![wins, losses, kills, deaths, assists, headshots];

            let match_number = text(format!("Matchs: {}", matches.matches.len()));

            let row3 = row![match_number];

            column![row1, row2, row3]
        }
    };
    column![submenu, page].into()
}
