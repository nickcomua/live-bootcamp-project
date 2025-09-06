use std::collections::HashMap;

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        if let Some(user) = self.users.get(email) {
            Ok(user.clone())
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        if let Some(user) = self.users.get(email) {
            if user.password == password {
                Ok(())
            } else {
                Err(UserStoreError::InvalidCredentials)
            }
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }
}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_user() {
        let mut store = HashmapUserStore::default();

        let user = User::new("test@example.com".to_string(), "password123".to_string(), true);

        assert!(store.add_user(user.clone()).is_ok());
        assert_eq!(store.get_user("test@example.com").unwrap(), user);
    }

    #[test]
    fn test_get_user() {
        let mut store = HashmapUserStore::default();

        let user = User::new("test@example.com".to_string(), "password123".to_string(), true);

        assert!(store.add_user(user.clone()).is_ok());
        assert_eq!(store.get_user("test@example.com").unwrap(), user); 
    }

    #[test]
    fn test_validate_user() {
        let mut store = HashmapUserStore::default();

        let user = User::new("test@example.com".to_string(), "password123".to_string(), true);

        assert!(store.add_user(user.clone()).is_ok());
        assert!(store.validate_user("test@example.com", "password123").is_ok());
        assert!(store.validate_user("test@example.com", "password1234").is_err());
    }
}
