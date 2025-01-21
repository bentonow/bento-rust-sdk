use crate::{Client, CommandData, CommandResponse, Error, Result};
use tracing::instrument;

impl Client {
    /// Execute commands on subscribers
    ///
    /// # Arguments
    /// * `commands` - Vector of commands to execute
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    ///
    /// # Errors
    /// * `Error::InvalidRequest` if no commands are provided
    /// * `Error::InvalidEmail` if any email is invalid
    /// * `Error::InvalidRequest` if any command query is empty
    /// * `Error::InvalidCommand` if an invalid command type is provided
    /// * `Error::UnexpectedResponse` if the API returns an error
    #[instrument(skip(self))]
    pub async fn subscriber_command(&self, commands: Vec<CommandData>) -> Result<()> {
        if commands.is_empty() {
            return Err(Error::InvalidRequest("No commands provided".into()));
        }

        for command in &commands {
            if !command.email.contains('@') {
                return Err(Error::InvalidEmail(command.email.clone()));
            }
            if command.query.is_empty() {
                return Err(Error::InvalidRequest("Command query is required".into()));
            }
        }

        let url = self.build_url("/fetch/commands")?;
        let response = self.request(
            self.http_client
                .post(&url)
                .json(&serde_json::json!({
                    "command": commands
                }))
        ).await?;

        let command_response: CommandResponse = response.json().await?;

        if command_response.failed > 0 {
            return Err(Error::UnexpectedResponse(
                format!("Command execution partially failed: {} succeeded, {} failed",
                        command_response.results, command_response.failed)
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CommandType;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path, body_json};
    use serde_json::json;

    #[tokio::test]
    async fn test_subscriber_command() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/fetch/commands"))
            .and(body_json(json!({
                "command": [{
                    "command": "add_tag",
                    "email": "test@example.com",
                    "query": "new-tag"
                }]
            })))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(json!({
                    "results": 1,
                    "failed": 0
                })))
            .mount(&mock_server)
            .await;

        let client = crate::test_utils::create_test_client(mock_server.uri());

        let command = CommandData {
            command: CommandType::AddTag,
            email: "test@example.com".to_string(),
            query: "new-tag".to_string(),
        };

        let result = client.subscriber_command(vec![command]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_subscriber_command_partial_failure() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/fetch/commands"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(json!({
                    "results": 0,
                    "failed": 1
                })))
            .mount(&mock_server)
            .await;

        let client = crate::test_utils::create_test_client(mock_server.uri());

        let command = CommandData {
            command: CommandType::Subscribe,
            email: "test@example.com".to_string(),
            query: "subscribe".to_string(),
        };

        let result = client.subscriber_command(vec![command]).await;
        assert!(matches!(result, Err(Error::UnexpectedResponse(_))));
    }

    #[tokio::test]
    async fn test_invalid_command_input() {
        let mock_server = MockServer::start().await;
        let client = crate::test_utils::create_test_client(mock_server.uri());

        // Test empty commands
        let result = client.subscriber_command(vec![]).await;
        assert!(matches!(result, Err(Error::InvalidRequest(_))));

        // Test invalid email
        let command = CommandData {
            command: CommandType::AddTag,
            email: "invalid-email".to_string(),
            query: "test-tag".to_string(),
        };
        let result = client.subscriber_command(vec![command]).await;
        assert!(matches!(result, Err(Error::InvalidEmail(_))));

        // Test empty query
        let command = CommandData {
            command: CommandType::AddTag,
            email: "test@example.com".to_string(),
            query: "".to_string(),
        };
        let result = client.subscriber_command(vec![command]).await;
        assert!(matches!(result, Err(Error::InvalidRequest(_))));
    }
}