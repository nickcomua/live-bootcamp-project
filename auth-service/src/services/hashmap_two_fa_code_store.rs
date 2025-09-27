use std::collections::HashMap;

use crate::domain::{
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    email::Email,
};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

// TODO: implement TwoFACodeStore for HashmapTwoFACodeStore
#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        self.codes.insert(email, (login_attempt_id, code));
        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        self.codes.remove(email);
        Ok(())
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        match self.codes.get(email) {
            Some((login_attempt_id, code)) => Ok((login_attempt_id.clone(), code.clone())),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_code() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse("test@example.com".to_owned()).unwrap();
        let login_attempt_id_old = LoginAttemptId::default();
        let code_old = TwoFACode::default();

        store.add_code(email.clone(), login_attempt_id_old.clone(), code_old.clone()).await.unwrap();

        let (login_attempt_id, code) = store.get_code(&email).await.unwrap();

        assert_eq!(login_attempt_id, login_attempt_id_old);
        assert_eq!(code, code_old);
    }

    #[tokio::test]
    async fn test_remove_code() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse("test@example.com".to_owned()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();

        store.add_code(email.clone(), login_attempt_id, code).await.unwrap();

        store.remove_code(&email).await.unwrap();

        let result = store.get_code(&email).await;

        assert!(result.is_err());
    }
}