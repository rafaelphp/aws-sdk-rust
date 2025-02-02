// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_resource_evaluation_summary::_get_resource_evaluation_summary_output::GetResourceEvaluationSummaryOutputBuilder;

pub use crate::operation::get_resource_evaluation_summary::_get_resource_evaluation_summary_input::GetResourceEvaluationSummaryInputBuilder;

impl GetResourceEvaluationSummaryInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_resource_evaluation_summary::GetResourceEvaluationSummaryOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_resource_evaluation_summary::GetResourceEvaluationSummaryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_resource_evaluation_summary();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetResourceEvaluationSummary`.
///
/// <p>Returns a summary of resource evaluation for the specified resource evaluation ID from the proactive rules that were run. The results indicate which evaluation context was used to evaluate the rules, which resource details were evaluated, the evaluation mode that was run, and whether the resource details comply with the configuration of the proactive rules. </p> <note>
/// <p>To see additional information about the evaluation result, such as which rule flagged a resource as NON_COMPLIANT, use the <a href="https://docs.aws.amazon.com/config/latest/APIReference/API_GetComplianceDetailsByResource.html">GetComplianceDetailsByResource</a> API. For more information, see the <a href="https://docs.aws.amazon.com/config/latest/APIReference/API_GetResourceEvaluationSummary.html#API_GetResourceEvaluationSummary_Examples">Examples</a> section.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetResourceEvaluationSummaryFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_resource_evaluation_summary::builders::GetResourceEvaluationSummaryInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl GetResourceEvaluationSummaryFluentBuilder {
    /// Creates a new `GetResourceEvaluationSummary`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetResourceEvaluationSummary as a reference.
    pub fn as_input(&self) -> &crate::operation::get_resource_evaluation_summary::builders::GetResourceEvaluationSummaryInputBuilder {
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
        crate::operation::get_resource_evaluation_summary::GetResourceEvaluationSummaryOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_resource_evaluation_summary::GetResourceEvaluationSummaryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_resource_evaluation_summary::GetResourceEvaluationSummary::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_resource_evaluation_summary::GetResourceEvaluationSummary::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::get_resource_evaluation_summary::GetResourceEvaluationSummaryOutput,
            crate::operation::get_resource_evaluation_summary::GetResourceEvaluationSummaryError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_resource_evaluation_summary::GetResourceEvaluationSummaryError>,
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
    /// <p>The unique <code>ResourceEvaluationId</code> of Amazon Web Services resource execution for which you want to retrieve the evaluation summary.</p>
    pub fn resource_evaluation_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_evaluation_id(input.into());
        self
    }
    /// <p>The unique <code>ResourceEvaluationId</code> of Amazon Web Services resource execution for which you want to retrieve the evaluation summary.</p>
    pub fn set_resource_evaluation_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_evaluation_id(input);
        self
    }
    /// <p>The unique <code>ResourceEvaluationId</code> of Amazon Web Services resource execution for which you want to retrieve the evaluation summary.</p>
    pub fn get_resource_evaluation_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_evaluation_id()
    }
}
