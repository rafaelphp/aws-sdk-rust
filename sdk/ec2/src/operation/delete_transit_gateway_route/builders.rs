// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_transit_gateway_route::_delete_transit_gateway_route_output::DeleteTransitGatewayRouteOutputBuilder;

pub use crate::operation::delete_transit_gateway_route::_delete_transit_gateway_route_input::DeleteTransitGatewayRouteInputBuilder;

impl DeleteTransitGatewayRouteInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_transit_gateway_route::DeleteTransitGatewayRouteOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_transit_gateway_route::DeleteTransitGatewayRouteError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_transit_gateway_route();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteTransitGatewayRoute`.
///
/// <p>Deletes the specified route from the specified transit gateway route table.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteTransitGatewayRouteFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_transit_gateway_route::builders::DeleteTransitGatewayRouteInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DeleteTransitGatewayRouteFluentBuilder {
    /// Creates a new `DeleteTransitGatewayRoute`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteTransitGatewayRoute as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_transit_gateway_route::builders::DeleteTransitGatewayRouteInputBuilder {
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
        crate::operation::delete_transit_gateway_route::DeleteTransitGatewayRouteOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_transit_gateway_route::DeleteTransitGatewayRouteError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_transit_gateway_route::DeleteTransitGatewayRoute::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_transit_gateway_route::DeleteTransitGatewayRoute::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::delete_transit_gateway_route::DeleteTransitGatewayRouteOutput,
            crate::operation::delete_transit_gateway_route::DeleteTransitGatewayRouteError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::delete_transit_gateway_route::DeleteTransitGatewayRouteError>,
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
    /// <p>The ID of the transit gateway route table.</p>
    pub fn transit_gateway_route_table_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.transit_gateway_route_table_id(input.into());
        self
    }
    /// <p>The ID of the transit gateway route table.</p>
    pub fn set_transit_gateway_route_table_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_transit_gateway_route_table_id(input);
        self
    }
    /// <p>The ID of the transit gateway route table.</p>
    pub fn get_transit_gateway_route_table_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_transit_gateway_route_table_id()
    }
    /// <p>The CIDR range for the route. This must match the CIDR for the route exactly.</p>
    pub fn destination_cidr_block(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.destination_cidr_block(input.into());
        self
    }
    /// <p>The CIDR range for the route. This must match the CIDR for the route exactly.</p>
    pub fn set_destination_cidr_block(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_destination_cidr_block(input);
        self
    }
    /// <p>The CIDR range for the route. This must match the CIDR for the route exactly.</p>
    pub fn get_destination_cidr_block(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_destination_cidr_block()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
}
