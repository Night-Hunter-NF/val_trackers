pub mod api;
mod pages;
pub mod theme;
mod websocket;

use std::{collections::HashMap, env, sync::Arc};

use api::{
    matchs::{get_matchs, Matchs},
    profile::{get_profile, Profile},
    single_match::{get_match, Match},
    token_n_entitlement::TokenNEntitlement,
};
use iced::{
    executor,
    widget::{self, button, column, container, row, text},
    Application, Command, Settings, Theme,
};
use pages::{home::home_page, live::live_page};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};
use tokio::{fs, sync::Mutex};

use crate::websocket::messages;

const BASE_URL: &str = "https://public-api.tracker.gg/v2/valorant/standard";
const TRN_API_KEY: &str = "319e5540-bd60-4f5a-9660-6858c9a01350";

pub type Element = iced::Element<'static, Message, iced::Renderer>;

// KEY: "319e5540-bd60-4f5a-9660-6858c9a01350",
// PROFILE: "https://api.tracker.gg/api/v2/valorant/standard/profile/",
// MATCHES: "https://api.tracker.gg/api/v2/valorant/standard/matches/",
// INSIGHTS: "https://api.tracker.gg/api/v1/valorant/insights/",
// SEASONS: "https://api.tracker.gg/api/v1/valorant/db/seasons",
// GUIDES: "https://api.tracker.gg/api/v1/valorant/guides",
// BULK: "https://api.tracker.gg/api/v1/valorant/overwolf/ow-bulk-lookup",

// "/segments/agent?playlist="
// "/segments/map?playlist="
// "/segments/weapon?playlist="

#[derive(Clone, Debug)]
pub struct LockFile {
    pub name: String,
    pub pid: String,
    port: String,
    password: String,
    pub protocol: String,
}

#[tokio::main]
async fn main() {
    // let json = "{\"data\":{\"presences\":[{\"actor\":null,\"basic\":\"\",\"details\":null,\"game_name\":\"Cupid\",\"game_tag\":\"2840\",\"location\":null,\"msg\":null,\"name\":\"\",\"patchline\":null,\"pid\":\"6c1f63c8-39db-5231-a59f-5e589ae8633b@br1.pvp.net\",\"platform\":null,\"private\":\"ew0KCSJpc1ZhbGlkIjogdHJ1ZSwNCgkic2Vzc2lvbkxvb3BTdGF0ZSI6ICJJTkdBTUUiLA0KCSJwYXJ0eU93bmVyU2Vzc2lvbkxvb3BTdGF0ZSI6ICJJTkdBTUUiLA0KCSJjdXN0b21HYW1lTmFtZSI6ICIiLA0KCSJjdXN0b21HYW1lVGVhbSI6ICIiLA0KCSJwYXJ0eU93bmVyTWF0Y2hNYXAiOiAiL0dhbWUvTWFwcy9UcmlhZC9UcmlhZCIsDQoJInBhcnR5T3duZXJNYXRjaEN1cnJlbnRUZWFtIjogIlJlZCIsDQoJInBhcnR5T3duZXJNYXRjaFNjb3JlQWxseVRlYW0iOiA0LA0KCSJwYXJ0eU93bmVyTWF0Y2hTY29yZUVuZW15VGVhbSI6IDQsDQoJInBhcnR5T3duZXJQcm92aXNpb25pbmdGbG93IjogIk1hdGNobWFraW5nIiwNCgkicHJvdmlzaW9uaW5nRmxvdyI6ICJNYXRjaG1ha2luZyIsDQoJIm1hdGNoTWFwIjogIi9HYW1lL01hcHMvVHJpYWQvVHJpYWQiLA0KCSJwYXJ0eUlkIjogIjAwMjMyMDAwLWIyMjUtNGVhZS05MGQ2LTM4ZmViMmI5MmU2MyIsDQoJImlzUGFydHlPd25lciI6IGZhbHNlLA0KCSJwYXJ0eVN0YXRlIjogIkRFRkFVTFQiLA0KCSJwYXJ0eUFjY2Vzc2liaWxpdHkiOiAiQ0xPU0VEIiwNCgkibWF4UGFydHlTaXplIjogNSwNCgkicXVldWVJZCI6ICJjb21wZXRpdGl2ZSIsDQoJInBhcnR5TEZNIjogZmFsc2UsDQoJInBhcnR5Q2xpZW50VmVyc2lvbiI6ICJyZWxlYXNlLTA1LjEwLXNoaXBwaW5nLTExLTc5Njk4NCIsDQoJInBhcnR5U2l6ZSI6IDIsDQoJInRvdXJuYW1lbnRJZCI6ICIiLA0KCSJyb3N0ZXJJZCI6ICIiLA0KCSJwYXJ0eVZlcnNpb24iOiAxNjY5Mzc3Njk2MzIwLA0KCSJxdWV1ZUVudHJ5VGltZSI6ICIyMDIyLjExLjI1LTExLjQ3LjE1IiwNCgkicGxheWVyQ2FyZElkIjogIjdjZjA2NTUwLTQzMmMtODg0MC1mOWM3LWE2YjcxZWU4NTIxYSIsDQoJInBsYXllclRpdGxlSWQiOiAiNThkYmM0YWYtNGJhZi1iZDRiLTcwODQtOWY5MjQ4NWI0MDA2IiwNCgkicHJlZmVycmVkTGV2ZWxCb3JkZXJJZCI6ICIiLA0KCSJhY2NvdW50TGV2ZWwiOiAzNywNCgkiY29tcGV0aXRpdmVUaWVyIjogMCwNCgkibGVhZGVyYm9hcmRQb3NpdGlvbiI6IDAsDQoJImlzSWRsZSI6IGZhbHNlDQp9\",\"privateJwt\":null,\"product\":\"valorant\",\"puuid\":\"6c1f63c8-39db-5231-a59f-5e589ae8633b\",\"region\":\"br1\",\"resource\":\"RC-3953891677\",\"state\":\"dnd\",\"summary\":\"\",\"time\":1669377627578}]},\"eventType\":\"Update\",\"uri\":\"/chat/v4/presences\"}";

    // let test = serde_json::from_str::<messages::Messages>(json);
    // println!("{:?}", test);

    App::run(Settings {
        ..Default::default()
    })
    .unwrap();
}

#[derive(Clone, Debug)]
pub enum Recieved {
    Match(Match),
    Matchs(Matchs),
    Profile(Profile),
}

#[derive(Clone, Debug)]
pub enum Message {
    Match(String),
    Matchs,
    Profile,
    UpdateUser(String),
    Receiver(Recieved),
    SetPage(Pages),
    SetHomePage(HomePages),
    Event(websocket::Event),
    File(String),
    SetLiveState(LiveState),
}

#[derive(Clone, Debug)]
pub struct CurrentUser {
    name: String,
    tag: String,
    pid: Option<String>,
}

impl Default for CurrentUser {
    fn default() -> Self {
        Self {
            name: "NightHunter".to_string(),
            tag: "000".to_string(),
            pid: None,
        }
    }
}

#[derive(Clone, Debug, EnumIter, IntoStaticStr)]
pub enum Pages {
    Home,
    Live,
}
#[derive(Clone, Debug)]
pub enum HomePages {
    Competitive,
}

#[derive(Clone, Debug, PartialEq)]
struct PreGameData {
    pre_game_match_id: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiveState {
    Menu,
    PreGame(PreGameData),
    InGame,
}

#[derive(Clone, Debug)]
struct App {
    match_info: Option<Match>,
    matchs: Option<Matchs>,
    profile: Option<Profile>,
    input_text: String,
    current_user: CurrentUser,
    page: Pages,
    home_page: HomePages,
    lock_file: Arc<Mutex<Option<LockFile>>>,
    client: reqwest::Client,
    websocket_status: String,
    live_state: LiveState,
    token: TokenNEntitlement,
    // parties: HashMap<String, Vec<String>>
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = theme::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let user = CurrentUser::default();
        let client = reqwest::Client::new();
        let lockfile = Arc::new(Mutex::new(None));
        (
            App {
                match_info: None,
                matchs: None,
                profile: None,
                input_text: "".to_string(),
                current_user: user.clone(),
                page: Pages::Home,
                home_page: HomePages::Competitive,
                lock_file: lockfile.clone(),
                client: client.clone(),

                websocket_status: "Disconected".to_string(),
                live_state: LiveState::Menu,
                token: TokenNEntitlement::new(client.clone(), lockfile),
            },
            Command::batch([
                Command::perform(
                    get_profile(user.name.clone(), user.tag.clone(), client.clone()),
                    |x| Message::Receiver(Recieved::Profile(x.unwrap())),
                ),
                Command::perform(
                    get_matchs(user.name.clone(), user.tag.clone(), client),
                    |x| Message::Receiver(Recieved::Matchs(x.unwrap())),
                ),
                Command::perform(
                    fs::read_to_string(format!(
                        "{}\\Riot Games\\Riot Client\\Config\\lockfile",
                        env::var("LocalAppData").unwrap()
                    )),
                    |x| Message::File(x.unwrap()),
                ),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("Video Player")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        websocket::val_events(&self.lock_file).map(Message::Event)
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Match(id) => {
                return Command::perform(get_match(id, self.client.clone()), |x| {
                    Message::Receiver(Recieved::Match(x.unwrap()))
                });
            }
            Message::Matchs => {
                return Command::perform(
                    get_matchs(
                        self.current_user.name.clone(),
                        self.current_user.tag.clone(),
                        self.client.clone(),
                    ),
                    |x| Message::Receiver(Recieved::Matchs(x.unwrap())),
                );
            }
            Message::Profile => {
                return Command::perform(
                    get_profile(
                        self.current_user.name.clone(),
                        self.current_user.tag.clone(),
                        self.client.clone(),
                    ),
                    |x| Message::Receiver(Recieved::Profile(x.unwrap())),
                );
            }
            Message::Receiver(item) => match item {
                Recieved::Match(m) => self.match_info = Some(m),
                Recieved::Matchs(matchs) => self.matchs = Some(matchs),
                Recieved::Profile(profile) => self.profile = Some(profile),
            },
            Message::UpdateUser(message) => {
                self.input_text = message;
            }
            Message::SetPage(page) => self.page = page,
            Message::SetHomePage(page) => self.home_page = page,
            Message::Event(event) => match event {
                websocket::Event::Connected => {
                    self.websocket_status = "Connected".to_string();
                    println!("Connected");
                }
                websocket::Event::Disconnected => {
                    self.websocket_status = "Disconnected".to_string();
                    println!("Disconnected");
                }
                websocket::Event::MessageReceived(message) => match message {
                    messages::Messages::ChatV6Conversations(_) => (),
                    messages::Messages::ChatV5Messages(_) => {}
                    messages::Messages::ChatV4Presences(pressence) => {}
                    messages::Messages::ChatV5Participants(_) => (),
                    messages::Messages::None => (),
                    messages::Messages::ServiceMessage(message) => {
                        if self.live_state == LiveState::Menu {
                            if let Some(id) = message.pre_match() {
                                self.live_state = LiveState::PreGame(PreGameData {
                                    pre_game_match_id: id,
                                });
                            }
                        }
                    }
                },
                websocket::Event::ErrSendingEvents => {
                    self.websocket_status = "ErrSendingEvents".to_string();
                    println!("ErrSendingEvents");
                }
                websocket::Event::SendEvents => {
                    self.websocket_status = "SendEvents".to_string();
                    println!("SendEvents");
                }
                websocket::Event::ValOpen => {
                    self.websocket_status = "ValOpen".to_string();
                    println!("ValOpen");
                }
                websocket::Event::WaitingForVal => {
                    self.websocket_status = "WaitingForVal".to_string();
                    println!("WaitingForVal");
                }
            },
            Message::File(f) => {
                let lock_vec = f.split(":").collect::<Vec<&str>>();
                let lock = self.lock_file.try_lock();
                if let Ok(mut file) = lock {
                    *file = Some(LockFile {
                        name: lock_vec[0].to_string(),
                        pid: lock_vec[1].to_string(),
                        port: lock_vec[2].to_string(),
                        password: lock_vec[3].to_string(),
                        protocol: lock_vec[4].to_string(),
                    })
                }
            }
            Message::SetLiveState(state) => self.live_state = state,
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let display_user = row![text(format!(
            "current user is:{}#{}",
            self.current_user.name, self.current_user.tag
        ))];

        let menu = widget::Row::with_children(
            Pages::iter()
                .map(|p| {
                    let t: &'static str = p.clone().into();
                    button(text(t)).on_press(Message::SetPage(p)).into()
                })
                .collect::<Vec<Element>>(),
        );

        let page = match self.page {
            Pages::Home => {
                if let Some(profile) = &self.profile {
                    if let Some(mat) = &self.matchs {
                        home_page(profile, mat, &self.home_page)
                    } else {
                        column![].into()
                    }
                } else {
                    column![].into()
                }
            }
            Pages::Live => live_page(&self.live_state),
        };

        container(column![display_user, menu, page])
            .center_x()
            .center_y()
            .into()
    }
}
