// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::add_endpoints::_add_endpoints_output::AddEndpointsOutputBuilder;

pub use crate::operation::add_endpoints::_add_endpoints_input::AddEndpointsInputBuilder;

impl AddEndpointsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::add_endpoints::AddEndpointsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::add_endpoints::AddEndpointsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.add_endpoints();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AddEndpoints`.
///
/// <p>Add endpoints to an endpoint group. The <code>AddEndpoints</code> API operation is the recommended option for adding endpoints. The alternative options are to add endpoints when you create an endpoint group (with the <a href="https://docs.aws.amazon.com/global-accelerator/latest/api/API_CreateEndpointGroup.html">CreateEndpointGroup</a> API) or when you update an endpoint group (with the <a href="https://docs.aws.amazon.com/global-accelerator/latest/api/API_UpdateEndpointGroup.html">UpdateEndpointGroup</a> API). </p>
/// <p>There are two advantages to using <code>AddEndpoints</code> to add endpoints:</p>
/// <ul>
/// <li> <p>It's faster, because Global Accelerator only has to resolve the new endpoints that you're adding.</p> </li>
/// <li> <p>It's more convenient, because you don't need to specify all of the current endpoints that are already in the endpoint group in addition to the new endpoints that you want to add.</p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AddEndpointsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::add_endpoints::builders::AddEndpointsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl AddEndpointsFluentBuilder {
    /// Creates a new `AddEndpoints`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AddEndpoints as a reference.
    pub fn as_input(&self) -> &crate::operation::add_endpoints::builders::AddEndpointsInputBuilder {
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
        crate::operation::add_endpoints::AddEndpointsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::add_endpoints::AddEndpointsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::add_endpoints::AddEndpoints::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::add_endpoints::AddEndpoints::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::add_endpoints::AddEndpointsOutput,
            crate::operation::add_endpoints::AddEndpointsError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::add_endpoints::AddEndpointsError>,
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
    /// Appends an item to `EndpointConfigurations`.
    ///
    /// To override the contents of this collection use [`set_endpoint_configurations`](Self::set_endpoint_configurations).
    ///
    /// <p>The list of endpoint objects.</p>
    pub fn endpoint_configurations(mut self, input: crate::types::EndpointConfiguration) -> Self {
        self.inner = self.inner.endpoint_configurations(input);
        self
    }
    /// <p>The list of endpoint objects.</p>
    pub fn set_endpoint_configurations(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::EndpointConfiguration>>) -> Self {
        self.inner = self.inner.set_endpoint_configurations(input);
        self
    }
    /// <p>The list of endpoint objects.</p>
    pub fn get_endpoint_configurations(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::EndpointConfiguration>> {
        self.inner.get_endpoint_configurations()
    }
    /// <p>The Amazon Resource Name (ARN) of the endpoint group.</p>
    pub fn endpoint_group_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.endpoint_group_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the endpoint group.</p>
    pub fn set_endpoint_group_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_endpoint_group_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the endpoint group.</p>
    pub fn get_endpoint_group_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_endpoint_group_arn()
    }
}
