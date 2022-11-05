use std::collections::HashSet;

use parking_lot::Mutex;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::messages::InternalMessages;

pub struct AppState {
    pub user_set: Mutex<HashSet<Uuid>>,
    pub tx: broadcast::Sender<InternalMessages>,
    pub broadcast_secret: String,
}
