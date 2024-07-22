//! src/constants.rs
//! This file defines all the constants used throughout the application.
//! Define things here to make life easy
//!

/// Template constants
pub mod html_templates {
    pub const REGISTER: &str = "register.html";
    pub const LOGIN: &str = "login.html";
    pub const HOMEPAGE: &str = "homepage.html";
    pub const E500: &str = "500.html";
}

/// email templates
pub mod email_templates {
    pub const EMAIL_VERIFICATION: &str = "emails/email_verification.html";
}

/// Strings
pub mod strings {
    pub const WELCOME_EMAIL_SUBJECT: &str = "Welcome to TradeSalsa!";
    pub const INTERNAL_SERVER_ERROR: &str = "Internal Server Error";
    pub const REGISTER_ACCOUNT_SUCCESS: &str = "Successfully registered account!";
    pub const INVALID_CREDENTIALS: &str = "Invalid Credentials";
    pub const FAILED_TO_COMPILE_SCSS: &str = "Failed to compile SCSS";
    pub const FAILED_TO_WRITE_SCSS: &str = "Failed to write SCSS";
}

/// paths
pub mod route_paths {
    pub const ROOT: &str = "/";
    pub const REGISTER: &str = "/register";
    pub const LOGIN: &str = "/login";
    pub const LOGOUT: &str = "/logout";
    pub const HEALTH: &str = "/health";
    pub const PROTECTED: &str = "/protected";
}

