use crate::domain::UserEmail;
use crate::domain::UserPassword;

pub struct NewUser {
    pub email: UserEmail,
    pub password: UserPassword,
}
