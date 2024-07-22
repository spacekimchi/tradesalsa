// use secrecy::ExposeSecret;
// use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::ContentType;
use std::collections::HashMap;
use tera::{Context, Tera};
use crate::configuration::EmailSettings;

pub async fn send_email(
    to: &str,
    subject: &str,
    template_name: &str,
    context: &HashMap<&str, &str>,
    tera: &Tera,
    email_settings: &EmailSettings,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tera_context = Context::new();
    for (key, value) in context {
        tera_context.insert(*key, value);
    }

    let email_body = tera.render(template_name, &tera_context)?;

    let email = Message::builder()
        .from(email_settings.admin_email.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(email_body)?;

    // let creds = Credentials::new(email_settings.smtp_username.into(),
    // email_settings.smtp_password.expose_secret().into());
    // let mailer = SmtpTransport::relay(email_settings.admin_email)?
    //     .credentials(creds)
    //     .build();
    // Configure the local Python SMTP debugging server as the SMTP server
    let mailer = SmtpTransport::builder_dangerous(&email_settings.smtp_host)
        .port(email_settings.smtp_port)
        .build();

    mailer.send(&email)?;

    Ok(())
}

