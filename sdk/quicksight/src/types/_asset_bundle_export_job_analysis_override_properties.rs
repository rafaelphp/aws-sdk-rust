// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Controls how a specific <code>Analysis</code> resource is parameterized in the returned CloudFormation template.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AssetBundleExportJobAnalysisOverrideProperties {
    /// <p>The ARN of the specific <code>Analysis</code> resource whose override properties are configured in this structure.</p>
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>A list of <code>Analysis</code> resource properties to generate variables for in the returned CloudFormation template.</p>
    pub properties: ::std::option::Option<::std::vec::Vec<crate::types::AssetBundleExportJobAnalysisPropertyToOverride>>,
}
impl AssetBundleExportJobAnalysisOverrideProperties {
    /// <p>The ARN of the specific <code>Analysis</code> resource whose override properties are configured in this structure.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>A list of <code>Analysis</code> resource properties to generate variables for in the returned CloudFormation template.</p>
    pub fn properties(&self) -> ::std::option::Option<&[crate::types::AssetBundleExportJobAnalysisPropertyToOverride]> {
        self.properties.as_deref()
    }
}
impl AssetBundleExportJobAnalysisOverrideProperties {
    /// Creates a new builder-style object to manufacture [`AssetBundleExportJobAnalysisOverrideProperties`](crate::types::AssetBundleExportJobAnalysisOverrideProperties).
    pub fn builder() -> crate::types::builders::AssetBundleExportJobAnalysisOverridePropertiesBuilder {
        crate::types::builders::AssetBundleExportJobAnalysisOverridePropertiesBuilder::default()
    }
}

/// A builder for [`AssetBundleExportJobAnalysisOverrideProperties`](crate::types::AssetBundleExportJobAnalysisOverrideProperties).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AssetBundleExportJobAnalysisOverridePropertiesBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) properties: ::std::option::Option<::std::vec::Vec<crate::types::AssetBundleExportJobAnalysisPropertyToOverride>>,
}
impl AssetBundleExportJobAnalysisOverridePropertiesBuilder {
    /// <p>The ARN of the specific <code>Analysis</code> resource whose override properties are configured in this structure.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the specific <code>Analysis</code> resource whose override properties are configured in this structure.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The ARN of the specific <code>Analysis</code> resource whose override properties are configured in this structure.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
    }
    /// Appends an item to `properties`.
    ///
    /// To override the contents of this collection use [`set_properties`](Self::set_properties).
    ///
    /// <p>A list of <code>Analysis</code> resource properties to generate variables for in the returned CloudFormation template.</p>
    pub fn properties(mut self, input: crate::types::AssetBundleExportJobAnalysisPropertyToOverride) -> Self {
        let mut v = self.properties.unwrap_or_default();
        v.push(input);
        self.properties = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of <code>Analysis</code> resource properties to generate variables for in the returned CloudFormation template.</p>
    pub fn set_properties(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AssetBundleExportJobAnalysisPropertyToOverride>>,
    ) -> Self {
        self.properties = input;
        self
    }
    /// <p>A list of <code>Analysis</code> resource properties to generate variables for in the returned CloudFormation template.</p>
    pub fn get_properties(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AssetBundleExportJobAnalysisPropertyToOverride>> {
        &self.properties
    }
    /// Consumes the builder and constructs a [`AssetBundleExportJobAnalysisOverrideProperties`](crate::types::AssetBundleExportJobAnalysisOverrideProperties).
    pub fn build(self) -> crate::types::AssetBundleExportJobAnalysisOverrideProperties {
        crate::types::AssetBundleExportJobAnalysisOverrideProperties {
            arn: self.arn,
            properties: self.properties,
        }
    }
}
