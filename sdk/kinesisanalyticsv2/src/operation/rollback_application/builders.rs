// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::rollback_application::_rollback_application_output::RollbackApplicationOutputBuilder;

pub use crate::operation::rollback_application::_rollback_application_input::RollbackApplicationInputBuilder;

impl RollbackApplicationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::rollback_application::RollbackApplicationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::rollback_application::RollbackApplicationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.rollback_application();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RollbackApplication`.
///
/// <p>Reverts the application to the previous running version. You can roll back an application if you suspect it is stuck in a transient status. </p>
/// <p>You can roll back an application only if it is in the <code>UPDATING</code> or <code>AUTOSCALING</code> status.</p>
/// <p>When you rollback an application, it loads state data from the last successful snapshot. If the application has no snapshots, Kinesis Data Analytics rejects the rollback request.</p>
/// <p>This action is not supported for Kinesis Data Analytics for SQL applications.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RollbackApplicationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::rollback_application::builders::RollbackApplicationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl RollbackApplicationFluentBuilder {
    /// Creates a new `RollbackApplication`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RollbackApplication as a reference.
    pub fn as_input(&self) -> &crate::operation::rollback_application::builders::RollbackApplicationInputBuilder {
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
        crate::operation::rollback_application::RollbackApplicationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::rollback_application::RollbackApplicationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::rollback_application::RollbackApplication::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::rollback_application::RollbackApplication::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::rollback_application::RollbackApplicationOutput,
            crate::operation::rollback_application::RollbackApplicationError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::rollback_application::RollbackApplicationError>,
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
    /// <p>The name of the application.</p>
    pub fn application_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.application_name(input.into());
        self
    }
    /// <p>The name of the application.</p>
    pub fn set_application_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_application_name(input);
        self
    }
    /// <p>The name of the application.</p>
    pub fn get_application_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_application_name()
    }
    /// <p>The current application version ID. You can retrieve the application version ID using <code>DescribeApplication</code>.</p>
    pub fn current_application_version_id(mut self, input: i64) -> Self {
        self.inner = self.inner.current_application_version_id(input);
        self
    }
    /// <p>The current application version ID. You can retrieve the application version ID using <code>DescribeApplication</code>.</p>
    pub fn set_current_application_version_id(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_current_application_version_id(input);
        self
    }
    /// <p>The current application version ID. You can retrieve the application version ID using <code>DescribeApplication</code>.</p>
    pub fn get_current_application_version_id(&self) -> &::std::option::Option<i64> {
        self.inner.get_current_application_version_id()
    }
}
