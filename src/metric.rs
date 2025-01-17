/// This file is generated by scripts/generate_code.py
/// Please do not modify it directly.
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize, Default)]
#[allow(non_snake_case)]
#[serde(deny_unknown_fields)]
pub(crate) struct Metric {
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) archived: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) blocksDeleteOnBranches: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) blocksForcePushOnBranches: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) branchProtectionAppliesToAdmins: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) branchesAreProtected: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) codeApproved: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) codeReviewOneReviewers: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) contributorsFromOrgOrCompany: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) createdRecently: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) dependencyUpdateToolConfigured: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) dismissesStaleReviews: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) fuzzed: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) hasBinaryArtifacts: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) hasDangerousWorkflowScriptInjection: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) hasDangerousWorkflowUntrustedCheckout: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) hasFSFOrOSIApprovedLicense: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) hasLicenseFile: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) hasNoGitHubWorkflowPermissionUnknown: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) hasOSVVulnerabilities: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) hasOpenSSFBadge: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) hasPermissiveLicense: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) hasRecentCommits: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) hasReleaseSBOM: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) hasSBOM: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) hasUnverifiedBinaryArtifacts: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) issueActivityByProjectMember: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) jobLevelPermissions: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) packagedWithAutomatedWorkflow: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) pinsDependencies: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) releasesAreSigned: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) releasesHaveProvenance: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) releasesHaveVerifiedProvenance: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) requiresApproversForPullRequests: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) requiresCodeOwnersReview: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) requiresLastPushApproval: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) requiresPRsToChangeCode: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) requiresUpToDateBranches: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) runsStatusChecksBeforeMerging: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) sastToolConfigured: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) sastToolRunsOnAllCommits: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) securityPolicyContainsLinks: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) securityPolicyContainsText: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) securityPolicyContainsVulnerabilityDisclosure: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) securityPolicyPresent: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) testsRunInCI: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) topLevelPermissions: Option<f32>,
    #[serde(
        default,
        deserialize_with = "zero_to_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) webhooksUseSecrets: Option<f32>,
}

impl Metric {
    pub(crate) fn probes(&self) -> impl Iterator<Item = (&'static str, f32)> + '_ {
        [
            self.archived.map(|weight| ("archived", weight)),
            self.blocksDeleteOnBranches
                .map(|weight| ("blocksDeleteOnBranches", weight)),
            self.blocksForcePushOnBranches
                .map(|weight| ("blocksForcePushOnBranches", weight)),
            self.branchProtectionAppliesToAdmins
                .map(|weight| ("branchProtectionAppliesToAdmins", weight)),
            self.branchesAreProtected
                .map(|weight| ("branchesAreProtected", weight)),
            self.codeApproved.map(|weight| ("codeApproved", weight)),
            self.codeReviewOneReviewers
                .map(|weight| ("codeReviewOneReviewers", weight)),
            self.contributorsFromOrgOrCompany
                .map(|weight| ("contributorsFromOrgOrCompany", weight)),
            self.createdRecently
                .map(|weight| ("createdRecently", weight)),
            self.dependencyUpdateToolConfigured
                .map(|weight| ("dependencyUpdateToolConfigured", weight)),
            self.dismissesStaleReviews
                .map(|weight| ("dismissesStaleReviews", weight)),
            self.fuzzed.map(|weight| ("fuzzed", weight)),
            self.hasBinaryArtifacts
                .map(|weight| ("hasBinaryArtifacts", weight)),
            self.hasDangerousWorkflowScriptInjection
                .map(|weight| ("hasDangerousWorkflowScriptInjection", weight)),
            self.hasDangerousWorkflowUntrustedCheckout
                .map(|weight| ("hasDangerousWorkflowUntrustedCheckout", weight)),
            self.hasFSFOrOSIApprovedLicense
                .map(|weight| ("hasFSFOrOSIApprovedLicense", weight)),
            self.hasLicenseFile.map(|weight| ("hasLicenseFile", weight)),
            self.hasNoGitHubWorkflowPermissionUnknown
                .map(|weight| ("hasNoGitHubWorkflowPermissionUnknown", weight)),
            self.hasOSVVulnerabilities
                .map(|weight| ("hasOSVVulnerabilities", weight)),
            self.hasOpenSSFBadge
                .map(|weight| ("hasOpenSSFBadge", weight)),
            self.hasPermissiveLicense
                .map(|weight| ("hasPermissiveLicense", weight)),
            self.hasRecentCommits
                .map(|weight| ("hasRecentCommits", weight)),
            self.hasReleaseSBOM.map(|weight| ("hasReleaseSBOM", weight)),
            self.hasSBOM.map(|weight| ("hasSBOM", weight)),
            self.hasUnverifiedBinaryArtifacts
                .map(|weight| ("hasUnverifiedBinaryArtifacts", weight)),
            self.issueActivityByProjectMember
                .map(|weight| ("issueActivityByProjectMember", weight)),
            self.jobLevelPermissions
                .map(|weight| ("jobLevelPermissions", weight)),
            self.packagedWithAutomatedWorkflow
                .map(|weight| ("packagedWithAutomatedWorkflow", weight)),
            self.pinsDependencies
                .map(|weight| ("pinsDependencies", weight)),
            self.releasesAreSigned
                .map(|weight| ("releasesAreSigned", weight)),
            self.releasesHaveProvenance
                .map(|weight| ("releasesHaveProvenance", weight)),
            self.releasesHaveVerifiedProvenance
                .map(|weight| ("releasesHaveVerifiedProvenance", weight)),
            self.requiresApproversForPullRequests
                .map(|weight| ("requiresApproversForPullRequests", weight)),
            self.requiresCodeOwnersReview
                .map(|weight| ("requiresCodeOwnersReview", weight)),
            self.requiresLastPushApproval
                .map(|weight| ("requiresLastPushApproval", weight)),
            self.requiresPRsToChangeCode
                .map(|weight| ("requiresPRsToChangeCode", weight)),
            self.requiresUpToDateBranches
                .map(|weight| ("requiresUpToDateBranches", weight)),
            self.runsStatusChecksBeforeMerging
                .map(|weight| ("runsStatusChecksBeforeMerging", weight)),
            self.sastToolConfigured
                .map(|weight| ("sastToolConfigured", weight)),
            self.sastToolRunsOnAllCommits
                .map(|weight| ("sastToolRunsOnAllCommits", weight)),
            self.securityPolicyContainsLinks
                .map(|weight| ("securityPolicyContainsLinks", weight)),
            self.securityPolicyContainsText
                .map(|weight| ("securityPolicyContainsText", weight)),
            self.securityPolicyContainsVulnerabilityDisclosure
                .map(|weight| ("securityPolicyContainsVulnerabilityDisclosure", weight)),
            self.securityPolicyPresent
                .map(|weight| ("securityPolicyPresent", weight)),
            self.testsRunInCI.map(|weight| ("testsRunInCI", weight)),
            self.topLevelPermissions
                .map(|weight| ("topLevelPermissions", weight)),
            self.webhooksUseSecrets
                .map(|weight| ("webhooksUseSecrets", weight)),
        ]
        .into_iter()
        .flatten()
    }
}

fn zero_to_none<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Option::<f32>::deserialize(deserializer)?;
    Ok(match value {
        Some(0.0) => None,
        _ => value,
    })
}

impl std::fmt::Display for Metric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match toml::to_string(self) {
            Ok(toml_str) => write!(f, "{}", toml_str),
            Err(err) => write!(f, "Error serializing to TOML: {}", err),
        }
    }
}

#[cfg(test)]
pub(crate) static EXAMPLE_METRIC_STR: &str = r#"
archived = 0.1
blocksDeleteOnBranches = 0.2
blocksForcePushOnBranches = 0.3
branchProtectionAppliesToAdmins = 0.4
branchesAreProtected = 0.5
codeApproved = 0.6
codeReviewOneReviewers = 0.7
contributorsFromOrgOrCompany = 0.8
createdRecently = 0.9
dependencyUpdateToolConfigured = 1.0
dismissesStaleReviews = 1.1
fuzzed = 1.2
hasBinaryArtifacts = 1.3
hasDangerousWorkflowScriptInjection = 1.4
hasDangerousWorkflowUntrustedCheckout = 1.5
hasFSFOrOSIApprovedLicense = 1.6
hasLicenseFile = 1.7
hasNoGitHubWorkflowPermissionUnknown = 1.8
hasOSVVulnerabilities = 1.9
hasOpenSSFBadge = 2.0
hasPermissiveLicense = 2.1
hasRecentCommits = 2.2
hasReleaseSBOM = 2.3
hasSBOM = 2.4
hasUnverifiedBinaryArtifacts = 2.5
issueActivityByProjectMember = 2.6
jobLevelPermissions = 2.7
packagedWithAutomatedWorkflow = 2.8
pinsDependencies = 2.9
releasesAreSigned = 3.0
releasesHaveProvenance = 3.1
releasesHaveVerifiedProvenance = 3.2
requiresApproversForPullRequests = 3.3
requiresCodeOwnersReview = 3.4
requiresLastPushApproval = 3.5
requiresPRsToChangeCode = 3.6
requiresUpToDateBranches = 3.7
runsStatusChecksBeforeMerging = 3.8
sastToolConfigured = 3.9
sastToolRunsOnAllCommits = 4.0
securityPolicyContainsLinks = 4.1
securityPolicyContainsText = 4.2
securityPolicyContainsVulnerabilityDisclosure = 4.3
securityPolicyPresent = 4.4
testsRunInCI = 4.5
topLevelPermissions = 4.6
webhooksUseSecrets = 4.7
"#;

#[cfg(test)]
pub(crate) static EXAMPLE_METRIC: Metric = Metric {
    archived: Some(0.1),
    blocksDeleteOnBranches: Some(0.2),
    blocksForcePushOnBranches: Some(0.3),
    branchProtectionAppliesToAdmins: Some(0.4),
    branchesAreProtected: Some(0.5),
    codeApproved: Some(0.6),
    codeReviewOneReviewers: Some(0.7),
    contributorsFromOrgOrCompany: Some(0.8),
    createdRecently: Some(0.9),
    dependencyUpdateToolConfigured: Some(1.0),
    dismissesStaleReviews: Some(1.1),
    fuzzed: Some(1.2),
    hasBinaryArtifacts: Some(1.3),
    hasDangerousWorkflowScriptInjection: Some(1.4),
    hasDangerousWorkflowUntrustedCheckout: Some(1.5),
    hasFSFOrOSIApprovedLicense: Some(1.6),
    hasLicenseFile: Some(1.7),
    hasNoGitHubWorkflowPermissionUnknown: Some(1.8),
    hasOSVVulnerabilities: Some(1.9),
    hasOpenSSFBadge: Some(2.0),
    hasPermissiveLicense: Some(2.1),
    hasRecentCommits: Some(2.2),
    hasReleaseSBOM: Some(2.3),
    hasSBOM: Some(2.4),
    hasUnverifiedBinaryArtifacts: Some(2.5),
    issueActivityByProjectMember: Some(2.6),
    jobLevelPermissions: Some(2.7),
    packagedWithAutomatedWorkflow: Some(2.8),
    pinsDependencies: Some(2.9),
    releasesAreSigned: Some(3.0),
    releasesHaveProvenance: Some(3.1),
    releasesHaveVerifiedProvenance: Some(3.2),
    requiresApproversForPullRequests: Some(3.3),
    requiresCodeOwnersReview: Some(3.4),
    requiresLastPushApproval: Some(3.5),
    requiresPRsToChangeCode: Some(3.6),
    requiresUpToDateBranches: Some(3.7),
    runsStatusChecksBeforeMerging: Some(3.8),
    sastToolConfigured: Some(3.9),
    sastToolRunsOnAllCommits: Some(4.0),
    securityPolicyContainsLinks: Some(4.1),
    securityPolicyContainsText: Some(4.2),
    securityPolicyContainsVulnerabilityDisclosure: Some(4.3),
    securityPolicyPresent: Some(4.4),
    testsRunInCI: Some(4.5),
    topLevelPermissions: Some(4.6),
    webhooksUseSecrets: Some(4.7),
};
