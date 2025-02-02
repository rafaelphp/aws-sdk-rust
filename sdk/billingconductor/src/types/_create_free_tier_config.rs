// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> The possible Amazon Web Services Free Tier configurations. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateFreeTierConfig {
    /// <p> Activate or deactivate Amazon Web Services Free Tier. </p>
    pub activated: ::std::option::Option<bool>,
}
impl CreateFreeTierConfig {
    /// <p> Activate or deactivate Amazon Web Services Free Tier. </p>
    pub fn activated(&self) -> ::std::option::Option<bool> {
        self.activated
    }
}
impl CreateFreeTierConfig {
    /// Creates a new builder-style object to manufacture [`CreateFreeTierConfig`](crate::types::CreateFreeTierConfig).
    pub fn builder() -> crate::types::builders::CreateFreeTierConfigBuilder {
        crate::types::builders::CreateFreeTierConfigBuilder::default()
    }
}

/// A builder for [`CreateFreeTierConfig`](crate::types::CreateFreeTierConfig).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateFreeTierConfigBuilder {
    pub(crate) activated: ::std::option::Option<bool>,
}
impl CreateFreeTierConfigBuilder {
    /// <p> Activate or deactivate Amazon Web Services Free Tier. </p>
    pub fn activated(mut self, input: bool) -> Self {
        self.activated = ::std::option::Option::Some(input);
        self
    }
    /// <p> Activate or deactivate Amazon Web Services Free Tier. </p>
    pub fn set_activated(mut self, input: ::std::option::Option<bool>) -> Self {
        self.activated = input;
        self
    }
    /// <p> Activate or deactivate Amazon Web Services Free Tier. </p>
    pub fn get_activated(&self) -> &::std::option::Option<bool> {
        &self.activated
    }
    /// Consumes the builder and constructs a [`CreateFreeTierConfig`](crate::types::CreateFreeTierConfig).
    pub fn build(self) -> crate::types::CreateFreeTierConfig {
        crate::types::CreateFreeTierConfig { activated: self.activated }
    }
}
