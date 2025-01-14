// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_vpc_endpoint::_modify_vpc_endpoint_output::ModifyVpcEndpointOutputBuilder;

pub use crate::operation::modify_vpc_endpoint::_modify_vpc_endpoint_input::ModifyVpcEndpointInputBuilder;

impl ModifyVpcEndpointInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_vpc_endpoint::ModifyVpcEndpointOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::modify_vpc_endpoint::ModifyVpcEndpointError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_vpc_endpoint();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyVpcEndpoint`.
///
/// <p>Modifies attributes of a specified VPC endpoint. The attributes that you can modify depend on the type of VPC endpoint (interface, gateway, or Gateway Load Balancer). For more information, see the <a href="https://docs.aws.amazon.com/vpc/latest/privatelink/">Amazon Web Services PrivateLink Guide</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyVpcEndpointFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_vpc_endpoint::builders::ModifyVpcEndpointInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl ModifyVpcEndpointFluentBuilder {
    /// Creates a new `ModifyVpcEndpoint`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyVpcEndpoint as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_vpc_endpoint::builders::ModifyVpcEndpointInputBuilder {
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
        crate::operation::modify_vpc_endpoint::ModifyVpcEndpointOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::modify_vpc_endpoint::ModifyVpcEndpointError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_vpc_endpoint::ModifyVpcEndpoint::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_vpc_endpoint::ModifyVpcEndpoint::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::modify_vpc_endpoint::ModifyVpcEndpointOutput,
            crate::operation::modify_vpc_endpoint::ModifyVpcEndpointError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::modify_vpc_endpoint::ModifyVpcEndpointError>,
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
    /// <p>The ID of the endpoint.</p>
    pub fn vpc_endpoint_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vpc_endpoint_id(input.into());
        self
    }
    /// <p>The ID of the endpoint.</p>
    pub fn set_vpc_endpoint_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vpc_endpoint_id(input);
        self
    }
    /// <p>The ID of the endpoint.</p>
    pub fn get_vpc_endpoint_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_vpc_endpoint_id()
    }
    /// <p>(Gateway endpoint) Specify <code>true</code> to reset the policy document to the default policy. The default policy allows full access to the service.</p>
    pub fn reset_policy(mut self, input: bool) -> Self {
        self.inner = self.inner.reset_policy(input);
        self
    }
    /// <p>(Gateway endpoint) Specify <code>true</code> to reset the policy document to the default policy. The default policy allows full access to the service.</p>
    pub fn set_reset_policy(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_reset_policy(input);
        self
    }
    /// <p>(Gateway endpoint) Specify <code>true</code> to reset the policy document to the default policy. The default policy allows full access to the service.</p>
    pub fn get_reset_policy(&self) -> &::std::option::Option<bool> {
        self.inner.get_reset_policy()
    }
    /// <p>(Interface and gateway endpoints) A policy to attach to the endpoint that controls access to the service. The policy must be in valid JSON format.</p>
    pub fn policy_document(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.policy_document(input.into());
        self
    }
    /// <p>(Interface and gateway endpoints) A policy to attach to the endpoint that controls access to the service. The policy must be in valid JSON format.</p>
    pub fn set_policy_document(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_policy_document(input);
        self
    }
    /// <p>(Interface and gateway endpoints) A policy to attach to the endpoint that controls access to the service. The policy must be in valid JSON format.</p>
    pub fn get_policy_document(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_policy_document()
    }
    /// Appends an item to `AddRouteTableIds`.
    ///
    /// To override the contents of this collection use [`set_add_route_table_ids`](Self::set_add_route_table_ids).
    ///
    /// <p>(Gateway endpoint) The IDs of the route tables to associate with the endpoint.</p>
    pub fn add_route_table_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.add_route_table_ids(input.into());
        self
    }
    /// <p>(Gateway endpoint) The IDs of the route tables to associate with the endpoint.</p>
    pub fn set_add_route_table_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_add_route_table_ids(input);
        self
    }
    /// <p>(Gateway endpoint) The IDs of the route tables to associate with the endpoint.</p>
    pub fn get_add_route_table_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_add_route_table_ids()
    }
    /// Appends an item to `RemoveRouteTableIds`.
    ///
    /// To override the contents of this collection use [`set_remove_route_table_ids`](Self::set_remove_route_table_ids).
    ///
    /// <p>(Gateway endpoint) The IDs of the route tables to disassociate from the endpoint.</p>
    pub fn remove_route_table_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.remove_route_table_ids(input.into());
        self
    }
    /// <p>(Gateway endpoint) The IDs of the route tables to disassociate from the endpoint.</p>
    pub fn set_remove_route_table_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_remove_route_table_ids(input);
        self
    }
    /// <p>(Gateway endpoint) The IDs of the route tables to disassociate from the endpoint.</p>
    pub fn get_remove_route_table_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_remove_route_table_ids()
    }
    /// Appends an item to `AddSubnetIds`.
    ///
    /// To override the contents of this collection use [`set_add_subnet_ids`](Self::set_add_subnet_ids).
    ///
    /// <p>(Interface and Gateway Load Balancer endpoints) The IDs of the subnets in which to serve the endpoint. For a Gateway Load Balancer endpoint, you can specify only one subnet.</p>
    pub fn add_subnet_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.add_subnet_ids(input.into());
        self
    }
    /// <p>(Interface and Gateway Load Balancer endpoints) The IDs of the subnets in which to serve the endpoint. For a Gateway Load Balancer endpoint, you can specify only one subnet.</p>
    pub fn set_add_subnet_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_add_subnet_ids(input);
        self
    }
    /// <p>(Interface and Gateway Load Balancer endpoints) The IDs of the subnets in which to serve the endpoint. For a Gateway Load Balancer endpoint, you can specify only one subnet.</p>
    pub fn get_add_subnet_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_add_subnet_ids()
    }
    /// Appends an item to `RemoveSubnetIds`.
    ///
    /// To override the contents of this collection use [`set_remove_subnet_ids`](Self::set_remove_subnet_ids).
    ///
    /// <p>(Interface endpoint) The IDs of the subnets from which to remove the endpoint.</p>
    pub fn remove_subnet_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.remove_subnet_ids(input.into());
        self
    }
    /// <p>(Interface endpoint) The IDs of the subnets from which to remove the endpoint.</p>
    pub fn set_remove_subnet_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_remove_subnet_ids(input);
        self
    }
    /// <p>(Interface endpoint) The IDs of the subnets from which to remove the endpoint.</p>
    pub fn get_remove_subnet_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_remove_subnet_ids()
    }
    /// Appends an item to `AddSecurityGroupIds`.
    ///
    /// To override the contents of this collection use [`set_add_security_group_ids`](Self::set_add_security_group_ids).
    ///
    /// <p>(Interface endpoint) The IDs of the security groups to associate with the network interface.</p>
    pub fn add_security_group_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.add_security_group_ids(input.into());
        self
    }
    /// <p>(Interface endpoint) The IDs of the security groups to associate with the network interface.</p>
    pub fn set_add_security_group_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_add_security_group_ids(input);
        self
    }
    /// <p>(Interface endpoint) The IDs of the security groups to associate with the network interface.</p>
    pub fn get_add_security_group_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_add_security_group_ids()
    }
    /// Appends an item to `RemoveSecurityGroupIds`.
    ///
    /// To override the contents of this collection use [`set_remove_security_group_ids`](Self::set_remove_security_group_ids).
    ///
    /// <p>(Interface endpoint) The IDs of the security groups to disassociate from the network interface.</p>
    pub fn remove_security_group_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.remove_security_group_ids(input.into());
        self
    }
    /// <p>(Interface endpoint) The IDs of the security groups to disassociate from the network interface.</p>
    pub fn set_remove_security_group_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_remove_security_group_ids(input);
        self
    }
    /// <p>(Interface endpoint) The IDs of the security groups to disassociate from the network interface.</p>
    pub fn get_remove_security_group_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_remove_security_group_ids()
    }
    /// <p>The IP address type for the endpoint.</p>
    pub fn ip_address_type(mut self, input: crate::types::IpAddressType) -> Self {
        self.inner = self.inner.ip_address_type(input);
        self
    }
    /// <p>The IP address type for the endpoint.</p>
    pub fn set_ip_address_type(mut self, input: ::std::option::Option<crate::types::IpAddressType>) -> Self {
        self.inner = self.inner.set_ip_address_type(input);
        self
    }
    /// <p>The IP address type for the endpoint.</p>
    pub fn get_ip_address_type(&self) -> &::std::option::Option<crate::types::IpAddressType> {
        self.inner.get_ip_address_type()
    }
    /// <p>The DNS options for the endpoint.</p>
    pub fn dns_options(mut self, input: crate::types::DnsOptionsSpecification) -> Self {
        self.inner = self.inner.dns_options(input);
        self
    }
    /// <p>The DNS options for the endpoint.</p>
    pub fn set_dns_options(mut self, input: ::std::option::Option<crate::types::DnsOptionsSpecification>) -> Self {
        self.inner = self.inner.set_dns_options(input);
        self
    }
    /// <p>The DNS options for the endpoint.</p>
    pub fn get_dns_options(&self) -> &::std::option::Option<crate::types::DnsOptionsSpecification> {
        self.inner.get_dns_options()
    }
    /// <p>(Interface endpoint) Indicates whether a private hosted zone is associated with the VPC.</p>
    pub fn private_dns_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.private_dns_enabled(input);
        self
    }
    /// <p>(Interface endpoint) Indicates whether a private hosted zone is associated with the VPC.</p>
    pub fn set_private_dns_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_private_dns_enabled(input);
        self
    }
    /// <p>(Interface endpoint) Indicates whether a private hosted zone is associated with the VPC.</p>
    pub fn get_private_dns_enabled(&self) -> &::std::option::Option<bool> {
        self.inner.get_private_dns_enabled()
    }
}
