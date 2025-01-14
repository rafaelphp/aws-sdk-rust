// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_security_configs::_list_security_configs_output::ListSecurityConfigsOutputBuilder;

pub use crate::operation::list_security_configs::_list_security_configs_input::ListSecurityConfigsInputBuilder;

impl ListSecurityConfigsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_security_configs::ListSecurityConfigsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_security_configs::ListSecurityConfigsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_security_configs();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListSecurityConfigs`.
///
/// <p>Returns information about configured OpenSearch Serverless security configurations. For more information, see <a href="https://docs.aws.amazon.com/opensearch-service/latest/developerguide/serverless-saml.html">SAML authentication for Amazon OpenSearch Serverless</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListSecurityConfigsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_security_configs::builders::ListSecurityConfigsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl ListSecurityConfigsFluentBuilder {
    /// Creates a new `ListSecurityConfigs`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListSecurityConfigs as a reference.
    pub fn as_input(&self) -> &crate::operation::list_security_configs::builders::ListSecurityConfigsInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_security_configs::ListSecurityConfigsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_security_configs::ListSecurityConfigsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_security_configs::ListSecurityConfigs::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_security_configs::ListSecurityConfigs::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::list_security_configs::ListSecurityConfigsOutput,
            crate::operation::list_security_configs::ListSecurityConfigsError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_security_configs::ListSecurityConfigsError>,
    > {
        ::std::result::Result::Ok(crate::client::customize::orchestrator::CustomizableOperation {
            customizable_send: ::std::boxed::Box::new(move |config_override| {
                ::std::boxed::Box::pin(async { self.config_override(config_override).send().await })
            }),
            config_override: None,
            interceptors: vec![],
            runtime_plugins: vec![],
        })
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_security_configs::paginator::ListSecurityConfigsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_security_configs::paginator::ListSecurityConfigsPaginator {
        crate::operation::list_security_configs::paginator::ListSecurityConfigsPaginator::new(self.handle, self.inner)
    }
    /// <p>The type of security configuration.</p>
    pub fn r#type(mut self, input: crate::types::SecurityConfigType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>The type of security configuration.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::SecurityConfigType>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The type of security configuration.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::SecurityConfigType> {
        self.inner.get_type()
    }
    /// <p>If your initial <code>ListSecurityConfigs</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in subsequent <code>ListSecurityConfigs</code> operations, which returns results in the next page. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If your initial <code>ListSecurityConfigs</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in subsequent <code>ListSecurityConfigs</code> operations, which returns results in the next page. </p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>If your initial <code>ListSecurityConfigs</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in subsequent <code>ListSecurityConfigs</code> operations, which returns results in the next page. </p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>An optional parameter that specifies the maximum number of results to return. You can use <code>nextToken</code> to get the next page of results. The default is 20.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>An optional parameter that specifies the maximum number of results to return. You can use <code>nextToken</code> to get the next page of results. The default is 20.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>An optional parameter that specifies the maximum number of results to return. You can use <code>nextToken</code> to get the next page of results. The default is 20.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
