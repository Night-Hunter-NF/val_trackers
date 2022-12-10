pub mod messages;
mod subscribe_to;

use messages::Messages;

use std::sync::Arc;

use subscribe_to::EVENTS;

use iced::{
    futures::{SinkExt, StreamExt},
    subscription, Subscription,
};
use native_tls::TlsConnector;

use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{
    tungstenite::{
        handshake::client::{generate_key, Request},
        Message,
    },
    Connector, MaybeTlsStream, WebSocketStream,
};

use crate::LockFile;

#[derive(Debug, Clone)]
pub enum Event {
    Connected,
    Disconnected,
    ErrSendingEvents,
    ValOpen,
    WaitingForVal,
    SendEvents,
    MessageReceived(Messages),
}

#[derive(Debug)]
enum ValState {
    Disconnected(Arc<Mutex<Option<LockFile>>>),
    Connected(
        WebSocketStream<MaybeTlsStream<TcpStream>>,
        Arc<Mutex<Option<LockFile>>>,
    ),
    Subscripe(
        WebSocketStream<MaybeTlsStream<TcpStream>>,
        Arc<Mutex<Option<LockFile>>>,
    ),
    Closed(
        WebSocketStream<MaybeTlsStream<TcpStream>>,
        Arc<Mutex<Option<LockFile>>>,
    ),
}

pub fn val_events(lockfile: &Arc<Mutex<Option<LockFile>>>) -> Subscription<Event> {
    subscription::unfold(
        "val_events",
        ValState::Disconnected(lockfile.clone()),
        |state| async move {
            match state {
                ValState::Disconnected(lockfile) => {
                    let file = { lockfile.lock().await.clone() };

                    if let Some(file) = file {
                        let tls_connector = TlsConnector::builder()
                            .danger_accept_invalid_certs(true)
                            .build()
                            .unwrap();
                        let request = Request::builder()
                            .method("GET")
                            .header("Host", format!("127.0.0.1:{}", file.port))
                            .header("Connection", "Upgrade")
                            .header("Upgrade", "websocket")
                            .header("Sec-WebSocket-Version", "13")
                            .header("Sec-WebSocket-Key", generate_key())
                            .header(
                                "Authorization",
                                format!(
                                    "Basic {}",
                                    base64::encode(format!("riot:{}", file.password))
                                ),
                            )
                            .uri(format!("wss://127.0.0.1:{}", file.port))
                            .body(())
                            .unwrap();
                        match tokio_tungstenite::connect_async_tls_with_config(
                            request,
                            None,
                            Some(Connector::NativeTls(tls_connector)),
                        )
                        .await
                        {
                            Ok((websocket, _)) => {
                                return (
                                    Some(Event::WaitingForVal),
                                    ValState::Closed(websocket, lockfile),
                                );
                            }
                            Err(_) => {
                                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                                return (
                                    Some(Event::Disconnected),
                                    ValState::Disconnected(lockfile),
                                );
                            }
                        }
                    } else {
                        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

                        (Some(Event::Disconnected), ValState::Disconnected(lockfile))
                    }
                }
                ValState::Closed(websocket, lockfile) => {
                    let client = reqwest::Client::builder()
                        .danger_accept_invalid_certs(true)
                        .build()
                        .unwrap();
                    let file = { lockfile.lock().await.clone() };
                    if let Some(lock) = file {
                        let test = client
                            .get(format!("https://127.0.0.1:{}/help", lock.port))
                            .basic_auth("riot", Some(lock.password))
                            .send()
                            .await;
                        if let Ok(asdf) = test {
                            let text = asdf.text().await.unwrap();
                            if text.contains("OnJsonApiEvent_chat_v4_presences") {
                                return (
                                    Some(Event::ValOpen),
                                    ValState::Subscripe(websocket, lockfile),
                                );
                            }
                        }
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

                    (None, ValState::Closed(websocket, lockfile))
                }
                ValState::Subscripe(mut websocket, lockfile) => {
                    for event in EVENTS {
                        let resualt = websocket
                            .feed(Message::Text(format!("[5, \"{}\"]", event)))
                            .await;
                        if resualt.is_err() {
                            println!("error sending event \n Err{}", resualt.unwrap_err())
                        }
                    }
                    if  websocket.flush().await.is_err() {
                        return (
                            Some(Event::ErrSendingEvents),
                            ValState::Connected(websocket, lockfile),
                        );
                    }
                    (
                        Some(Event::SendEvents),
                        ValState::Connected(websocket, lockfile),
                    )
                }
                ValState::Connected(mut websocket, lockfile) => {
                    let message = websocket.next().await;
                    if let Some(received) = message {
                        match received {
                            Ok(Message::Text(text)) => {
                                if text.starts_with("[8") {
                                    let fomated_text = text.split_once("{").unwrap().1;
                                    let json = format!(
                                        "{{{}",
                                        fomated_text.split_at(fomated_text.len() - 1).0
                                    );
                                    let resualt_message = serde_json::from_str::<Messages>(&json);
                                    if let Ok(message) = resualt_message {
                                        return (
                                            Some(Event::MessageReceived(message)),
                                            ValState::Connected(websocket, lockfile),
                                        );
                                    } else {
                                        println!("json: {}", json);
                                    }
                                }
                                (None, ValState::Connected(websocket, lockfile))
                            }
                            Ok(_) => (None, ValState::Connected(websocket, lockfile)),
                            Err(_) => (Some(Event::Disconnected), ValState::Disconnected(lockfile)),
                        }
                    } else {
                        (Some(Event::Disconnected), ValState::Disconnected(lockfile))
                    }
                }
            }
        },
    )
}
