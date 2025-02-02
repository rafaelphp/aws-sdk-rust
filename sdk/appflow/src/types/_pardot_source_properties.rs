// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The properties that are applied when Salesforce Pardot is being used as a source.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PardotSourceProperties {
    /// <p>The object specified in the Salesforce Pardot flow source.</p>
    pub object: ::std::option::Option<::std::string::String>,
}
impl PardotSourceProperties {
    /// <p>The object specified in the Salesforce Pardot flow source.</p>
    pub fn object(&self) -> ::std::option::Option<&str> {
        self.object.as_deref()
    }
}
impl PardotSourceProperties {
    /// Creates a new builder-style object to manufacture [`PardotSourceProperties`](crate::types::PardotSourceProperties).
    pub fn builder() -> crate::types::builders::PardotSourcePropertiesBuilder {
        crate::types::builders::PardotSourcePropertiesBuilder::default()
    }
}

/// A builder for [`PardotSourceProperties`](crate::types::PardotSourceProperties).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PardotSourcePropertiesBuilder {
    pub(crate) object: ::std::option::Option<::std::string::String>,
}
impl PardotSourcePropertiesBuilder {
    /// <p>The object specified in the Salesforce Pardot flow source.</p>
    pub fn object(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.object = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The object specified in the Salesforce Pardot flow source.</p>
    pub fn set_object(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.object = input;
        self
    }
    /// <p>The object specified in the Salesforce Pardot flow source.</p>
    pub fn get_object(&self) -> &::std::option::Option<::std::string::String> {
        &self.object
    }
    /// Consumes the builder and constructs a [`PardotSourceProperties`](crate::types::PardotSourceProperties).
    pub fn build(self) -> crate::types::PardotSourceProperties {
        crate::types::PardotSourceProperties { object: self.object }
    }
}
