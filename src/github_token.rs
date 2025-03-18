use crate::Error;

pub(crate) static GITHUB_TOKEN_NAME: &str = "GITHUB_TOKEN";
pub(crate) static USER_AGENT_HEADER: &str = "User-Agent";
pub(crate) static USER_AGENT: &str = "secure_sum (info@aunovis.de)";

pub(crate) fn ensure_valid_github_token() -> Result<(), Error> {
    let token = get_token()?;
    check_token_validity(&token)
}

fn get_token() -> Result<String, Error> {
    dotenvy::dotenv().ok();
    std::env::var(GITHUB_TOKEN_NAME).map_err(|e| {
        let message = format!("Error reading {GITHUB_TOKEN_NAME} from environment: {e}");
        Error::Other(message)
    })
}

fn check_token_validity(token: &str) -> Result<(), Error> {
    static ENDPOINT: &str = "https://api.github.com/rate_limit";
    static AUTH_HEADER: &str = "Authorization";
    let header_value = format!("token {token}");
    let response = reqwest::blocking::Client::new()
        .get(ENDPOINT)
        .header(AUTH_HEADER, header_value)
        .header(USER_AGENT_HEADER, USER_AGENT)
        .send()?
        .error_for_status();
    match response {
        Ok(_) => Ok(()),
        Err(e) => {
            let message = format!("Your GITHBU_TOKEN is apparently not valid: {e}");
            Err(Error::Other(message))
        }
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;

    static TOY_TOKEN: &str = "github_please_trust_me_i_am_a_token";

    #[test]
    #[serial]
    fn get_token_reads_token_from_env() {
        unsafe {
            std::env::set_var(GITHUB_TOKEN_NAME, TOY_TOKEN);
        }
        assert_eq!(get_token().unwrap(), TOY_TOKEN);
        unsafe {
            std::env::remove_var(GITHUB_TOKEN_NAME);
        }
    }

    #[test]
    #[serial]
    /// For this test to work, a .env file containing the GITHUB_TOKEN needs to exists.
    fn get_token_reads_dotenv_file() {
        unsafe {
            std::env::remove_var(GITHUB_TOKEN_NAME);
        }
        assert!(!get_token().unwrap().is_empty());
    }

    #[test]
    #[serial]
    /// For this test to work, a .env file containing the GITHUB_TOKEN needs to exists.
    fn valid_token_is_valid() {
        let token = get_token().unwrap();
        let result = check_token_validity(&token);
        assert!(result.is_ok(), "{}", result.unwrap_err());
    }

    #[test]
    #[serial]
    fn invalid_token_is_invalid() {
        let result = check_token_validity(TOY_TOKEN);
        assert!(result.is_err(), "{}", result.unwrap_err());
    }
}
