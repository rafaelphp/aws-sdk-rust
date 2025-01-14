// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::report_instance_status::_report_instance_status_output::ReportInstanceStatusOutputBuilder;

pub use crate::operation::report_instance_status::_report_instance_status_input::ReportInstanceStatusInputBuilder;

impl ReportInstanceStatusInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::report_instance_status::ReportInstanceStatusOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::report_instance_status::ReportInstanceStatusError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.report_instance_status();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ReportInstanceStatus`.
///
/// <p>Submits feedback about the status of an instance. The instance must be in the <code>running</code> state. If your experience with the instance differs from the instance status returned by <code>DescribeInstanceStatus</code>, use <code>ReportInstanceStatus</code> to report your experience with the instance. Amazon EC2 collects this information to improve the accuracy of status checks.</p>
/// <p>Use of this action does not change the value returned by <code>DescribeInstanceStatus</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ReportInstanceStatusFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::report_instance_status::builders::ReportInstanceStatusInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl ReportInstanceStatusFluentBuilder {
    /// Creates a new `ReportInstanceStatus`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ReportInstanceStatus as a reference.
    pub fn as_input(&self) -> &crate::operation::report_instance_status::builders::ReportInstanceStatusInputBuilder {
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
        crate::operation::report_instance_status::ReportInstanceStatusOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::report_instance_status::ReportInstanceStatusError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::report_instance_status::ReportInstanceStatus::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::report_instance_status::ReportInstanceStatus::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::report_instance_status::ReportInstanceStatusOutput,
            crate::operation::report_instance_status::ReportInstanceStatusError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::report_instance_status::ReportInstanceStatusError>,
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
    /// <p>Descriptive text about the health state of your instance.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>Descriptive text about the health state of your instance.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>Descriptive text about the health state of your instance.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
    /// <p>The time at which the reported instance health state ended.</p>
    pub fn end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.end_time(input);
        self
    }
    /// <p>The time at which the reported instance health state ended.</p>
    pub fn set_end_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_end_time(input);
        self
    }
    /// <p>The time at which the reported instance health state ended.</p>
    pub fn get_end_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_end_time()
    }
    /// Appends an item to `Instances`.
    ///
    /// To override the contents of this collection use [`set_instances`](Self::set_instances).
    ///
    /// <p>The instances.</p>
    pub fn instances(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instances(input.into());
        self
    }
    /// <p>The instances.</p>
    pub fn set_instances(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_instances(input);
        self
    }
    /// <p>The instances.</p>
    pub fn get_instances(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_instances()
    }
    /// Appends an item to `ReasonCodes`.
    ///
    /// To override the contents of this collection use [`set_reason_codes`](Self::set_reason_codes).
    ///
    /// <p>The reason codes that describe the health state of your instance.</p>
    /// <ul>
    /// <li> <p> <code>instance-stuck-in-state</code>: My instance is stuck in a state.</p> </li>
    /// <li> <p> <code>unresponsive</code>: My instance is unresponsive.</p> </li>
    /// <li> <p> <code>not-accepting-credentials</code>: My instance is not accepting my credentials.</p> </li>
    /// <li> <p> <code>password-not-available</code>: A password is not available for my instance.</p> </li>
    /// <li> <p> <code>performance-network</code>: My instance is experiencing performance problems that I believe are network related.</p> </li>
    /// <li> <p> <code>performance-instance-store</code>: My instance is experiencing performance problems that I believe are related to the instance stores.</p> </li>
    /// <li> <p> <code>performance-ebs-volume</code>: My instance is experiencing performance problems that I believe are related to an EBS volume.</p> </li>
    /// <li> <p> <code>performance-other</code>: My instance is experiencing performance problems.</p> </li>
    /// <li> <p> <code>other</code>: [explain using the description parameter]</p> </li>
    /// </ul>
    pub fn reason_codes(mut self, input: crate::types::ReportInstanceReasonCodes) -> Self {
        self.inner = self.inner.reason_codes(input);
        self
    }
    /// <p>The reason codes that describe the health state of your instance.</p>
    /// <ul>
    /// <li> <p> <code>instance-stuck-in-state</code>: My instance is stuck in a state.</p> </li>
    /// <li> <p> <code>unresponsive</code>: My instance is unresponsive.</p> </li>
    /// <li> <p> <code>not-accepting-credentials</code>: My instance is not accepting my credentials.</p> </li>
    /// <li> <p> <code>password-not-available</code>: A password is not available for my instance.</p> </li>
    /// <li> <p> <code>performance-network</code>: My instance is experiencing performance problems that I believe are network related.</p> </li>
    /// <li> <p> <code>performance-instance-store</code>: My instance is experiencing performance problems that I believe are related to the instance stores.</p> </li>
    /// <li> <p> <code>performance-ebs-volume</code>: My instance is experiencing performance problems that I believe are related to an EBS volume.</p> </li>
    /// <li> <p> <code>performance-other</code>: My instance is experiencing performance problems.</p> </li>
    /// <li> <p> <code>other</code>: [explain using the description parameter]</p> </li>
    /// </ul>
    pub fn set_reason_codes(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ReportInstanceReasonCodes>>) -> Self {
        self.inner = self.inner.set_reason_codes(input);
        self
    }
    /// <p>The reason codes that describe the health state of your instance.</p>
    /// <ul>
    /// <li> <p> <code>instance-stuck-in-state</code>: My instance is stuck in a state.</p> </li>
    /// <li> <p> <code>unresponsive</code>: My instance is unresponsive.</p> </li>
    /// <li> <p> <code>not-accepting-credentials</code>: My instance is not accepting my credentials.</p> </li>
    /// <li> <p> <code>password-not-available</code>: A password is not available for my instance.</p> </li>
    /// <li> <p> <code>performance-network</code>: My instance is experiencing performance problems that I believe are network related.</p> </li>
    /// <li> <p> <code>performance-instance-store</code>: My instance is experiencing performance problems that I believe are related to the instance stores.</p> </li>
    /// <li> <p> <code>performance-ebs-volume</code>: My instance is experiencing performance problems that I believe are related to an EBS volume.</p> </li>
    /// <li> <p> <code>performance-other</code>: My instance is experiencing performance problems.</p> </li>
    /// <li> <p> <code>other</code>: [explain using the description parameter]</p> </li>
    /// </ul>
    pub fn get_reason_codes(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ReportInstanceReasonCodes>> {
        self.inner.get_reason_codes()
    }
    /// <p>The time at which the reported instance health state began.</p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.start_time(input);
        self
    }
    /// <p>The time at which the reported instance health state began.</p>
    pub fn set_start_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_start_time(input);
        self
    }
    /// <p>The time at which the reported instance health state began.</p>
    pub fn get_start_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_start_time()
    }
    /// <p>The status of all instances listed.</p>
    pub fn status(mut self, input: crate::types::ReportStatusType) -> Self {
        self.inner = self.inner.status(input);
        self
    }
    /// <p>The status of all instances listed.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::ReportStatusType>) -> Self {
        self.inner = self.inner.set_status(input);
        self
    }
    /// <p>The status of all instances listed.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::ReportStatusType> {
        self.inner.get_status()
    }
}
