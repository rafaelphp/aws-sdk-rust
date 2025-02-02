// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_tunnels::_list_tunnels_output::ListTunnelsOutputBuilder;

pub use crate::operation::list_tunnels::_list_tunnels_input::ListTunnelsInputBuilder;

impl ListTunnelsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_tunnels::ListTunnelsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_tunnels::ListTunnelsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_tunnels();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListTunnels`.
///
/// <p>List all tunnels for an Amazon Web Services account. Tunnels are listed by creation time in descending order, newer tunnels will be listed before older tunnels.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">ListTunnels</a> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListTunnelsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_tunnels::builders::ListTunnelsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl ListTunnelsFluentBuilder {
    /// Creates a new `ListTunnels`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListTunnels as a reference.
    pub fn as_input(&self) -> &crate::operation::list_tunnels::builders::ListTunnelsInputBuilder {
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
        crate::operation::list_tunnels::ListTunnelsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_tunnels::ListTunnelsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_tunnels::ListTunnels::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_tunnels::ListTunnels::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::list_tunnels::ListTunnelsOutput,
            crate::operation::list_tunnels::ListTunnelsError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_tunnels::ListTunnelsError>,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_tunnels::paginator::ListTunnelsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_tunnels::paginator::ListTunnelsPaginator {
        crate::operation::list_tunnels::paginator::ListTunnelsPaginator::new(self.handle, self.inner)
    }
    /// <p>The name of the IoT thing associated with the destination device.</p>
    pub fn thing_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.thing_name(input.into());
        self
    }
    /// <p>The name of the IoT thing associated with the destination device.</p>
    pub fn set_thing_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_thing_name(input);
        self
    }
    /// <p>The name of the IoT thing associated with the destination device.</p>
    pub fn get_thing_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_thing_name()
    }
    /// <p>The maximum number of results to return at once.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return at once.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return at once.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>To retrieve the next set of results, the nextToken value from a previous response; otherwise null to receive the first set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>To retrieve the next set of results, the nextToken value from a previous response; otherwise null to receive the first set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>To retrieve the next set of results, the nextToken value from a previous response; otherwise null to receive the first set of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
