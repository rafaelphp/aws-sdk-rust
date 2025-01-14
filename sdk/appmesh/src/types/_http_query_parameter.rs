// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that represents the query parameter in the request.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct HttpQueryParameter {
    /// <p>A name for the query parameter that will be matched on.</p>
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The query parameter to match on.</p>
    pub r#match: ::std::option::Option<crate::types::QueryParameterMatch>,
}
impl HttpQueryParameter {
    /// <p>A name for the query parameter that will be matched on.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The query parameter to match on.</p>
    pub fn r#match(&self) -> ::std::option::Option<&crate::types::QueryParameterMatch> {
        self.r#match.as_ref()
    }
}
impl HttpQueryParameter {
    /// Creates a new builder-style object to manufacture [`HttpQueryParameter`](crate::types::HttpQueryParameter).
    pub fn builder() -> crate::types::builders::HttpQueryParameterBuilder {
        crate::types::builders::HttpQueryParameterBuilder::default()
    }
}

/// A builder for [`HttpQueryParameter`](crate::types::HttpQueryParameter).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct HttpQueryParameterBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) r#match: ::std::option::Option<crate::types::QueryParameterMatch>,
}
impl HttpQueryParameterBuilder {
    /// <p>A name for the query parameter that will be matched on.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A name for the query parameter that will be matched on.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>A name for the query parameter that will be matched on.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>The query parameter to match on.</p>
    pub fn r#match(mut self, input: crate::types::QueryParameterMatch) -> Self {
        self.r#match = ::std::option::Option::Some(input);
        self
    }
    /// <p>The query parameter to match on.</p>
    pub fn set_match(mut self, input: ::std::option::Option<crate::types::QueryParameterMatch>) -> Self {
        self.r#match = input;
        self
    }
    /// <p>The query parameter to match on.</p>
    pub fn get_match(&self) -> &::std::option::Option<crate::types::QueryParameterMatch> {
        &self.r#match
    }
    /// Consumes the builder and constructs a [`HttpQueryParameter`](crate::types::HttpQueryParameter).
    pub fn build(self) -> crate::types::HttpQueryParameter {
        crate::types::HttpQueryParameter {
            name: self.name,
            r#match: self.r#match,
        }
    }
}
