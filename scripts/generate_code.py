import os
import requests

# GitHub repository details
owner = "ossf"
repo = "scorecard"
path = "probes"
url = f"https://api.github.com/repos/{owner}/{repo}/contents/{path}"

# GitHub API requires a User-Agent header
headers = {
    "User-Agent": "Python-Directory-Fetcher"
}

TARGET_PATH = os.path.join("src", "metric.rs")

TEMPLATE = """
/// This file is generated by scripts/generate_code.py
/// Please do not modify it directly.

use serde::{{Deserialize, Serialize}};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[allow(non_snake_case)]
#[serde(deny_unknown_fields)]
pub(crate) struct Metric {{
    {members}
}}

fn zero_to_none<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: serde::Deserializer<'de>,
{{
    let value = Option::<f32>::deserialize(deserializer)?;
    Ok(match value {{
        Some(0.0) => None,
        _ => value,
    }})
}}

impl std::fmt::Display for Metric {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        match toml::to_string(self) {{
            Ok(toml_str) => write!(f, "{{}}", toml_str),
            Err(err) => write!(f, "Error serializing to TOML: {{}}", err),
        }}
    }}
}}

#[cfg(test)]
pub(crate) static EXAMPLE_METRIC_STR: &str = r#"
{member_assignements}
"#;

#[cfg(test)]
pub(crate) static EXAMPLE_METRIC: Metric = Metric {{
    {assigned_members}
}};
"""

MEMBER_PRELUDE = "#[serde(default, deserialize_with = \"zero_to_none\", skip_serializing_if = \"Option::is_none\")] pub(crate)"
MEMBER_TYPE = ": Option<f32>"

def get_probes(url):
    try:
        # Send a GET request to GitHub API
        response = requests.get(url, headers=headers)
        response.raise_for_status()  # Raise error for HTTP issues
        
        # Parse the JSON response
        contents = response.json()
        
        # Filter directories (type == "dir")
        directories = [item['name'] for item in contents if item['type'] == 'dir']

        return directories
    except requests.exceptions.RequestException as e:
        print(f"Error: {e}")

def construct_members_string(probes):
    members = [f"{MEMBER_PRELUDE}{probe}{MEMBER_TYPE}" for probe in probes]
    return ",".join(members)

def assign_test_values(probes):
    return [((index + 1)/10, probe) for index, probe in enumerate(probes)]

def construct_member_assignemnt_string(assigned_probes):
    assignements = [f"{probe} = {val}" for val, probe in assigned_probes]
    return "\n".join(assignements)

def construct_assigned_members_string(assigned_probes):
    assignements = [f"{probe}: Some({val})" for val, probe in assigned_probes]
    return ",".join(assignements)

probes = get_probes(url)
assigned_probes = assign_test_values(probes)
members = construct_members_string(probes)
member_assignments = construct_member_assignemnt_string(assigned_probes)
assigned_members = construct_assigned_members_string(assigned_probes)
with open(TARGET_PATH, 'w') as metric_file:
    metric_file.write(TEMPLATE.format(members = members, 
                                      member_assignements = member_assignments, 
                                      assigned_members = assigned_members))
