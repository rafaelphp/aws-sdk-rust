// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_space::_update_space_output::UpdateSpaceOutputBuilder;

pub use crate::operation::update_space::_update_space_input::UpdateSpaceInputBuilder;

impl UpdateSpaceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_space::UpdateSpaceOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_space::UpdateSpaceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_space();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateSpace`.
///
/// <p>Updates the settings of a space.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateSpaceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_space::builders::UpdateSpaceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl UpdateSpaceFluentBuilder {
    /// Creates a new `UpdateSpace`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateSpace as a reference.
    pub fn as_input(&self) -> &crate::operation::update_space::builders::UpdateSpaceInputBuilder {
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
        crate::operation::update_space::UpdateSpaceOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_space::UpdateSpaceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_space::UpdateSpace::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_space::UpdateSpace::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_space::UpdateSpaceOutput,
            crate::operation::update_space::UpdateSpaceError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_space::UpdateSpaceError>,
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
    /// <p>The ID of the associated Domain.</p>
    pub fn domain_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_id(input.into());
        self
    }
    /// <p>The ID of the associated Domain.</p>
    pub fn set_domain_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_id(input);
        self
    }
    /// <p>The ID of the associated Domain.</p>
    pub fn get_domain_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_id()
    }
    /// <p>The name of the space.</p>
    pub fn space_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.space_name(input.into());
        self
    }
    /// <p>The name of the space.</p>
    pub fn set_space_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_space_name(input);
        self
    }
    /// <p>The name of the space.</p>
    pub fn get_space_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_space_name()
    }
    /// <p>A collection of space settings.</p>
    pub fn space_settings(mut self, input: crate::types::SpaceSettings) -> Self {
        self.inner = self.inner.space_settings(input);
        self
    }
    /// <p>A collection of space settings.</p>
    pub fn set_space_settings(mut self, input: ::std::option::Option<crate::types::SpaceSettings>) -> Self {
        self.inner = self.inner.set_space_settings(input);
        self
    }
    /// <p>A collection of space settings.</p>
    pub fn get_space_settings(&self) -> &::std::option::Option<crate::types::SpaceSettings> {
        self.inner.get_space_settings()
    }
}
