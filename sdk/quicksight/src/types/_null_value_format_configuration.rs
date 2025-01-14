// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The options that determine the null value format configuration.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct NullValueFormatConfiguration {
    /// <p>Determines the null string of null values.</p>
    pub null_string: ::std::option::Option<::std::string::String>,
}
impl NullValueFormatConfiguration {
    /// <p>Determines the null string of null values.</p>
    pub fn null_string(&self) -> ::std::option::Option<&str> {
        self.null_string.as_deref()
    }
}
impl ::std::fmt::Debug for NullValueFormatConfiguration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("NullValueFormatConfiguration");
        formatter.field("null_string", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl NullValueFormatConfiguration {
    /// Creates a new builder-style object to manufacture [`NullValueFormatConfiguration`](crate::types::NullValueFormatConfiguration).
    pub fn builder() -> crate::types::builders::NullValueFormatConfigurationBuilder {
        crate::types::builders::NullValueFormatConfigurationBuilder::default()
    }
}

/// A builder for [`NullValueFormatConfiguration`](crate::types::NullValueFormatConfiguration).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct NullValueFormatConfigurationBuilder {
    pub(crate) null_string: ::std::option::Option<::std::string::String>,
}
impl NullValueFormatConfigurationBuilder {
    /// <p>Determines the null string of null values.</p>
    pub fn null_string(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.null_string = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Determines the null string of null values.</p>
    pub fn set_null_string(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.null_string = input;
        self
    }
    /// <p>Determines the null string of null values.</p>
    pub fn get_null_string(&self) -> &::std::option::Option<::std::string::String> {
        &self.null_string
    }
    /// Consumes the builder and constructs a [`NullValueFormatConfiguration`](crate::types::NullValueFormatConfiguration).
    pub fn build(self) -> crate::types::NullValueFormatConfiguration {
        crate::types::NullValueFormatConfiguration {
            null_string: self.null_string,
        }
    }
}
impl ::std::fmt::Debug for NullValueFormatConfigurationBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("NullValueFormatConfigurationBuilder");
        formatter.field("null_string", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
