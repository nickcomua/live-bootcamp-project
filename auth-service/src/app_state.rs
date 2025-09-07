use std::sync::{Arc};

use tokio::sync::RwLock;

use crate::domain::UserStore;

// Using a type alias to improve readability!
pub type UserStoreType = Arc<RwLock<dyn UserStore>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
}