use iced::widget::{container, text, row, column};

use crate::LiveState;

pub fn live_page(state: &LiveState) -> crate::Element {
    match state {
        LiveState::Menu => {
    container(text("live page")).into()

        }
        LiveState::PreGame => {
            let friendly_team = column![];
            let enemy_team = column![];
            let teams = row![friendly_team, enemy_team];
            container(teams).into()

        }
        LiveState::InGame => {
    container(text("live page")).into()

        }
    }
}
