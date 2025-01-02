# AUNOVIS Secure Sum

<img align="right" src="https://raw.githubusercontent.com/aunovis/secure_sum/main/img/secure_sam.svg" alt="Secure Sam, Secure Sum's mascot" width="200"/>

## About

Most modern software depends on numerous open source packages scattered over various ecosystems. A vulnerability deep within the dependency tree can potentially affect the whole supply chain. Which dependencies should you trust, and which should you rather avoid?

The [OSSF Scorecard](https://github.com/ossf/scorecard) project aims to answer that question. It analyses open source repositories with regard to various aspects of their security posture, and assigns a score between 0 and 10.

However, you may not agree with Scorecard's prioritisation of security aspects. Maybe a certain aspect is far more or less relevant to you than to the default algorithm. To quote an [article by the devlopers](https://openssf.org/blog/2024/04/17/beyond-scores-with-openssf-scorecard-granular-structured-results-for-custom-policy-enforcement/), "defining a security score from heuristics is an inherently opinionated process. [...] The current Scorecard output format lacks granularity for consumers to enable such custom risk evaluation."

To solve this issue, Scorecard has exposed the results of the various evaluations as machine-readable output. All that is left to do is to parse and combine them to a single score according to a customisable metric.

This is what AUNOVIS Secure Sum does.

## Setup

Most checks that scorecard runs require a personal access token (PAT) for the GitHub API. The [GitHub Docs](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens) explain how to generate one. We recommend a fine-grained access token with public-repo read-only access, as this is all that is needed by most scorecard checks.

When executed, scorecard looks for an environment variable called `GITHUB_TOKEN`. The fastest way to get it to work is to temporarily export the variable:
<details>
<summary>Unix</summary>

```bash
export GITHUB_TOKEN=<your PAT>
```

</details>

<details>
<summary>Powershell</summary>

```ps
$env:GITHUB_TOKEN="<your PAT>"
```

</details>

<details>
<summary>CMD</summary>

```cmd
set GITHUB_TOKEN=<your PAT>
```

</details>


## Usage

First, you have to define a metrics file. This tells AUNOVIS Secure Sum your priorities when evaluating projects. The file is written in [TOML format](https://toml.io/) and contains all probes you want to run, paired with a weight factor. The `system_tests` folder contains [an example file](https://github.com/aunovis/secure_sum/blob/main/system_tests/example_metrics.toml).

A list of all available probes can be found [in the scorecard repo](https://github.com/ossf/scorecard/tree/main/probes). They are kept up to date with [the corresponding rust file](https://github.com/aunovis/secure_sum/blob/main/src/metric.rs).

To run the analyses and apply the metrics, pass the metrics file as the first and the target as the second argument:
```
secure_sum <path/to/metrics/file> <target>
```

For example, to run Secure Sum against a single repository, run:
```
secure_sum example_metrics.toml https://github.com/aunovis/secure_sum
```
The URL has to start with `https://` or `http://`, otherwise Secure Sum will look for a lokal file.

## Known Issues

### 401 Bad Credentials

When running Secure Sum you may encounter the error message "repo unreachable: GET <URL>: 401 Bad Credentials". This error originates from the GitHub API.
Scorecard requires you to have a personal access token (PAT) for the GitHub API. Possible causes for this error message are:
- You do not have a PAT. Follow the [setup section](#setup) to create one.
- Your PAT expired.
- The email stored in your PAT does not correspond to the one in your `~/.gitconfig`. A solution is offered at the end of [ossf/scorecard#2559](https://github.com/ossf/scorecard/issues/2559).
- You have an expired token stored in a similar environment variable. Scorecard checks them in a specific order, and uses the first one (compare [ossf/scorecard#4475](https://github.com/ossf/scorecard/issues/4475)). The ideal solution is to find out what part of your system is exporting this old PAT. The quick and dirty solution is to export your new token as `GITHUB_AUTH_TOKEN`, because that is the first value that scorecard checks.
