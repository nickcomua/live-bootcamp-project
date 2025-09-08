use validator::validate_email;
#[cfg(test)]
use quickcheck::Arbitrary;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email {
    email: String,
}

#[derive(Debug, Clone)]
pub struct EmailValidationError;

impl Email {
    pub fn parse(email: String) -> Result<Self, EmailValidationError> {
        if validate_email(&email) {
            Ok(Self { email })
        } else {
            Err(EmailValidationError)
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.email
    }
}

#[cfg(test)]
impl Arbitrary for Email {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        use fake::{Fake, faker::internet::en::FreeEmail};

        Self::parse(FreeEmail().fake_with_rng(g)).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use crate::domain::Email;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn user_creation_test(email: Email) -> bool {
        email.as_ref().contains('@')
    }

    #[test]
    fn email_parsing_test() {
        let email = Email::parse("nick@live.com".to_string()).unwrap();
        assert_eq!(email.as_ref(), "nick@live.com");

        assert!(Email::parse("nick@".to_string()).is_err());
        assert!(Email::parse("@live.com".to_string()).is_err());
        assert!(Email::parse("nick".to_string()).is_err());
    }
}
