use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct Inquiry {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
}

impl Inquiry {
    pub fn new(name: &str, email: &str, message: &str) -> Result<Self, &'static str> {
        let name = name.trim();
        let email = email.trim().to_lowercase();
        let message = message.trim();

        if name.is_empty() || name.len() > 120 {
            return Err("Name is required and must be 120 characters or fewer.");
        }
        if email.len() > 254 || !email.split_once('@').is_some_and(|(local, domain)| {
            !local.is_empty() && domain.contains('.') && !email.chars().any(char::is_whitespace)
        }) {
            return Err("A valid email address is required.");
        }
        if message.is_empty() || message.len() > 5000 {
            return Err("Message is required and must be 5000 characters or fewer.");
        }

        Ok(Self {
            id: Uuid::new_v4(),
            name: name.to_owned(),
            email,
            message: message.to_owned(),
            created_at: Utc::now(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Inquiry;

    #[test]
    fn inquiry_trims_fields_and_normalizes_email() {
        let inquiry = Inquiry::new(" Harish ", " USER@EXAMPLE.COM ", " Hello ").unwrap();
        assert_eq!(inquiry.name, "Harish");
        assert_eq!(inquiry.email, "user@example.com");
        assert_eq!(inquiry.message, "Hello");
    }

    #[test]
    fn inquiry_rejects_empty_or_invalid_fields() {
        assert!(Inquiry::new("", "user@example.com", "Hello").is_err());
        assert!(Inquiry::new("Harish", "not-an-email", "Hello").is_err());
        assert!(Inquiry::new("Harish", "user@example.com", "").is_err());
    }
}
