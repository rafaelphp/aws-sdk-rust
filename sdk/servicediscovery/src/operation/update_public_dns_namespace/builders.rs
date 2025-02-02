// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_public_dns_namespace::_update_public_dns_namespace_output::UpdatePublicDnsNamespaceOutputBuilder;

pub use crate::operation::update_public_dns_namespace::_update_public_dns_namespace_input::UpdatePublicDnsNamespaceInputBuilder;

impl UpdatePublicDnsNamespaceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_public_dns_namespace::UpdatePublicDnsNamespaceOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_public_dns_namespace::UpdatePublicDnsNamespaceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_public_dns_namespace();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdatePublicDnsNamespace`.
///
/// <p>Updates a public DNS namespace.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdatePublicDnsNamespaceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_public_dns_namespace::builders::UpdatePublicDnsNamespaceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl UpdatePublicDnsNamespaceFluentBuilder {
    /// Creates a new `UpdatePublicDnsNamespace`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdatePublicDnsNamespace as a reference.
    pub fn as_input(&self) -> &crate::operation::update_public_dns_namespace::builders::UpdatePublicDnsNamespaceInputBuilder {
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
        crate::operation::update_public_dns_namespace::UpdatePublicDnsNamespaceOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_public_dns_namespace::UpdatePublicDnsNamespaceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_public_dns_namespace::UpdatePublicDnsNamespace::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_public_dns_namespace::UpdatePublicDnsNamespace::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_public_dns_namespace::UpdatePublicDnsNamespaceOutput,
            crate::operation::update_public_dns_namespace::UpdatePublicDnsNamespaceError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_public_dns_namespace::UpdatePublicDnsNamespaceError>,
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
    /// <p>The ID of the namespace being updated.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The ID of the namespace being updated.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The ID of the namespace being updated.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
    /// <p>A unique string that identifies the request and that allows failed <code>UpdatePublicDnsNamespace</code> requests to be retried without the risk of running the operation twice. <code>UpdaterRequestId</code> can be any unique string (for example, a date/timestamp).</p>
    pub fn updater_request_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.updater_request_id(input.into());
        self
    }
    /// <p>A unique string that identifies the request and that allows failed <code>UpdatePublicDnsNamespace</code> requests to be retried without the risk of running the operation twice. <code>UpdaterRequestId</code> can be any unique string (for example, a date/timestamp).</p>
    pub fn set_updater_request_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_updater_request_id(input);
        self
    }
    /// <p>A unique string that identifies the request and that allows failed <code>UpdatePublicDnsNamespace</code> requests to be retried without the risk of running the operation twice. <code>UpdaterRequestId</code> can be any unique string (for example, a date/timestamp).</p>
    pub fn get_updater_request_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_updater_request_id()
    }
    /// <p>Updated properties for the public DNS namespace.</p>
    pub fn namespace(mut self, input: crate::types::PublicDnsNamespaceChange) -> Self {
        self.inner = self.inner.namespace(input);
        self
    }
    /// <p>Updated properties for the public DNS namespace.</p>
    pub fn set_namespace(mut self, input: ::std::option::Option<crate::types::PublicDnsNamespaceChange>) -> Self {
        self.inner = self.inner.set_namespace(input);
        self
    }
    /// <p>Updated properties for the public DNS namespace.</p>
    pub fn get_namespace(&self) -> &::std::option::Option<crate::types::PublicDnsNamespaceChange> {
        self.inner.get_namespace()
    }
}
