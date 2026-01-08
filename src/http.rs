use ureq::{Agent, Body, http::Response};

use crate::error::Error;

static USER_AGENT_HEADER: &str = "User-Agent";
static USER_AGENT: &str = "secure_sum (info@aunovis.de)";
static AUTH_HEADER: &str = "Authorization";

pub(crate) fn http_get(url: &str, auth_header: Option<&str>) -> Result<Response<Body>, Error> {
    let agent: Agent = Agent::config_builder()
        .http_status_as_error(true)
        .https_only(true)
        .build()
        .into();
    let mut builder = agent.get(url).header(USER_AGENT_HEADER, USER_AGENT);
    if let Some(auth) = auth_header {
        builder = builder.header(AUTH_HEADER, auth);
    }
    builder.call().map_err(Into::into)
}
