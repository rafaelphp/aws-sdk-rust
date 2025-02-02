// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_model_card_export_job::_create_model_card_export_job_output::CreateModelCardExportJobOutputBuilder;

pub use crate::operation::create_model_card_export_job::_create_model_card_export_job_input::CreateModelCardExportJobInputBuilder;

impl CreateModelCardExportJobInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_model_card_export_job::CreateModelCardExportJobOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_model_card_export_job::CreateModelCardExportJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_model_card_export_job();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateModelCardExportJob`.
///
/// <p>Creates an Amazon SageMaker Model Card export job.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateModelCardExportJobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_model_card_export_job::builders::CreateModelCardExportJobInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl CreateModelCardExportJobFluentBuilder {
    /// Creates a new `CreateModelCardExportJob`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateModelCardExportJob as a reference.
    pub fn as_input(&self) -> &crate::operation::create_model_card_export_job::builders::CreateModelCardExportJobInputBuilder {
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
        crate::operation::create_model_card_export_job::CreateModelCardExportJobOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_model_card_export_job::CreateModelCardExportJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_model_card_export_job::CreateModelCardExportJob::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_model_card_export_job::CreateModelCardExportJob::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::create_model_card_export_job::CreateModelCardExportJobOutput,
            crate::operation::create_model_card_export_job::CreateModelCardExportJobError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_model_card_export_job::CreateModelCardExportJobError>,
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
    /// <p>The name of the model card to export.</p>
    pub fn model_card_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.model_card_name(input.into());
        self
    }
    /// <p>The name of the model card to export.</p>
    pub fn set_model_card_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_model_card_name(input);
        self
    }
    /// <p>The name of the model card to export.</p>
    pub fn get_model_card_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_model_card_name()
    }
    /// <p>The version of the model card to export. If a version is not provided, then the latest version of the model card is exported.</p>
    pub fn model_card_version(mut self, input: i32) -> Self {
        self.inner = self.inner.model_card_version(input);
        self
    }
    /// <p>The version of the model card to export. If a version is not provided, then the latest version of the model card is exported.</p>
    pub fn set_model_card_version(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_model_card_version(input);
        self
    }
    /// <p>The version of the model card to export. If a version is not provided, then the latest version of the model card is exported.</p>
    pub fn get_model_card_version(&self) -> &::std::option::Option<i32> {
        self.inner.get_model_card_version()
    }
    /// <p>The name of the model card export job.</p>
    pub fn model_card_export_job_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.model_card_export_job_name(input.into());
        self
    }
    /// <p>The name of the model card export job.</p>
    pub fn set_model_card_export_job_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_model_card_export_job_name(input);
        self
    }
    /// <p>The name of the model card export job.</p>
    pub fn get_model_card_export_job_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_model_card_export_job_name()
    }
    /// <p>The model card output configuration that specifies the Amazon S3 path for exporting.</p>
    pub fn output_config(mut self, input: crate::types::ModelCardExportOutputConfig) -> Self {
        self.inner = self.inner.output_config(input);
        self
    }
    /// <p>The model card output configuration that specifies the Amazon S3 path for exporting.</p>
    pub fn set_output_config(mut self, input: ::std::option::Option<crate::types::ModelCardExportOutputConfig>) -> Self {
        self.inner = self.inner.set_output_config(input);
        self
    }
    /// <p>The model card output configuration that specifies the Amazon S3 path for exporting.</p>
    pub fn get_output_config(&self) -> &::std::option::Option<crate::types::ModelCardExportOutputConfig> {
        self.inner.get_output_config()
    }
}
