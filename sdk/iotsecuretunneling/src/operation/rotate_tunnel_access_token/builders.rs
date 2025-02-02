// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::rotate_tunnel_access_token::_rotate_tunnel_access_token_output::RotateTunnelAccessTokenOutputBuilder;

pub use crate::operation::rotate_tunnel_access_token::_rotate_tunnel_access_token_input::RotateTunnelAccessTokenInputBuilder;

impl RotateTunnelAccessTokenInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::rotate_tunnel_access_token::RotateTunnelAccessTokenOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::rotate_tunnel_access_token::RotateTunnelAccessTokenError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.rotate_tunnel_access_token();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RotateTunnelAccessToken`.
///
/// <p>Revokes the current client access token (CAT) and returns new CAT for clients to use when reconnecting to secure tunneling to access the same tunnel.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">RotateTunnelAccessToken</a> action.</p> <note>
/// <p>Rotating the CAT doesn't extend the tunnel duration. For example, say the tunnel duration is 12 hours and the tunnel has already been open for 4 hours. When you rotate the access tokens, the new tokens that are generated can only be used for the remaining 8 hours.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RotateTunnelAccessTokenFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::rotate_tunnel_access_token::builders::RotateTunnelAccessTokenInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl RotateTunnelAccessTokenFluentBuilder {
    /// Creates a new `RotateTunnelAccessToken`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RotateTunnelAccessToken as a reference.
    pub fn as_input(&self) -> &crate::operation::rotate_tunnel_access_token::builders::RotateTunnelAccessTokenInputBuilder {
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
        crate::operation::rotate_tunnel_access_token::RotateTunnelAccessTokenOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::rotate_tunnel_access_token::RotateTunnelAccessTokenError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::rotate_tunnel_access_token::RotateTunnelAccessToken::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::rotate_tunnel_access_token::RotateTunnelAccessToken::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::rotate_tunnel_access_token::RotateTunnelAccessTokenOutput,
            crate::operation::rotate_tunnel_access_token::RotateTunnelAccessTokenError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::rotate_tunnel_access_token::RotateTunnelAccessTokenError>,
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
    /// <p>The tunnel for which you want to rotate the access tokens.</p>
    pub fn tunnel_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tunnel_id(input.into());
        self
    }
    /// <p>The tunnel for which you want to rotate the access tokens.</p>
    pub fn set_tunnel_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_tunnel_id(input);
        self
    }
    /// <p>The tunnel for which you want to rotate the access tokens.</p>
    pub fn get_tunnel_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_tunnel_id()
    }
    /// <p>The mode of the client that will use the client token, which can be either the source or destination, or both source and destination.</p>
    pub fn client_mode(mut self, input: crate::types::ClientMode) -> Self {
        self.inner = self.inner.client_mode(input);
        self
    }
    /// <p>The mode of the client that will use the client token, which can be either the source or destination, or both source and destination.</p>
    pub fn set_client_mode(mut self, input: ::std::option::Option<crate::types::ClientMode>) -> Self {
        self.inner = self.inner.set_client_mode(input);
        self
    }
    /// <p>The mode of the client that will use the client token, which can be either the source or destination, or both source and destination.</p>
    pub fn get_client_mode(&self) -> &::std::option::Option<crate::types::ClientMode> {
        self.inner.get_client_mode()
    }
    /// <p>The destination configuration.</p>
    pub fn destination_config(mut self, input: crate::types::DestinationConfig) -> Self {
        self.inner = self.inner.destination_config(input);
        self
    }
    /// <p>The destination configuration.</p>
    pub fn set_destination_config(mut self, input: ::std::option::Option<crate::types::DestinationConfig>) -> Self {
        self.inner = self.inner.set_destination_config(input);
        self
    }
    /// <p>The destination configuration.</p>
    pub fn get_destination_config(&self) -> &::std::option::Option<crate::types::DestinationConfig> {
        self.inner.get_destination_config()
    }
}
