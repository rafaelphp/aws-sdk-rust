// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::enable_client_authentication::_enable_client_authentication_output::EnableClientAuthenticationOutputBuilder;

pub use crate::operation::enable_client_authentication::_enable_client_authentication_input::EnableClientAuthenticationInputBuilder;

impl EnableClientAuthenticationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::enable_client_authentication::EnableClientAuthenticationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::enable_client_authentication::EnableClientAuthenticationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.enable_client_authentication();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `EnableClientAuthentication`.
///
/// <p>Enables alternative client authentication methods for the specified directory.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct EnableClientAuthenticationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::enable_client_authentication::builders::EnableClientAuthenticationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl EnableClientAuthenticationFluentBuilder {
    /// Creates a new `EnableClientAuthentication`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the EnableClientAuthentication as a reference.
    pub fn as_input(&self) -> &crate::operation::enable_client_authentication::builders::EnableClientAuthenticationInputBuilder {
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
        crate::operation::enable_client_authentication::EnableClientAuthenticationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::enable_client_authentication::EnableClientAuthenticationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::enable_client_authentication::EnableClientAuthentication::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::enable_client_authentication::EnableClientAuthentication::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::enable_client_authentication::EnableClientAuthenticationOutput,
            crate::operation::enable_client_authentication::EnableClientAuthenticationError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::enable_client_authentication::EnableClientAuthenticationError>,
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
    /// <p>The identifier of the specified directory. </p>
    pub fn directory_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.directory_id(input.into());
        self
    }
    /// <p>The identifier of the specified directory. </p>
    pub fn set_directory_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_directory_id(input);
        self
    }
    /// <p>The identifier of the specified directory. </p>
    pub fn get_directory_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_directory_id()
    }
    /// <p>The type of client authentication to enable. Currently only the value <code>SmartCard</code> is supported. Smart card authentication in AD Connector requires that you enable Kerberos Constrained Delegation for the Service User to the LDAP service in your self-managed AD. </p>
    pub fn r#type(mut self, input: crate::types::ClientAuthenticationType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>The type of client authentication to enable. Currently only the value <code>SmartCard</code> is supported. Smart card authentication in AD Connector requires that you enable Kerberos Constrained Delegation for the Service User to the LDAP service in your self-managed AD. </p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::ClientAuthenticationType>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The type of client authentication to enable. Currently only the value <code>SmartCard</code> is supported. Smart card authentication in AD Connector requires that you enable Kerberos Constrained Delegation for the Service User to the LDAP service in your self-managed AD. </p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::ClientAuthenticationType> {
        self.inner.get_type()
    }
}
