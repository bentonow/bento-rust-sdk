use serde::{Deserialize, Serialize};
use crate::{EmailData, Error};

/// Represents a batch of email messages for processing.
///
/// The `EmailBatch` struct contains a collection of email messages (`EmailData`),
/// with a restriction on the maximum number of emails allowed in a single batch (60).
///
/// # Examples
///
/// Creating a new email batch:
/// ```
/// # use your_crate::{EmailBatch, EmailData, Error};
/// let emails = vec![
///     EmailData {
///         to: "recipient@example.com".into(),
///         from: "sender@example.com".into(),
///         subject: "Hello!".into(),
///         html_body: "<p>Hello, world!</p>".into(),
///         transactional: true,
///         personalizations: None,
///     }
/// ];
///
/// let batch = EmailBatch::new(emails).expect("Failed to create email batch");
/// assert_eq!(batch.len(), 1);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailBatch {
    /// List of email messages
    pub emails: Vec<EmailData>,
}

impl EmailBatch {
    /// Create a new email batch
    ///
    /// # Errors
    /// Returns an error if the batch size exceeds 60 emails
    pub fn new(emails: Vec<EmailData>) -> crate::Result<Self> {
        if emails.len() > 60 {
            return Err(Error::InvalidBatchSize(
                "Maximum batch size is 60 emails".into()
            ));
        }
        Ok(Self { emails })
    }

    /// Add an email to the batch
    ///
    /// # Errors
    /// Returns an error if adding would exceed the maximum batch size
    pub fn add_email(&mut self, email: EmailData) -> crate::Result<()> {
        if self.emails.len() >= 60 {
            return Err(Error::InvalidBatchSize(
                "Maximum batch size is 60 emails".into()
            ));
        }
        self.emails.push(email);
        Ok(())
    }

    /// Get the number of emails in the batch
    pub fn len(&self) -> usize {
        self.emails.len()
    }

    /// Check if the batch is empty
    pub fn is_empty(&self) -> bool {
        self.emails.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_batch_creation() {
        let emails = vec![EmailData {
            to: "test@example.com".into(),
            from: "sender@example.com".into(),
            subject: "Test".into(),
            html_body: "<p>Test</p>".into(),
            transactional: true,
            personalizations: None,
        }];

        let batch = EmailBatch::new(emails);
        assert!(batch.is_ok());
        assert_eq!(batch.unwrap().len(), 1);
    }

    #[test]
    fn test_email_batch_size_limit() {
        let emails = (0..61).map(|_| EmailData {
            to: "test@example.com".into(),
            from: "sender@example.com".into(),
            subject: "Test".into(),
            html_body: "<p>Test</p>".into(),
            transactional: true,
            personalizations: None,
        }).collect();

        let batch = EmailBatch::new(emails);
        assert!(batch.is_err());
        assert!(matches!(batch.unwrap_err(), Error::InvalidBatchSize(_)));
    }
}
