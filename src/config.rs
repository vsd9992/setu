use std::{collections::HashSet, env, net::IpAddr, time::Duration};

use url::{Host, Url};

use crate::error::SetuError;

const DEFAULT_JOPLIN_BASE_URL: &str = "http://127.0.0.1:41184";
const JOPLIN_BASE_URL_ENV: &str = "SETU_JOPLIN_BASE_URL";
const JOPLIN_TOKEN_ENV: &str = "JOPLIN_TOKEN";
const ALLOWED_NOTEBOOKS_ENV: &str = "SETU_ALLOWED_NOTEBOOKS";
const MAX_ALLOWED_NOTEBOOKS: usize = 20;
const MAX_NOTEBOOK_NAME_CHARS: usize = 255;

#[derive(Clone)]
pub struct Secret(String);

impl Secret {
    pub(crate) fn from_value(value: String) -> Result<Self, SetuError> {
        if value.trim().is_empty() {
            return Err(SetuError::InvalidJoplinToken);
        }

        Ok(Self(value))
    }

    pub(crate) fn expose(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for Secret {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("[REDACTED]")
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub joplin_base_url: Url,
    pub joplin_token: Option<Secret>,
    pub allowed_notebooks: Vec<String>,
    pub request_timeout: Duration,
}

impl Config {
    pub fn from_env() -> Result<Self, SetuError> {
        let value =
            env::var(JOPLIN_BASE_URL_ENV).unwrap_or_else(|_| DEFAULT_JOPLIN_BASE_URL.to_owned());

        let token = env::var(JOPLIN_TOKEN_ENV)
            .ok()
            .map(Secret::from_value)
            .transpose()?;
        let allowed_notebooks = env::var(ALLOWED_NOTEBOOKS_ENV)
            .ok()
            .map(|value| parse_allowed_notebooks(&value))
            .transpose()?
            .unwrap_or_default();

        Self::from_values(&value, token, allowed_notebooks)
    }

    #[cfg(test)]
    fn from_base_url(value: &str) -> Result<Self, SetuError> {
        Self::from_values(value, None, Vec::new())
    }

    fn from_values(
        value: &str,
        token: Option<Secret>,
        allowed_notebooks: Vec<String>,
    ) -> Result<Self, SetuError> {
        let mut url = Url::parse(value).map_err(|_| SetuError::InvalidJoplinUrl)?;

        if url.scheme() != "http" || !is_loopback(url.host()) {
            return Err(SetuError::UnsafeJoplinUrl);
        }

        if url.query().is_some() || url.fragment().is_some() || !url.username().is_empty() {
            return Err(SetuError::InvalidJoplinUrl);
        }

        if url.password().is_some() {
            return Err(SetuError::InvalidJoplinUrl);
        }

        let normalized_path = url.path().trim_end_matches('/').to_owned();
        url.set_path(&normalized_path);

        Ok(Self {
            joplin_base_url: url,
            joplin_token: token,
            allowed_notebooks,
            request_timeout: Duration::from_secs(3),
        })
    }
}

fn parse_allowed_notebooks(value: &str) -> Result<Vec<String>, SetuError> {
    let names: Vec<String> = value
        .split(',')
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
        .collect();

    if names.is_empty()
        || names.len() > MAX_ALLOWED_NOTEBOOKS
        || names
            .iter()
            .any(|name| name.chars().count() > MAX_NOTEBOOK_NAME_CHARS)
    {
        return Err(SetuError::InvalidNotebookAllowlist);
    }

    let mut unique = HashSet::new();
    if names.iter().any(|name| !unique.insert(name.to_lowercase())) {
        return Err(SetuError::InvalidNotebookAllowlist);
    }

    Ok(names)
}

fn is_loopback(host: Option<Host<&str>>) -> bool {
    match host {
        Some(Host::Ipv4(address)) => address.is_loopback(),
        Some(Host::Ipv6(address)) => address.is_loopback(),
        Some(Host::Domain(domain)) => {
            domain.eq_ignore_ascii_case("localhost")
                || domain
                    .parse::<IpAddr>()
                    .is_ok_and(|address| address.is_loopback())
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, Secret, parse_allowed_notebooks};

    #[test]
    fn accepts_loopback_http_urls() {
        for value in [
            "http://127.0.0.1:41184",
            "http://localhost:41184/",
            "http://[::1]:41184",
        ] {
            assert!(Config::from_base_url(value).is_ok(), "rejected {value}");
        }
    }

    #[test]
    fn rejects_non_loopback_or_credential_bearing_urls() {
        for value in [
            "https://127.0.0.1:41184",
            "http://192.168.1.10:41184",
            "http://example.com:41184",
            "http://user:secret@127.0.0.1:41184",
            "http://127.0.0.1:41184?token=secret",
        ] {
            assert!(Config::from_base_url(value).is_err(), "accepted {value}");
        }
    }

    #[test]
    fn secret_debug_output_is_redacted() {
        let secret = Secret::from_value("fake-test-token".to_owned()).unwrap();
        assert_eq!(format!("{secret:?}"), "[REDACTED]");
    }

    #[test]
    fn rejects_empty_tokens() {
        assert!(Secret::from_value("   ".to_owned()).is_err());
    }

    #[test]
    fn parses_a_bounded_notebook_allowlist() {
        assert_eq!(
            parse_allowed_notebooks("_setuDev, Setu Archive").unwrap(),
            vec!["_setuDev", "Setu Archive"]
        );
    }

    #[test]
    fn rejects_empty_or_duplicate_notebook_allowlists() {
        assert!(parse_allowed_notebooks(" , ").is_err());
        assert!(parse_allowed_notebooks("_setuDev,_SETUDEV").is_err());
    }
}
