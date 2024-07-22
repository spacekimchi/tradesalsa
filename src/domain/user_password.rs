use validator::{Validate, ValidationError};
use secrecy::{Secret, ExposeSecret};
use serde::{Serialize, Deserialize, Serializer};

// Wrapper type around Secret<String>
#[derive(Debug, Deserialize)]
pub struct SecretString(Secret<String>);

impl From<Secret<String>> for SecretString {
    fn from(secret: Secret<String>) -> Self {
        SecretString(secret)
    }
}

impl AsRef<Secret<String>> for SecretString {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}

// Implement Serialize for SecretString
impl Serialize for SecretString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.expose_secret())
    }
}

// Function to validate the password
fn validate_password(secret_password: &SecretString) -> Result<(), ValidationError> {
    let password = secret_password.as_ref().expose_secret();
    if password.len() < 8 {
        return Err(ValidationError::new("Password must be at least 8 characters long"));
    }
    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        return Err(ValidationError::new("Password must contain at least one lowercase letter"));
    }
    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        return Err(ValidationError::new("Password must contain at least one uppercase letter"));
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(ValidationError::new("Password must contain at least one digit"));
    }
    if !password.chars().any(|c| !c.is_alphanumeric()) {
        return Err(ValidationError::new("Password must contain at least one special character"));
    }
    Ok(())
}

#[derive(Debug, Validate)]
pub struct UserPassword {
    #[validate(custom(function = "validate_password"))]
    password: SecretString,
}

impl UserPassword {
    pub fn parse(s: Secret<String>) -> Result<UserPassword, String> {
        let secret_password = SecretString::from(s);
        let user_password = UserPassword {
            password: secret_password,
        };
        match user_password.validate() {
            Ok(_) => Ok(user_password),
            Err(_) => Err("Password is not a valid format.".to_string()),
        }
    }
}

impl AsRef<Secret<String>> for UserPassword {
    fn as_ref(&self) -> &Secret<String> {
        self.password.as_ref()
    }
}

impl AsRef<[u8]> for UserPassword {
    fn as_ref(&self) -> &[u8] {
        self.password.0.expose_secret().as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::UserPassword;
    use claims::assert_err;
    use secrecy::Secret;

    #[test]
    fn empty_string_is_rejected() {
        let password = "".to_string();
        assert_err!(UserPassword::parse(Secret::new(password)));
    }

    #[test]
    fn password_too_short_is_rejected() {
        let password = "Short1!".to_string();
        assert_err!(UserPassword::parse(Secret::new(password)));
    }

    #[test]
    fn password_missing_lowercase_is_rejected() {
        let password = "PASSWORD1!".to_string();
        assert_err!(UserPassword::parse(Secret::new(password)));
    }

    #[test]
    fn password_missing_uppercase_is_rejected() {
        let password = "password1!".to_string();
        assert_err!(UserPassword::parse(Secret::new(password)));
    }

    #[test]
    fn password_missing_digit_is_rejected() {
        let password = "Password!".to_string();
        assert_err!(UserPassword::parse(Secret::new(password)));
    }

    #[test]
    fn password_missing_special_character_is_rejected() {
        let password = "Password1".to_string();
        assert_err!(UserPassword::parse(Secret::new(password)));
    }

    #[derive(Debug, Clone)]
    struct ValidPasswordFixture(pub String);

    impl quickcheck::Arbitrary for ValidPasswordFixture {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            let password = "Valid1Password!".to_string(); // Replace with appropriate fake generation
            Self(password)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_passwords_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool {
        UserPassword::parse(Secret::new(valid_password.0)).is_ok()
    }
}

