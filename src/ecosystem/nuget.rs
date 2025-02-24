use std::{fs, path::Path};

use serde::Deserialize;

use crate::{target::SingleTarget, Error};

use super::{DepFile, Ecosystem};

#[derive(Debug, Deserialize)]
pub(super) struct Csproj {
    #[serde(rename = "ItemGroup", default)]
    item_groups: Vec<ItemGroup>,
}

#[derive(Debug, Deserialize)]
struct ItemGroup {
    #[serde(rename = "PackageReference", default)]
    package_references: Vec<PackageReference>,
}

#[derive(Debug, Deserialize)]
struct PackageReference {
    #[serde(rename = "@Include", default)]
    include: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct PackagesConfig {
    #[serde(rename = "package", default)]
    packages: Vec<Package>,
}

#[derive(Debug, Deserialize)]
struct Package {
    #[serde(rename = "@id")]
    id: String,
}

impl DepFile for Csproj {
    fn ecosystem(&self) -> super::Ecosystem {
        Ecosystem::NuGet
    }

    fn first_level_deps(&self) -> Vec<SingleTarget> {
        self.item_groups
            .iter()
            .map(|p| {
                p.package_references
                    .iter()
                    .map(|dep| SingleTarget::Package(dep.include.to_owned(), self.ecosystem()))
            })
            .flatten()
            .collect()
    }
}

impl DepFile for PackagesConfig {
    fn ecosystem(&self) -> super::Ecosystem {
        Ecosystem::NuGet
    }

    fn first_level_deps(&self) -> Vec<SingleTarget> {
        self.packages
            .iter()
            .map(|dep| SingleTarget::Package(dep.id.to_owned(), self.ecosystem()))
            .collect()
    }
}

impl Csproj {
    pub(super) fn parse(file: &Path) -> Result<Self, Error> {
        let contents = fs::read_to_string(file)?;
        Self::parse_str(&contents)
    }

    fn parse_str(contents: &str) -> Result<Self, Error> {
        let depfile = quick_xml::de::from_str(contents)?;
        Ok(depfile)
    }
}

impl PackagesConfig {
    pub(super) fn parse(file: &Path) -> Result<Self, Error> {
        let contents = fs::read_to_string(file)?;
        Self::parse_str(&contents)
    }

    fn parse_str(contents: &str) -> Result<Self, Error> {
        let depfile = quick_xml::de::from_str(contents)?;
        Ok(depfile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn almost_empty_csproj_can_be_parsed() {
        let content = r#"<Project Sdk="Microsoft.NET.Sdk"></Project>"#;
        let result = Csproj::parse_str(&content);
        assert!(result.is_ok(), "{}", result.err().unwrap());
        let depfile = result.unwrap();
        assert!(depfile.item_groups.is_empty());
    }

    #[test]
    fn almost_empty_packages_config_can_be_parsed() {
        let content = r#"<?xml version="1.0" encoding="utf-8"?><packages/>"#;
        let result = PackagesConfig::parse_str(&content);
        assert!(result.is_ok(), "{}", result.err().unwrap());
        let depfile = result.unwrap();
        assert!(depfile.packages.is_empty());
    }

    #[test]
    fn small_csproj_can_be_parsed() {
        let content = r#"
<Project Sdk="Microsoft.NET.Sdk">
  <ItemGroup>
    <PackageReference Include="System.Xml.XPath.XmlDocument" Version="4.3.0" />
  </ItemGroup>
</Project>
    "#;
        let result = Csproj::parse_str(&content);
        assert!(result.is_ok(), "{}", result.err().unwrap());
        let depfile = result.unwrap();
        assert_eq!(depfile.item_groups.len(), 1);
        assert_eq!(depfile.item_groups[0].package_references.len(), 1);
        assert_eq!(
            depfile.item_groups[0].package_references[0].include,
            "System.Xml.XPath.XmlDocument"
        );
    }

    #[test]
    fn small_packages_config_can_be_parsed() {
        let content = r#"
<?xml version="1.0" encoding="utf-8"?>
<packages>
  <package id="Microsoft.Guardian.Cli" version="0.109.0"/>
</packages>
    "#;
        let result = PackagesConfig::parse_str(&content);
        assert!(result.is_ok(), "{}", result.err().unwrap());
        let depfile = result.unwrap();
        assert_eq!(depfile.packages.len(), 1);
        assert_eq!(depfile.packages[0].id, "Microsoft.Guardian.Cli");
    }

    #[test]
    fn csproj_with_several_item_groups_can_be_parsed() {
        let content = r#"
<Project Sdk="Microsoft.NET.Sdk">
    <ItemGroup>
        <PackageReference Include="Microsoft.SourceLink.GitHub" Version="$(MicrosoftSourceLinkGitHubPackageVersion)" PrivateAssets="All" />
    </ItemGroup>
    <ItemGroup Condition="'$(TargetFramework)' == 'netstandard1.3'">
        <PackageReference Include="System.Xml.XPath.XmlDocument" Version="4.3.0" />
    </ItemGroup>
</Project>
    "#;
        let result = Csproj::parse_str(&content);
        assert!(result.is_ok(), "{}", result.err().unwrap());
        let depfile = result.unwrap();
        assert_eq!(depfile.item_groups.len(), 2);
        assert_eq!(depfile.item_groups[0].package_references.len(), 1);
        assert_eq!(depfile.item_groups[1].package_references.len(), 1);
        assert_eq!(
            depfile.item_groups[0].package_references[0].include,
            "Microsoft.SourceLink.GitHub"
        );
        assert_eq!(
            depfile.item_groups[1].package_references[0].include,
            "System.Xml.XPath.XmlDocument"
        );
        assert_eq!(depfile.first_level_deps().len(), 2);
    }

    #[test]
    /// This input was not properly parsed by the original implementation.
    fn complicated_example_csproj_can_be_parsed() {
        let content = r#"
<Project Sdk="Microsoft.NET.Sdk">
  <ItemGroup>
    <PackageReference Include="Microsoft.CodeAnalysis.NetAnalyzers" Version="$(MicrosoftCodeAnalysisNetAnalyzersPackageVersion)" PrivateAssets="All" />
    <PackageReference Include="Microsoft.SourceLink.GitHub" Version="$(MicrosoftSourceLinkGitHubPackageVersion)" PrivateAssets="All" />
  </ItemGroup>
  <PropertyGroup Condition="'$(TargetFramework)' == 'netstandard1.0'">
    <AssemblyTitle>Json.NET .NET Standard 1.0</AssemblyTitle>
  </PropertyGroup>
  <ItemGroup Condition="'$(TargetFramework)' == 'netstandard1.3'">
    <PackageReference Include="System.Runtime.Serialization.Formatters" Version="$(SystemRuntimeSerializationFormattersPackageVersion)" />
    <PackageReference Include="System.Xml.XmlDocument" Version="$(SystemXmlXmlDocumentPackageVersion)" />
  </ItemGroup>
</Project>
    "#;
        let result = Csproj::parse_str(&content);
        assert!(result.is_ok(), "{}", result.err().unwrap());
        let depfile = result.unwrap();
        assert_eq!(depfile.item_groups.len(), 2);
        assert_eq!(depfile.first_level_deps().len(), 4);
    }
}
