use reqwest::{Client, StatusCode};
use serde::Deserialize;
use url::Url;

use crate::{
    config::{Config, Secret},
    error::SetuError,
};

// Joplin authenticates before resolving the requested note. This sentinel is
// deliberately not a valid Joplin item ID, so a valid token produces 404
// without returning a real item; an invalid token produces 403.
const AUTH_SENTINEL_NOTE_ID: &str = "setu-auth-check-not-a-note";
const NOTEBOOK_SEARCH_LIMIT: usize = 10;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AllowedNotebook {
    pub id: String,
    pub title: String,
    pub parent_id: String,
}

#[derive(Debug, Deserialize)]
struct NotebookSearchResponse {
    items: Vec<AllowedNotebook>,
    #[serde(default)]
    has_more: bool,
}

pub struct JoplinClient {
    base_url: Url,
    token: Option<Secret>,
    http: Client,
}

impl JoplinClient {
    pub fn new(config: &Config) -> Result<Self, SetuError> {
        let http = Client::builder()
            .timeout(config.request_timeout)
            .build()
            .map_err(|_| SetuError::JoplinConnection)?;

        Ok(Self {
            base_url: config.joplin_base_url.clone(),
            token: config.joplin_token.clone(),
            http,
        })
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    pub async fn ping(&self) -> Result<String, SetuError> {
        let url = self.endpoint("ping")?;
        let response = self
            .http
            .get(url)
            .send()
            .await
            .map_err(|_| SetuError::JoplinConnection)?;

        if !response.status().is_success() {
            return Err(SetuError::JoplinConnection);
        }

        response
            .text()
            .await
            .map_err(|_| SetuError::JoplinConnection)
    }

    pub async fn check_authentication_without_content(&self) -> Result<(), SetuError> {
        let token = self
            .token
            .as_ref()
            .ok_or(SetuError::JoplinAuthenticationFailed)?;
        let url = self.endpoint(&format!("notes/{AUTH_SENTINEL_NOTE_ID}"))?;
        let response = self
            .http
            .get(url)
            .query(&[("fields", "id"), ("token", token.expose())])
            .send()
            .await
            .map_err(|_| SetuError::JoplinConnection)?;

        match response.status() {
            StatusCode::NOT_FOUND => Ok(()),
            StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED => {
                Err(SetuError::JoplinAuthenticationFailed)
            }
            _ => Err(SetuError::UnexpectedJoplinResponse),
        }
    }

    pub async fn resolve_allowed_notebooks(
        &self,
        names: &[String],
    ) -> Result<Vec<AllowedNotebook>, SetuError> {
        let token = self
            .token
            .as_ref()
            .ok_or(SetuError::NotebookAllowlistRequiresToken)?;
        let mut resolved = Vec::with_capacity(names.len());

        for name in names {
            let url = self.endpoint("search")?;
            let response = self
                .http
                .get(url)
                .query(&[
                    ("query", name.as_str()),
                    ("type", "folder"),
                    ("fields", "id,title,parent_id"),
                    ("limit", "10"),
                    ("page", "1"),
                    ("token", token.expose()),
                ])
                .send()
                .await
                .map_err(|_| SetuError::JoplinConnection)?;

            if response.status() == StatusCode::FORBIDDEN
                || response.status() == StatusCode::UNAUTHORIZED
            {
                return Err(SetuError::JoplinAuthenticationFailed);
            }
            if !response.status().is_success() {
                return Err(SetuError::UnexpectedJoplinResponse);
            }

            let body: NotebookSearchResponse = response
                .json()
                .await
                .map_err(|_| SetuError::UnexpectedJoplinResponse)?;
            if body.has_more || body.items.len() > NOTEBOOK_SEARCH_LIMIT {
                return Err(SetuError::AllowedNotebookAmbiguous);
            }

            let exact: Vec<AllowedNotebook> = body
                .items
                .into_iter()
                .filter(|notebook| notebook.title.eq_ignore_ascii_case(name))
                .collect();

            match exact.as_slice() {
                [] => return Err(SetuError::AllowedNotebookNotFound),
                [notebook] => resolved.push(notebook.clone()),
                _ => return Err(SetuError::AllowedNotebookAmbiguous),
            }
        }

        Ok(resolved)
    }

    fn endpoint(&self, path: &str) -> Result<Url, SetuError> {
        self.base_url
            .join(path)
            .map_err(|_| SetuError::InvalidJoplinUrl)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use url::Url;
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers::method};

    use super::JoplinClient;
    use crate::{
        config::{Config, Secret},
        error::SetuError,
    };

    async fn client(token: &str) -> (MockServer, JoplinClient) {
        let server = MockServer::start().await;
        let config = Config {
            joplin_base_url: Url::parse(&server.uri()).unwrap(),
            joplin_token: Some(Secret::from_value(token.to_owned()).unwrap()),
            allowed_notebooks: Vec::new(),
            request_timeout: Duration::from_secs(1),
        };
        let client = JoplinClient::new(&config).unwrap();
        (server, client)
    }

    #[tokio::test]
    async fn accepts_not_found_as_authenticated_without_returning_content() {
        let (server, client) = client("fake-valid-token").await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(404).set_body_string("private-looking-body"))
            .mount(&server)
            .await;

        assert!(client.check_authentication_without_content().await.is_ok());
    }

    #[tokio::test]
    async fn maps_forbidden_to_sanitized_authentication_error() {
        let (server, client) = client("fake-rejected-token").await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(403).set_body_string("token=fake-rejected-token"))
            .mount(&server)
            .await;

        let error = client
            .check_authentication_without_content()
            .await
            .unwrap_err();
        assert!(matches!(error, SetuError::JoplinAuthenticationFailed));
        assert!(!error.to_string().contains("fake-rejected-token"));
    }

    #[tokio::test]
    async fn resolves_only_an_exact_notebook_match() {
        let (server, client) = client("fake-valid-token").await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [
                    { "id": "allowed-id", "title": "_setuDev", "parent_id": "" },
                    { "id": "other-id", "title": "_setuDev Archive", "parent_id": "" }
                ],
                "has_more": false
            })))
            .mount(&server)
            .await;

        let resolved = client
            .resolve_allowed_notebooks(&["_setuDev".to_owned()])
            .await
            .unwrap();
        assert_eq!(resolved.len(), 1);
        assert_eq!(resolved[0].id, "allowed-id");
    }

    #[tokio::test]
    async fn fails_closed_for_missing_or_ambiguous_notebooks() {
        let (missing_server, missing_client) = client("fake-valid-token").await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [], "has_more": false
            })))
            .mount(&missing_server)
            .await;
        assert!(matches!(
            missing_client
                .resolve_allowed_notebooks(&["_setuDev".to_owned()])
                .await,
            Err(SetuError::AllowedNotebookNotFound)
        ));

        let (ambiguous_server, ambiguous_client) = client("fake-valid-token").await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [
                    { "id": "one", "title": "_setuDev", "parent_id": "" },
                    { "id": "two", "title": "_SETUDEV", "parent_id": "" }
                ],
                "has_more": false
            })))
            .mount(&ambiguous_server)
            .await;
        assert!(matches!(
            ambiguous_client
                .resolve_allowed_notebooks(&["_setuDev".to_owned()])
                .await,
            Err(SetuError::AllowedNotebookAmbiguous)
        ));
    }
}
