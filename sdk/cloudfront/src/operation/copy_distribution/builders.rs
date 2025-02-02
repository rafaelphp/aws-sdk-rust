// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::copy_distribution::_copy_distribution_output::CopyDistributionOutputBuilder;

pub use crate::operation::copy_distribution::_copy_distribution_input::CopyDistributionInputBuilder;

impl CopyDistributionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::copy_distribution::CopyDistributionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::copy_distribution::CopyDistributionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.copy_distribution();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CopyDistribution`.
///
/// <p>Creates a staging distribution using the configuration of the provided primary distribution. A staging distribution is a copy of an existing distribution (called the primary distribution) that you can use in a continuous deployment workflow.</p>
/// <p>After you create a staging distribution, you can use <code>UpdateDistribution</code> to modify the staging distribution's configuration. Then you can use <code>CreateContinuousDeploymentPolicy</code> to incrementally move traffic to the staging distribution.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CopyDistributionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::copy_distribution::builders::CopyDistributionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl CopyDistributionFluentBuilder {
    /// Creates a new `CopyDistribution`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CopyDistribution as a reference.
    pub fn as_input(&self) -> &crate::operation::copy_distribution::builders::CopyDistributionInputBuilder {
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
        crate::operation::copy_distribution::CopyDistributionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::copy_distribution::CopyDistributionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::copy_distribution::CopyDistribution::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::copy_distribution::CopyDistribution::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::copy_distribution::CopyDistributionOutput,
            crate::operation::copy_distribution::CopyDistributionError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::copy_distribution::CopyDistributionError>,
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
    /// <p>The identifier of the primary distribution whose configuration you are copying. To get a distribution ID, use <code>ListDistributions</code>.</p>
    pub fn primary_distribution_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.primary_distribution_id(input.into());
        self
    }
    /// <p>The identifier of the primary distribution whose configuration you are copying. To get a distribution ID, use <code>ListDistributions</code>.</p>
    pub fn set_primary_distribution_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_primary_distribution_id(input);
        self
    }
    /// <p>The identifier of the primary distribution whose configuration you are copying. To get a distribution ID, use <code>ListDistributions</code>.</p>
    pub fn get_primary_distribution_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_primary_distribution_id()
    }
    /// <p>The type of distribution that your primary distribution will be copied to. The only valid value is <code>True</code>, indicating that you are copying to a staging distribution.</p>
    pub fn staging(mut self, input: bool) -> Self {
        self.inner = self.inner.staging(input);
        self
    }
    /// <p>The type of distribution that your primary distribution will be copied to. The only valid value is <code>True</code>, indicating that you are copying to a staging distribution.</p>
    pub fn set_staging(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_staging(input);
        self
    }
    /// <p>The type of distribution that your primary distribution will be copied to. The only valid value is <code>True</code>, indicating that you are copying to a staging distribution.</p>
    pub fn get_staging(&self) -> &::std::option::Option<bool> {
        self.inner.get_staging()
    }
    /// <p>The version identifier of the primary distribution whose configuration you are copying. This is the <code>ETag</code> value returned in the response to <code>GetDistribution</code> and <code>GetDistributionConfig</code>.</p>
    pub fn if_match(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.if_match(input.into());
        self
    }
    /// <p>The version identifier of the primary distribution whose configuration you are copying. This is the <code>ETag</code> value returned in the response to <code>GetDistribution</code> and <code>GetDistributionConfig</code>.</p>
    pub fn set_if_match(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_if_match(input);
        self
    }
    /// <p>The version identifier of the primary distribution whose configuration you are copying. This is the <code>ETag</code> value returned in the response to <code>GetDistribution</code> and <code>GetDistributionConfig</code>.</p>
    pub fn get_if_match(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_if_match()
    }
    /// <p>A value that uniquely identifies a request to create a resource. This helps to prevent CloudFront from creating a duplicate resource if you accidentally resubmit an identical request.</p>
    pub fn caller_reference(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.caller_reference(input.into());
        self
    }
    /// <p>A value that uniquely identifies a request to create a resource. This helps to prevent CloudFront from creating a duplicate resource if you accidentally resubmit an identical request.</p>
    pub fn set_caller_reference(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_caller_reference(input);
        self
    }
    /// <p>A value that uniquely identifies a request to create a resource. This helps to prevent CloudFront from creating a duplicate resource if you accidentally resubmit an identical request.</p>
    pub fn get_caller_reference(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_caller_reference()
    }
}
