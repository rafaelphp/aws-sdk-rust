// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_configured_table_association::_update_configured_table_association_output::UpdateConfiguredTableAssociationOutputBuilder;

pub use crate::operation::update_configured_table_association::_update_configured_table_association_input::UpdateConfiguredTableAssociationInputBuilder;

impl UpdateConfiguredTableAssociationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_configured_table_association::UpdateConfiguredTableAssociationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_configured_table_association::UpdateConfiguredTableAssociationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_configured_table_association();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateConfiguredTableAssociation`.
///
/// <p>Updates a configured table association.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateConfiguredTableAssociationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_configured_table_association::builders::UpdateConfiguredTableAssociationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl UpdateConfiguredTableAssociationFluentBuilder {
    /// Creates a new `UpdateConfiguredTableAssociation`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateConfiguredTableAssociation as a reference.
    pub fn as_input(&self) -> &crate::operation::update_configured_table_association::builders::UpdateConfiguredTableAssociationInputBuilder {
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
        crate::operation::update_configured_table_association::UpdateConfiguredTableAssociationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_configured_table_association::UpdateConfiguredTableAssociationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_configured_table_association::UpdateConfiguredTableAssociation::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_configured_table_association::UpdateConfiguredTableAssociation::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_configured_table_association::UpdateConfiguredTableAssociationOutput,
            crate::operation::update_configured_table_association::UpdateConfiguredTableAssociationError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_configured_table_association::UpdateConfiguredTableAssociationError>,
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
    /// <p>The unique identifier for the configured table association to update. Currently accepts the configured table association ID.</p>
    pub fn configured_table_association_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.configured_table_association_identifier(input.into());
        self
    }
    /// <p>The unique identifier for the configured table association to update. Currently accepts the configured table association ID.</p>
    pub fn set_configured_table_association_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_configured_table_association_identifier(input);
        self
    }
    /// <p>The unique identifier for the configured table association to update. Currently accepts the configured table association ID.</p>
    pub fn get_configured_table_association_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_configured_table_association_identifier()
    }
    /// <p>The unique ID for the membership that the configured table association belongs to.</p>
    pub fn membership_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.membership_identifier(input.into());
        self
    }
    /// <p>The unique ID for the membership that the configured table association belongs to.</p>
    pub fn set_membership_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_membership_identifier(input);
        self
    }
    /// <p>The unique ID for the membership that the configured table association belongs to.</p>
    pub fn get_membership_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_membership_identifier()
    }
    /// <p>A new description for the configured table association.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A new description for the configured table association.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A new description for the configured table association.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The service will assume this role to access catalog metadata and query the table.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The service will assume this role to access catalog metadata and query the table.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>The service will assume this role to access catalog metadata and query the table.</p>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_role_arn()
    }
}
