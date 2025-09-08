#[cfg(test)]
use quickcheck::Arbitrary;

#[derive(Debug, Clone)]
pub struct PasswordValidationError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password {
    password: String,
}

impl Password {
    pub fn parse(password: String) -> Result<Self, PasswordValidationError> {
        if password.len() < 8 {
            return Err(PasswordValidationError);
        }
        Ok(Self { password })
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.password
    }
}

#[cfg(test)]
impl Arbitrary for Password {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        use rand::{Rng, SeedableRng, seq::IndexedRandom};

        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz\
                                 ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                 0123456789\
                                 !@#$%^&*()-_=+[]{};:,.<>?/";
        let mut rng = rand::rngs::StdRng::seed_from_u64(g.next_u64());
        let len = rng.random_range(8..=20);
        let password: String = (0..len)
            .map(|_| *CHARSET.choose(&mut rng).unwrap() as char)
            .collect();
        Self::parse(password).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Password;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn password_test(password: Password) -> bool {
        password.as_ref().len() >= 8
    }

    #[test]
    fn password_parsing_test() {
        let password = Password::parse("password".to_string()).unwrap();
        assert_eq!(password.as_ref(), "password");

        assert!(Password::parse("passwor".to_string()).is_err());
        assert!(Password::parse("pass".to_string()).is_err());
        assert!(Password::parse("pass123".to_string()).is_err());
    }
}
