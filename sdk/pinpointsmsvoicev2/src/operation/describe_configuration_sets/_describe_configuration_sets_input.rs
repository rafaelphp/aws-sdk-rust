// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeConfigurationSetsInput {
    /// <p>An array of strings. Each element can be either a ConfigurationSetName or ConfigurationSetArn.</p>
    pub configuration_set_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>An array of filters to apply to the results that are returned.</p>
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::ConfigurationSetFilter>>,
    /// <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of results to return per each request.</p>
    pub max_results: ::std::option::Option<i32>,
}
impl DescribeConfigurationSetsInput {
    /// <p>An array of strings. Each element can be either a ConfigurationSetName or ConfigurationSetArn.</p>
    pub fn configuration_set_names(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.configuration_set_names.as_deref()
    }
    /// <p>An array of filters to apply to the results that are returned.</p>
    pub fn filters(&self) -> ::std::option::Option<&[crate::types::ConfigurationSetFilter]> {
        self.filters.as_deref()
    }
    /// <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of results to return per each request.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
}
impl DescribeConfigurationSetsInput {
    /// Creates a new builder-style object to manufacture [`DescribeConfigurationSetsInput`](crate::operation::describe_configuration_sets::DescribeConfigurationSetsInput).
    pub fn builder() -> crate::operation::describe_configuration_sets::builders::DescribeConfigurationSetsInputBuilder {
        crate::operation::describe_configuration_sets::builders::DescribeConfigurationSetsInputBuilder::default()
    }
}

/// A builder for [`DescribeConfigurationSetsInput`](crate::operation::describe_configuration_sets::DescribeConfigurationSetsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeConfigurationSetsInputBuilder {
    pub(crate) configuration_set_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::ConfigurationSetFilter>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
}
impl DescribeConfigurationSetsInputBuilder {
    /// Appends an item to `configuration_set_names`.
    ///
    /// To override the contents of this collection use [`set_configuration_set_names`](Self::set_configuration_set_names).
    ///
    /// <p>An array of strings. Each element can be either a ConfigurationSetName or ConfigurationSetArn.</p>
    pub fn configuration_set_names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.configuration_set_names.unwrap_or_default();
        v.push(input.into());
        self.configuration_set_names = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of strings. Each element can be either a ConfigurationSetName or ConfigurationSetArn.</p>
    pub fn set_configuration_set_names(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.configuration_set_names = input;
        self
    }
    /// <p>An array of strings. Each element can be either a ConfigurationSetName or ConfigurationSetArn.</p>
    pub fn get_configuration_set_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.configuration_set_names
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>An array of filters to apply to the results that are returned.</p>
    pub fn filters(mut self, input: crate::types::ConfigurationSetFilter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of filters to apply to the results that are returned.</p>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ConfigurationSetFilter>>) -> Self {
        self.filters = input;
        self
    }
    /// <p>An array of filters to apply to the results that are returned.</p>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ConfigurationSetFilter>> {
        &self.filters
    }
    /// <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// <p>The maximum number of results to return per each request.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to return per each request.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The maximum number of results to return per each request.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }
    /// Consumes the builder and constructs a [`DescribeConfigurationSetsInput`](crate::operation::describe_configuration_sets::DescribeConfigurationSetsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_configuration_sets::DescribeConfigurationSetsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_configuration_sets::DescribeConfigurationSetsInput {
            configuration_set_names: self.configuration_set_names,
            filters: self.filters,
            next_token: self.next_token,
            max_results: self.max_results,
        })
    }
}
