use std::sync::Arc;

use tokio::sync::RwLock;

use crate::domain::{BannedTokenStore, TwoFACodeStore, UserStore};

// Using a type alias to improve readability!
pub type UserStoreType = Arc<RwLock<dyn UserStore>>;
pub type BannedTokenStoreType = Arc<RwLock<dyn BannedTokenStore>>;
pub type TwoFACodeStoreType = Arc<RwLock<dyn TwoFACodeStore>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub banned_token_store: BannedTokenStoreType,
    pub two_fa_code_store: TwoFACodeStoreType,
}
