# AUNOVIS Secure Sum

## About

Most modern software depends on numerous open source packages scattered over various ecosystems. A vulnerability deep within the dependency tree can potentially affect the whole supply chain. Which dependencies should you trust, and which should you rather avoid?

The [OSSF Scorecard](https://github.com/ossf/scorecard) project aims to answer that question. It analyses open source repositories with regard to various aspects of their security posture, and assigns a score between 0 and 10.

However, you may not agree with Scorecard's prioritisation of security aspects. Maybe a certain aspect is far more or less relevant to you than to the default algorithm. To quote an [article by the devlopers](https://openssf.org/blog/2024/04/17/beyond-scores-with-openssf-scorecard-granular-structured-results-for-custom-policy-enforcement/), "defining a security score from heuristics is an inherently opinionated process. [...] The current Scorecard output format lacks granularity for consumers to enable such custom risk evaluation."

To solve this issue, Scorecard has exposed the results of the various evaluations as machine-readable output. All that is left to do is to parse and combine them to a single score according to a customisable metric.

This is what AUNOVIS Secure Sum does.
