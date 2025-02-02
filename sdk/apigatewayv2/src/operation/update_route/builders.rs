// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_route::_update_route_output::UpdateRouteOutputBuilder;

pub use crate::operation::update_route::_update_route_input::UpdateRouteInputBuilder;

impl UpdateRouteInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_route::UpdateRouteOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_route::UpdateRouteError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_route();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateRoute`.
///
/// <p>Updates a Route.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateRouteFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_route::builders::UpdateRouteInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl UpdateRouteFluentBuilder {
    /// Creates a new `UpdateRoute`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateRoute as a reference.
    pub fn as_input(&self) -> &crate::operation::update_route::builders::UpdateRouteInputBuilder {
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
        crate::operation::update_route::UpdateRouteOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_route::UpdateRouteError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_route::UpdateRoute::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_route::UpdateRoute::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_route::UpdateRouteOutput,
            crate::operation::update_route::UpdateRouteError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_route::UpdateRouteError>,
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
    /// <p>The API identifier.</p>
    pub fn api_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.api_id(input.into());
        self
    }
    /// <p>The API identifier.</p>
    pub fn set_api_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_api_id(input);
        self
    }
    /// <p>The API identifier.</p>
    pub fn get_api_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_api_id()
    }
    /// <p>Specifies whether an API key is required for the route. Supported only for WebSocket APIs.</p>
    pub fn api_key_required(mut self, input: bool) -> Self {
        self.inner = self.inner.api_key_required(input);
        self
    }
    /// <p>Specifies whether an API key is required for the route. Supported only for WebSocket APIs.</p>
    pub fn set_api_key_required(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_api_key_required(input);
        self
    }
    /// <p>Specifies whether an API key is required for the route. Supported only for WebSocket APIs.</p>
    pub fn get_api_key_required(&self) -> &::std::option::Option<bool> {
        self.inner.get_api_key_required()
    }
    /// Appends an item to `AuthorizationScopes`.
    ///
    /// To override the contents of this collection use [`set_authorization_scopes`](Self::set_authorization_scopes).
    ///
    /// <p>The authorization scopes supported by this route.</p>
    pub fn authorization_scopes(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.authorization_scopes(input.into());
        self
    }
    /// <p>The authorization scopes supported by this route.</p>
    pub fn set_authorization_scopes(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_authorization_scopes(input);
        self
    }
    /// <p>The authorization scopes supported by this route.</p>
    pub fn get_authorization_scopes(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_authorization_scopes()
    }
    /// <p>The authorization type for the route. For WebSocket APIs, valid values are NONE for open access, AWS_IAM for using AWS IAM permissions, and CUSTOM for using a Lambda authorizer For HTTP APIs, valid values are NONE for open access, JWT for using JSON Web Tokens, AWS_IAM for using AWS IAM permissions, and CUSTOM for using a Lambda authorizer.</p>
    pub fn authorization_type(mut self, input: crate::types::AuthorizationType) -> Self {
        self.inner = self.inner.authorization_type(input);
        self
    }
    /// <p>The authorization type for the route. For WebSocket APIs, valid values are NONE for open access, AWS_IAM for using AWS IAM permissions, and CUSTOM for using a Lambda authorizer For HTTP APIs, valid values are NONE for open access, JWT for using JSON Web Tokens, AWS_IAM for using AWS IAM permissions, and CUSTOM for using a Lambda authorizer.</p>
    pub fn set_authorization_type(mut self, input: ::std::option::Option<crate::types::AuthorizationType>) -> Self {
        self.inner = self.inner.set_authorization_type(input);
        self
    }
    /// <p>The authorization type for the route. For WebSocket APIs, valid values are NONE for open access, AWS_IAM for using AWS IAM permissions, and CUSTOM for using a Lambda authorizer For HTTP APIs, valid values are NONE for open access, JWT for using JSON Web Tokens, AWS_IAM for using AWS IAM permissions, and CUSTOM for using a Lambda authorizer.</p>
    pub fn get_authorization_type(&self) -> &::std::option::Option<crate::types::AuthorizationType> {
        self.inner.get_authorization_type()
    }
    /// <p>The identifier of the Authorizer resource to be associated with this route. The authorizer identifier is generated by API Gateway when you created the authorizer.</p>
    pub fn authorizer_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.authorizer_id(input.into());
        self
    }
    /// <p>The identifier of the Authorizer resource to be associated with this route. The authorizer identifier is generated by API Gateway when you created the authorizer.</p>
    pub fn set_authorizer_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_authorizer_id(input);
        self
    }
    /// <p>The identifier of the Authorizer resource to be associated with this route. The authorizer identifier is generated by API Gateway when you created the authorizer.</p>
    pub fn get_authorizer_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_authorizer_id()
    }
    /// <p>The model selection expression for the route. Supported only for WebSocket APIs.</p>
    pub fn model_selection_expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.model_selection_expression(input.into());
        self
    }
    /// <p>The model selection expression for the route. Supported only for WebSocket APIs.</p>
    pub fn set_model_selection_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_model_selection_expression(input);
        self
    }
    /// <p>The model selection expression for the route. Supported only for WebSocket APIs.</p>
    pub fn get_model_selection_expression(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_model_selection_expression()
    }
    /// <p>The operation name for the route.</p>
    pub fn operation_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.operation_name(input.into());
        self
    }
    /// <p>The operation name for the route.</p>
    pub fn set_operation_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_operation_name(input);
        self
    }
    /// <p>The operation name for the route.</p>
    pub fn get_operation_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_operation_name()
    }
    /// Adds a key-value pair to `RequestModels`.
    ///
    /// To override the contents of this collection use [`set_request_models`](Self::set_request_models).
    ///
    /// <p>The request models for the route. Supported only for WebSocket APIs.</p>
    pub fn request_models(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.request_models(k.into(), v.into());
        self
    }
    /// <p>The request models for the route. Supported only for WebSocket APIs.</p>
    pub fn set_request_models(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_request_models(input);
        self
    }
    /// <p>The request models for the route. Supported only for WebSocket APIs.</p>
    pub fn get_request_models(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_request_models()
    }
    /// Adds a key-value pair to `RequestParameters`.
    ///
    /// To override the contents of this collection use [`set_request_parameters`](Self::set_request_parameters).
    ///
    /// <p>The request parameters for the route. Supported only for WebSocket APIs.</p>
    pub fn request_parameters(mut self, k: impl ::std::convert::Into<::std::string::String>, v: crate::types::ParameterConstraints) -> Self {
        self.inner = self.inner.request_parameters(k.into(), v);
        self
    }
    /// <p>The request parameters for the route. Supported only for WebSocket APIs.</p>
    pub fn set_request_parameters(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::ParameterConstraints>>,
    ) -> Self {
        self.inner = self.inner.set_request_parameters(input);
        self
    }
    /// <p>The request parameters for the route. Supported only for WebSocket APIs.</p>
    pub fn get_request_parameters(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::ParameterConstraints>> {
        self.inner.get_request_parameters()
    }
    /// <p>The route ID.</p>
    pub fn route_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.route_id(input.into());
        self
    }
    /// <p>The route ID.</p>
    pub fn set_route_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_route_id(input);
        self
    }
    /// <p>The route ID.</p>
    pub fn get_route_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_route_id()
    }
    /// <p>The route key for the route.</p>
    pub fn route_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.route_key(input.into());
        self
    }
    /// <p>The route key for the route.</p>
    pub fn set_route_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_route_key(input);
        self
    }
    /// <p>The route key for the route.</p>
    pub fn get_route_key(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_route_key()
    }
    /// <p>The route response selection expression for the route. Supported only for WebSocket APIs.</p>
    pub fn route_response_selection_expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.route_response_selection_expression(input.into());
        self
    }
    /// <p>The route response selection expression for the route. Supported only for WebSocket APIs.</p>
    pub fn set_route_response_selection_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_route_response_selection_expression(input);
        self
    }
    /// <p>The route response selection expression for the route. Supported only for WebSocket APIs.</p>
    pub fn get_route_response_selection_expression(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_route_response_selection_expression()
    }
    /// <p>The target for the route.</p>
    pub fn target(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.target(input.into());
        self
    }
    /// <p>The target for the route.</p>
    pub fn set_target(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_target(input);
        self
    }
    /// <p>The target for the route.</p>
    pub fn get_target(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_target()
    }
}
