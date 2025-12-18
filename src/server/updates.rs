use dioxus::{
    fullstack::{WebSocketOptions, Websocket},
    prelude::*,
};
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use rand::random;
#[cfg(feature = "server")]
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize)]
pub(crate) struct UpdateEvent;

#[cfg(feature = "server")]
mod server_only {
    use std::{
        collections::{HashMap, HashSet},
        ops::Deref,
        sync::LazyLock,
    };

    use dioxus::fullstack::TypedWebsocket;
    use tokio::sync::{Mutex, RwLock};

    use crate::server::updates::UpdateEvent;

    pub(super) struct SubscribedClient {
        pub(super) websocket: Mutex<TypedWebsocket<UpdateEvent, UpdateEvent>>,
    }

    pub(super) struct SubscribedClients(RwLock<HashMap<u64, SubscribedClient>>);

    impl Deref for SubscribedClients {
        type Target = RwLock<HashMap<u64, SubscribedClient>>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    pub(super) static SUBSCRIBED_CLIENTS: LazyLock<SubscribedClients> =
        LazyLock::new(|| SubscribedClients(RwLock::new(HashMap::new())));

    pub(crate) async fn publish_update() {
        let mut disconnected_client_ids = HashSet::new();
        let subscribed_clients = SUBSCRIBED_CLIENTS.read().await;
        for (id, client) in subscribed_clients.iter() {
            if client
                .websocket
                .lock()
                .await
                .send(UpdateEvent)
                .await
                .is_err()
            {
                disconnected_client_ids.insert(*id);
            }
        }
        drop(subscribed_clients);
        if !disconnected_client_ids.is_empty() {
            let mut subscribed_clients = SUBSCRIBED_CLIENTS.write().await;
            subscribed_clients.retain(|id, _| !disconnected_client_ids.contains(id));
        }
    }
}

#[cfg(feature = "server")]
pub(super) use server_only::*;

#[get("/api/subscribe_to_updates")]
pub(crate) async fn subscribe_to_updates(
    websocket_options: WebSocketOptions,
) -> Result<Websocket<UpdateEvent, UpdateEvent>> {
    Ok(websocket_options.on_upgrade(move |socket| async move {
        SUBSCRIBED_CLIENTS.write().await.insert(
            random(),
            SubscribedClient {
                websocket: Mutex::new(socket),
            },
        );
    }))
}
