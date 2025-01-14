// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains current status information for the configuration.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ConfigurationStatus {
    /// <p>The current state of the configuration.</p>
    pub state: ::std::option::Option<crate::types::ConfigurationState>,
    /// <p>Contains associated error information, if any.</p>
    pub error: ::std::option::Option<crate::types::ConfigurationErrorDetails>,
}
impl ConfigurationStatus {
    /// <p>The current state of the configuration.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::ConfigurationState> {
        self.state.as_ref()
    }
    /// <p>Contains associated error information, if any.</p>
    pub fn error(&self) -> ::std::option::Option<&crate::types::ConfigurationErrorDetails> {
        self.error.as_ref()
    }
}
impl ConfigurationStatus {
    /// Creates a new builder-style object to manufacture [`ConfigurationStatus`](crate::types::ConfigurationStatus).
    pub fn builder() -> crate::types::builders::ConfigurationStatusBuilder {
        crate::types::builders::ConfigurationStatusBuilder::default()
    }
}

/// A builder for [`ConfigurationStatus`](crate::types::ConfigurationStatus).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ConfigurationStatusBuilder {
    pub(crate) state: ::std::option::Option<crate::types::ConfigurationState>,
    pub(crate) error: ::std::option::Option<crate::types::ConfigurationErrorDetails>,
}
impl ConfigurationStatusBuilder {
    /// <p>The current state of the configuration.</p>
    pub fn state(mut self, input: crate::types::ConfigurationState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current state of the configuration.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::ConfigurationState>) -> Self {
        self.state = input;
        self
    }
    /// <p>The current state of the configuration.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::ConfigurationState> {
        &self.state
    }
    /// <p>Contains associated error information, if any.</p>
    pub fn error(mut self, input: crate::types::ConfigurationErrorDetails) -> Self {
        self.error = ::std::option::Option::Some(input);
        self
    }
    /// <p>Contains associated error information, if any.</p>
    pub fn set_error(mut self, input: ::std::option::Option<crate::types::ConfigurationErrorDetails>) -> Self {
        self.error = input;
        self
    }
    /// <p>Contains associated error information, if any.</p>
    pub fn get_error(&self) -> &::std::option::Option<crate::types::ConfigurationErrorDetails> {
        &self.error
    }
    /// Consumes the builder and constructs a [`ConfigurationStatus`](crate::types::ConfigurationStatus).
    pub fn build(self) -> crate::types::ConfigurationStatus {
        crate::types::ConfigurationStatus {
            state: self.state,
            error: self.error,
        }
    }
}
