// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_private_dns_namespace::_create_private_dns_namespace_output::CreatePrivateDnsNamespaceOutputBuilder;

pub use crate::operation::create_private_dns_namespace::_create_private_dns_namespace_input::CreatePrivateDnsNamespaceInputBuilder;

impl CreatePrivateDnsNamespaceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_private_dns_namespace::CreatePrivateDnsNamespaceOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_private_dns_namespace::CreatePrivateDnsNamespaceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_private_dns_namespace();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreatePrivateDnsNamespace`.
///
/// <p>Creates a private namespace based on DNS, which is visible only inside a specified Amazon VPC. The namespace defines your service naming scheme. For example, if you name your namespace <code>example.com</code> and name your service <code>backend</code>, the resulting DNS name for the service is <code>backend.example.com</code>. Service instances that are registered using a private DNS namespace can be discovered using either a <code>DiscoverInstances</code> request or using DNS. For the current quota on the number of namespaces that you can create using the same Amazon Web Services account, see <a href="https://docs.aws.amazon.com/cloud-map/latest/dg/cloud-map-limits.html">Cloud Map quotas</a> in the <i>Cloud Map Developer Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreatePrivateDnsNamespaceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_private_dns_namespace::builders::CreatePrivateDnsNamespaceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl CreatePrivateDnsNamespaceFluentBuilder {
    /// Creates a new `CreatePrivateDnsNamespace`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreatePrivateDnsNamespace as a reference.
    pub fn as_input(&self) -> &crate::operation::create_private_dns_namespace::builders::CreatePrivateDnsNamespaceInputBuilder {
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
        crate::operation::create_private_dns_namespace::CreatePrivateDnsNamespaceOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_private_dns_namespace::CreatePrivateDnsNamespaceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_private_dns_namespace::CreatePrivateDnsNamespace::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_private_dns_namespace::CreatePrivateDnsNamespace::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::create_private_dns_namespace::CreatePrivateDnsNamespaceOutput,
            crate::operation::create_private_dns_namespace::CreatePrivateDnsNamespaceError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_private_dns_namespace::CreatePrivateDnsNamespaceError>,
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
    /// <p>The name that you want to assign to this namespace. When you create a private DNS namespace, Cloud Map automatically creates an Amazon Route&nbsp;53 private hosted zone that has the same name as the namespace.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name that you want to assign to this namespace. When you create a private DNS namespace, Cloud Map automatically creates an Amazon Route&nbsp;53 private hosted zone that has the same name as the namespace.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name that you want to assign to this namespace. When you create a private DNS namespace, Cloud Map automatically creates an Amazon Route&nbsp;53 private hosted zone that has the same name as the namespace.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>A unique string that identifies the request and that allows failed <code>CreatePrivateDnsNamespace</code> requests to be retried without the risk of running the operation twice. <code>CreatorRequestId</code> can be any unique string (for example, a date/timestamp).</p>
    pub fn creator_request_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.creator_request_id(input.into());
        self
    }
    /// <p>A unique string that identifies the request and that allows failed <code>CreatePrivateDnsNamespace</code> requests to be retried without the risk of running the operation twice. <code>CreatorRequestId</code> can be any unique string (for example, a date/timestamp).</p>
    pub fn set_creator_request_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_creator_request_id(input);
        self
    }
    /// <p>A unique string that identifies the request and that allows failed <code>CreatePrivateDnsNamespace</code> requests to be retried without the risk of running the operation twice. <code>CreatorRequestId</code> can be any unique string (for example, a date/timestamp).</p>
    pub fn get_creator_request_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_creator_request_id()
    }
    /// <p>A description for the namespace.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description for the namespace.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description for the namespace.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The ID of the Amazon VPC that you want to associate the namespace with.</p>
    pub fn vpc(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vpc(input.into());
        self
    }
    /// <p>The ID of the Amazon VPC that you want to associate the namespace with.</p>
    pub fn set_vpc(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vpc(input);
        self
    }
    /// <p>The ID of the Amazon VPC that you want to associate the namespace with.</p>
    pub fn get_vpc(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_vpc()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to add to the namespace. Each tag consists of a key and an optional value that you define. Tags keys can be up to 128 characters in length, and tag values can be up to 256 characters in length.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The tags to add to the namespace. Each tag consists of a key and an optional value that you define. Tags keys can be up to 128 characters in length, and tag values can be up to 256 characters in length.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tags to add to the namespace. Each tag consists of a key and an optional value that you define. Tags keys can be up to 128 characters in length, and tag values can be up to 256 characters in length.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
    /// <p>Properties for the private DNS namespace.</p>
    pub fn properties(mut self, input: crate::types::PrivateDnsNamespaceProperties) -> Self {
        self.inner = self.inner.properties(input);
        self
    }
    /// <p>Properties for the private DNS namespace.</p>
    pub fn set_properties(mut self, input: ::std::option::Option<crate::types::PrivateDnsNamespaceProperties>) -> Self {
        self.inner = self.inner.set_properties(input);
        self
    }
    /// <p>Properties for the private DNS namespace.</p>
    pub fn get_properties(&self) -> &::std::option::Option<crate::types::PrivateDnsNamespaceProperties> {
        self.inner.get_properties()
    }
}
