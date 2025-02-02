// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_workflow_execution::_get_workflow_execution_output::GetWorkflowExecutionOutputBuilder;

pub use crate::operation::get_workflow_execution::_get_workflow_execution_input::GetWorkflowExecutionInputBuilder;

impl GetWorkflowExecutionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_workflow_execution::GetWorkflowExecutionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_workflow_execution::GetWorkflowExecutionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_workflow_execution();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetWorkflowExecution`.
///
/// <p>Get the runtime information that was logged for a specific runtime instance of the workflow.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetWorkflowExecutionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_workflow_execution::builders::GetWorkflowExecutionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl GetWorkflowExecutionFluentBuilder {
    /// Creates a new `GetWorkflowExecution`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetWorkflowExecution as a reference.
    pub fn as_input(&self) -> &crate::operation::get_workflow_execution::builders::GetWorkflowExecutionInputBuilder {
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
        crate::operation::get_workflow_execution::GetWorkflowExecutionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_workflow_execution::GetWorkflowExecutionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_workflow_execution::GetWorkflowExecution::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_workflow_execution::GetWorkflowExecution::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::get_workflow_execution::GetWorkflowExecutionOutput,
            crate::operation::get_workflow_execution::GetWorkflowExecutionError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_workflow_execution::GetWorkflowExecutionError>,
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
    /// <p>Use the unique identifier for a runtime instance of the workflow to get runtime details.</p>
    pub fn workflow_execution_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.workflow_execution_id(input.into());
        self
    }
    /// <p>Use the unique identifier for a runtime instance of the workflow to get runtime details.</p>
    pub fn set_workflow_execution_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_workflow_execution_id(input);
        self
    }
    /// <p>Use the unique identifier for a runtime instance of the workflow to get runtime details.</p>
    pub fn get_workflow_execution_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_workflow_execution_id()
    }
}
