use std::{fs, path::Path};

use serde::Deserialize;

use crate::{target::SingleTarget, Error};

use super::{DepFile, Ecosystem};

#[derive(Debug, Deserialize)]
pub(super) struct Csproj {
    #[serde(rename = "$value", default)]
    elements: Vec<CsprojElement>,
}

#[derive(Debug, Deserialize)]
enum CsprojElement {
    #[serde(rename = "ItemGroup")]
    ItemGroup(ItemGroup),
    #[serde(other)]
    Other,
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
        self.elements
            .iter()
            .filter_map(|e| match e {
                CsprojElement::ItemGroup(item_group) => Some(item_group),
                CsprojElement::Other => None,
            })
            .flat_map(|group| {
                group
                    .package_references
                    .iter()
                    .map(|dep| SingleTarget::Package(dep.include.to_owned(), self.ecosystem()))
            })
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
        assert!(result.is_ok(), "{}", result.unwrap_err());
        let depfile = result.unwrap();
        assert!(depfile.elements.is_empty());
    }

    #[test]
    fn almost_empty_packages_config_can_be_parsed() {
        let content = r#"<?xml version="1.0" encoding="utf-8"?><packages/>"#;
        let result = PackagesConfig::parse_str(&content);
        assert!(result.is_ok(), "{}", result.unwrap_err());
        let depfile = result.unwrap();
        assert!(depfile.packages.is_empty());
    }

    #[test]
    fn item_group_can_be_deserialized() {
        let content = r#"
<ItemGroup>
  <PackageReference Include="System.Xml.XPath.XmlDocument" Version="4.3.0" />
</ItemGroup>
    "#;
        let result = quick_xml::de::from_str::<ItemGroup>(&content);
        assert!(result.is_ok(), "{}", result.unwrap_err());
        let group = result.unwrap();
        assert!(!group.package_references.is_empty(), "{:#?}", group);
    }

    #[test]
    fn element_can_deserialize_anything() {
        let content = r#"
<ItemGroup>
  <PackageReference Include="System.Xml.XPath.XmlDocument" Version="4.3.0" />
</ItemGroup>
    "#;
        let result = quick_xml::de::from_str::<CsprojElement>(&content);
        assert!(result.is_ok(), "{}", result.unwrap_err());
        let element = result.unwrap();
        assert!(
            matches!(element, CsprojElement::ItemGroup(_)),
            "{:#?}",
            element
        );
        let content = r#"
<SomethingElse>
  <PackageReference Include="System.Xml.XPath.XmlDocument" Version="4.3.0" />
</SomethingElse>
    "#;
        let result = quick_xml::de::from_str::<CsprojElement>(&content);
        assert!(result.is_ok(), "{}", result.unwrap_err());
        let element = result.unwrap();
        assert!(matches!(element, CsprojElement::Other), "{:#?}", element);
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
        assert!(result.is_ok(), "{}", result.unwrap_err());
        let depfile = result.unwrap();
        assert_eq!(depfile.elements.len(), 1);
        let group = match &depfile.elements[0] {
            CsprojElement::ItemGroup(item_group) => item_group,
            CsprojElement::Other => panic!("Expected ItemGroup"),
        };
        assert_eq!(
            group.package_references[0].include,
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
        assert!(result.is_ok(), "{}", result.unwrap_err());
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
        assert!(result.is_ok(), "{}", result.unwrap_err());
        let depfile = result.unwrap();
        assert_eq!(depfile.elements.len(), 2);

        let group_0 = match &depfile.elements[0] {
            CsprojElement::ItemGroup(item_group) => item_group,
            CsprojElement::Other => panic!("Expected ItemGroup"),
        };

        let group_1 = match &depfile.elements[1] {
            CsprojElement::ItemGroup(item_group) => item_group,
            CsprojElement::Other => panic!("Expected ItemGroup"),
        };
        assert_eq!(
            group_0.package_references[0].include,
            "Microsoft.SourceLink.GitHub"
        );
        assert_eq!(
            group_1.package_references[0].include,
            "System.Xml.XPath.XmlDocument"
        );
        assert_eq!(depfile.first_level_deps().len(), 2);
    }

    #[test]
    /// This input was not properly parsed by the original implementation.
    fn csproj_with_separated_item_groups_can_be_parsed() {
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
        assert!(result.is_ok(), "{}", result.unwrap_err());
        let depfile = result.unwrap();
        assert_eq!(depfile.elements.len(), 3);
        assert_eq!(depfile.first_level_deps().len(), 4, "{:#?}", depfile);
    }
}
