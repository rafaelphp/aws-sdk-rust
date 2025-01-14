// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_workflows::_list_workflows_output::ListWorkflowsOutputBuilder;

pub use crate::operation::list_workflows::_list_workflows_input::ListWorkflowsInputBuilder;

impl ListWorkflowsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_workflows::ListWorkflowsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_workflows::ListWorkflowsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_workflows();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListWorkflows`.
///
/// <p>Query to list all workflows.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListWorkflowsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_workflows::builders::ListWorkflowsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl ListWorkflowsFluentBuilder {
    /// Creates a new `ListWorkflows`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListWorkflows as a reference.
    pub fn as_input(&self) -> &crate::operation::list_workflows::builders::ListWorkflowsInputBuilder {
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
        crate::operation::list_workflows::ListWorkflowsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_workflows::ListWorkflowsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_workflows::ListWorkflows::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_workflows::ListWorkflows::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::list_workflows::ListWorkflowsOutput,
            crate::operation::list_workflows::ListWorkflowsError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_workflows::ListWorkflowsError>,
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
    /// <p>The unique name of the domain.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>The unique name of the domain.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// <p>The unique name of the domain.</p>
    pub fn get_domain_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_name()
    }
    /// <p>The type of workflow. The only supported value is APPFLOW_INTEGRATION.</p>
    pub fn workflow_type(mut self, input: crate::types::WorkflowType) -> Self {
        self.inner = self.inner.workflow_type(input);
        self
    }
    /// <p>The type of workflow. The only supported value is APPFLOW_INTEGRATION.</p>
    pub fn set_workflow_type(mut self, input: ::std::option::Option<crate::types::WorkflowType>) -> Self {
        self.inner = self.inner.set_workflow_type(input);
        self
    }
    /// <p>The type of workflow. The only supported value is APPFLOW_INTEGRATION.</p>
    pub fn get_workflow_type(&self) -> &::std::option::Option<crate::types::WorkflowType> {
        self.inner.get_workflow_type()
    }
    /// <p>Status of workflow execution.</p>
    pub fn status(mut self, input: crate::types::Status) -> Self {
        self.inner = self.inner.status(input);
        self
    }
    /// <p>Status of workflow execution.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::Status>) -> Self {
        self.inner = self.inner.set_status(input);
        self
    }
    /// <p>Status of workflow execution.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::Status> {
        self.inner.get_status()
    }
    /// <p>Retrieve workflows started after timestamp.</p>
    pub fn query_start_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.query_start_date(input);
        self
    }
    /// <p>Retrieve workflows started after timestamp.</p>
    pub fn set_query_start_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_query_start_date(input);
        self
    }
    /// <p>Retrieve workflows started after timestamp.</p>
    pub fn get_query_start_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_query_start_date()
    }
    /// <p>Retrieve workflows ended after timestamp.</p>
    pub fn query_end_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.query_end_date(input);
        self
    }
    /// <p>Retrieve workflows ended after timestamp.</p>
    pub fn set_query_end_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_query_end_date(input);
        self
    }
    /// <p>Retrieve workflows ended after timestamp.</p>
    pub fn get_query_end_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_query_end_date()
    }
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results to return per page.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return per page.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return per page.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
