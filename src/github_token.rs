use crate::Error;

pub(crate) fn ensure_valid_github_token() -> Result<(), Error> {
    let token = get_token()?;
    check_token_validity(&token)
}

fn get_token() -> Result<String, Error> {
    dotenvy::dotenv().ok();
    todo!()
}

fn check_token_validity(token: &str) -> Result<(), Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;

    static GITHUB_TOKEN_NAME: &str = "GITHUB_TOKEN";
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
    fn get_token_reads_dotenv() {
        unsafe {
            std::env::remove_var(GITHUB_TOKEN_NAME);
        }
        assert!(!get_token().unwrap().is_empty());
    }

    #[test]
    #[serial]
    fn valid_token_is_valid() {
        let token = get_token().unwrap();
        let result = check_token_validity(&token);
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn invalid_token_is_invalid() {
        let result = check_token_validity(TOY_TOKEN);
        assert!(result.is_err());
    }
}
