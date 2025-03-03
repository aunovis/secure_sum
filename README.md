# AUNOVIS Secure Sum

<img align="right" src="https://raw.githubusercontent.com/aunovis/secure_sum/main/img/secure_sam.svg" alt="Secure Sam, Secure Sum's mascot" width="200"/>

## About

Most modern software depends on numerous open source packages scattered over various ecosystems. A vulnerability deep within the dependency tree can potentially affect the whole supply chain. Which dependencies should you trust, and which should you rather avoid?

The [OSSF Scorecard](https://github.com/ossf/scorecard) project aims to answer that question. It analyses open source repositories with regard to various aspects of their security posture, and assigns a score between 0 and 10.

However, you may not agree with Scorecard's prioritisation of security aspects. Maybe a certain aspect is far more or less relevant to you than to the default algorithm. To quote an [article by the devlopers](https://openssf.org/blog/2024/04/17/beyond-scores-with-openssf-scorecard-granular-structured-results-for-custom-policy-enforcement/), "defining a security score from heuristics is an inherently opinionated process. [...] The current Scorecard output format lacks granularity for consumers to enable such custom risk evaluation."

To solve this issue, Scorecard has exposed the results of the various evaluations as machine-readable output. All that is left to do is to parse and combine them to a single score according to a customisable metric.

This is what Secure Sum does.

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
<summary>Powershell/CMD</summary>

```cmd
set GITHUB_TOKEN=<your PAT>
```

</details>

A more persistent way is to write create a fiel called `.env` with the content `GITHUB_TOKEN=<your PAT>` in the folder from which you are calling Secure Sum.

> If you create a `.env` file inside a repository, do not forget to add it to your `.gitignore` file (or the analogous ignore file for your versioning ecosystem). You will leak secrets otherwise!


## Usage

### Metric File

First, you have to define a metric file. This tells Secure Sum your priorities when evaluating projects. The file is written in [TOML format](https://toml.io/) and contains:
- the names of all [probes](#probe-name) you want to run
- a [weight](#weight) factor
- an optional [maximum number of times](#max-times) this probe can contribute to the overall score.

Every entry in the metric file looks like this:
```toml
[[probe]]
name = "<name of the probe>"
weight = <any real number>
max_times = <any natural number> # Optional
```
The `system_tests` folder contains an [example metric file](https://github.com/aunovis/secure_sum/blob/main/system_tests/example_metric.toml) that is used in the tests.

#### Probe Name

A probe is some kind of check with an outcome that is either true, false, or a variation of "the probe check didn't work properly". The "fuzzed" probe for example checks if a repository is automatically fuzzed. The "archived" probe checks whether or not the repository is marked as archived.

Several probes can return their outcome several times. The "testsRunInCI" probe for example returns either false a single time, or true for every CI test that it discovers.

A list of all available probes can be found in the [scorecard repo](https://github.com/ossf/scorecard/tree/main/probes). The [corresponding rust file](https://github.com/aunovis/secure_sum/blob/main/src/probe_name.rs) is automatically kept up to date with it.

#### Weight

Weight factors can be any real number. Positive numbers should be used for qualities that are good to have (for example being fuzzed), negative numbers for qualities that are bad to have (for example being archived).

```toml
[[probes]]
name = "archived"
weight = -1

[[probes]]
name = "fuzzed"
weight = 1
```

A weight of zero is equivalent to omitting the probe from the metric file, the probe is not run in this case.

#### Max Times

Because several probes produce one outcome per finding, you need to be able to specify a maximum number of times this probe is allowed to contribute to the overall score. Otherwise, a repository with 10 known Vulnerabilities and 100 recent CI test runs could have a stellar score. Limiting the number of times the CI test run can contribute to for example 1 or 2 draws a much more realistic picture.

```toml
[[probes]]
name = "hasOSVVulnerabilities"
weight = -1

[[probes]]
name = "testsRunInCI"
weight = 1
max_times = 2
```

### Algorithm

Secure Sum's algorithm for calculating the total score is as follows:
1. Run all probes.
2. Keep only those probes whose outcome is either true or false.
3. Calculate the lowest weighed sum possible with these probes, by summing up all negative weights. Probes with several outcomes contribute several times, up to their `max_times` value, if specified.
4. Analogously calculate the highest weighed sum possible.
5. Calculate the actual result by summing all weights of probes with outcome true.
6. Linearly transform the three numbers such that the lowest possible value is mapped to 0, and the highest value to 10. The actual result will be in this interval.

As an example, consider the following metric:
```toml
[[probes]]
name = "archived"
weight = -1

[[probes]]
name = "hasOSVVulnerabilities"
weight = -2

[[probes]]
name = "hasDangerousWorkflowScriptInjection"
weight = -1

[[probes]]
name = "testsRunInCI"
weight = 1
max_times = 3
```
1. Our imaginary repo is not archived, has 3 known vulnerabilities and 4 tests run in CI. The dangerous workflow probe returns "NotApplicable".
2. The dangerous workflow probe is ignored in what follows.
3. The lowes possible weighed sum is -7, which is the weight of "archived" plus three times the weight for "hasOSVVulnerabilities".
4. The highest possible weighed sum is 3, because although the repo has 4 CI tests, it was restricted to contribute a maximum number of 3 times.
5. The actual result is -6+3=-3, which comes from 3 Vulnerabilities and 3 CI tests, the probes that returned true.
6. All values are shifted by 7, so that the final result is in the range 0 to 10. The total score of our repo thus becomes 4.0 of 10.

This algorithm is a choice. If you yould like Secure Sum to be configurable to use another algorithm, please [create an issue](https://github.com/aunovis/secure_sum/issues) and we will see what we can do.

### Program Call

To run the analyses and apply the metric, pass the metric file as the first and the target(s) as the second, third, etc. argument(s):
```
secure_sum <path/to/metric/file> <target> <additional-targts...>
```
The targets do not necessarily have to be from the same ecosystem.

For example, to run Secure Sum against a single repository, run:
```
secure_sum example_metric.toml https://github.com/aunovis/secure_sum
```
The URL of the target has to start with `https://` or `http://`, otherwise Secure Sum will look for a local file.

To run Secure Sum against the Rust ecosystem, target the Cargo.toml file:
```
secure_sum example_metric.toml Cargo.toml
```
It will then collect all first level dependencies and analyse them.

If a check containing the required metric has been run for a repository within the last week, Secure Sum will use the locally stored results. To overwrite this behavioiur and enforce a complete re-evaluation, you can use the `--rerun` flag.
```
secure_sum example_metric.toml https://github.com/aunovis/secure_sum --rerun
```

### Supported Ecosystems

- **Node.js:** Provide a `package.json` file.
- **NuGet:** 
  - Provide all `.csproj` XML files at your disposal, for example by using `$(find . -iname "*.csproj")` as an argument.
  - Alternatively (or additionally, really), provide a `packages.configs` XML file.
- **Rust:** Provide a `Cargo.toml` file.

Is your favourite ecosystem missing? Create an [issue](https://github.com/aunovis/secure_sum/issues) and we'll see what we can do about that.

## Known Issues

### 401 Bad Credentials

When running Secure Sum you may encounter the error message "repo unreachable: GET <URL>: 401 Bad Credentials". This error originates from the GitHub API.
Scorecard requires you to have a personal access token (PAT) for the GitHub API. Possible causes for this error message are:
- You do not have a PAT. Follow the [setup section](#setup) to create one.
- Your PAT expired.
- The email stored in your PAT does not correspond to the one in your `~/.gitconfig`. A solution is offered at the end of [ossf/scorecard#2559](https://github.com/ossf/scorecard/issues/2559).
- You have an expired token stored in a similar environment variable. Scorecard checks them in a specific order, and uses the first one (compare [ossf/scorecard#4475](https://github.com/ossf/scorecard/issues/4475)). The ideal solution is to find out what part of your system is exporting this old PAT. The quick and dirty solution is to export your new token as `GITHUB_AUTH_TOKEN`, because that is the first value that scorecard checks.
