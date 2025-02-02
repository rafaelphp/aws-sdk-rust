// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_dashboard_snapshot_job::_describe_dashboard_snapshot_job_output::DescribeDashboardSnapshotJobOutputBuilder;

pub use crate::operation::describe_dashboard_snapshot_job::_describe_dashboard_snapshot_job_input::DescribeDashboardSnapshotJobInputBuilder;

impl DescribeDashboardSnapshotJobInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_dashboard_snapshot_job::DescribeDashboardSnapshotJobOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_dashboard_snapshot_job::DescribeDashboardSnapshotJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_dashboard_snapshot_job();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeDashboardSnapshotJob`.
///
/// <p>Describes an existing snapshot job.</p>
/// <p>Poll job descriptions after a job starts to know the status of the job. For information on available status codes, see <code>JobStatus</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeDashboardSnapshotJobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_dashboard_snapshot_job::builders::DescribeDashboardSnapshotJobInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DescribeDashboardSnapshotJobFluentBuilder {
    /// Creates a new `DescribeDashboardSnapshotJob`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeDashboardSnapshotJob as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_dashboard_snapshot_job::builders::DescribeDashboardSnapshotJobInputBuilder {
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
        crate::operation::describe_dashboard_snapshot_job::DescribeDashboardSnapshotJobOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_dashboard_snapshot_job::DescribeDashboardSnapshotJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_dashboard_snapshot_job::DescribeDashboardSnapshotJob::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_dashboard_snapshot_job::DescribeDashboardSnapshotJob::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::describe_dashboard_snapshot_job::DescribeDashboardSnapshotJobOutput,
            crate::operation::describe_dashboard_snapshot_job::DescribeDashboardSnapshotJobError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::describe_dashboard_snapshot_job::DescribeDashboardSnapshotJobError>,
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
    /// <p>The ID of the Amazon Web Services account that the dashboard snapshot job is executed in.</p>
    pub fn aws_account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.aws_account_id(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that the dashboard snapshot job is executed in.</p>
    pub fn set_aws_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_aws_account_id(input);
        self
    }
    /// <p>The ID of the Amazon Web Services account that the dashboard snapshot job is executed in.</p>
    pub fn get_aws_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_aws_account_id()
    }
    /// <p>The ID of the dashboard that you have started a snapshot job for.</p>
    pub fn dashboard_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.dashboard_id(input.into());
        self
    }
    /// <p>The ID of the dashboard that you have started a snapshot job for.</p>
    pub fn set_dashboard_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_dashboard_id(input);
        self
    }
    /// <p>The ID of the dashboard that you have started a snapshot job for.</p>
    pub fn get_dashboard_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_dashboard_id()
    }
    /// <p>The ID of the job to be described. The job ID is set when you start a new job with a <code>StartDashboardSnapshotJob</code> API call.</p>
    pub fn snapshot_job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.snapshot_job_id(input.into());
        self
    }
    /// <p>The ID of the job to be described. The job ID is set when you start a new job with a <code>StartDashboardSnapshotJob</code> API call.</p>
    pub fn set_snapshot_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_snapshot_job_id(input);
        self
    }
    /// <p>The ID of the job to be described. The job ID is set when you start a new job with a <code>StartDashboardSnapshotJob</code> API call.</p>
    pub fn get_snapshot_job_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_snapshot_job_id()
    }
}
