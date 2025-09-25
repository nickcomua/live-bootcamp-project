use std::sync::Arc;

use tokio::sync::RwLock;

use crate::domain::{BannedTokenStore, UserStore};

// Using a type alias to improve readability!
pub type UserStoreType = Arc<RwLock<dyn UserStore>>;
pub type BannedTokenStoreType = Arc<RwLock<dyn BannedTokenStore>>;
#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub ban_token_store: BannedTokenStoreType,
}
