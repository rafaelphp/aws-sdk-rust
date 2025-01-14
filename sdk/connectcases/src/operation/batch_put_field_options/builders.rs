// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_put_field_options::_batch_put_field_options_output::BatchPutFieldOptionsOutputBuilder;

pub use crate::operation::batch_put_field_options::_batch_put_field_options_input::BatchPutFieldOptionsInputBuilder;

impl BatchPutFieldOptionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::batch_put_field_options::BatchPutFieldOptionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_put_field_options::BatchPutFieldOptionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.batch_put_field_options();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `BatchPutFieldOptions`.
///
/// <p>Creates and updates a set of field options for a single select field in a Cases domain.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BatchPutFieldOptionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::batch_put_field_options::builders::BatchPutFieldOptionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl BatchPutFieldOptionsFluentBuilder {
    /// Creates a new `BatchPutFieldOptions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the BatchPutFieldOptions as a reference.
    pub fn as_input(&self) -> &crate::operation::batch_put_field_options::builders::BatchPutFieldOptionsInputBuilder {
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
        crate::operation::batch_put_field_options::BatchPutFieldOptionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_put_field_options::BatchPutFieldOptionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::batch_put_field_options::BatchPutFieldOptions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::batch_put_field_options::BatchPutFieldOptions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::batch_put_field_options::BatchPutFieldOptionsOutput,
            crate::operation::batch_put_field_options::BatchPutFieldOptionsError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::batch_put_field_options::BatchPutFieldOptionsError>,
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
    /// <p>The unique identifier of the Cases domain. </p>
    pub fn domain_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_id(input.into());
        self
    }
    /// <p>The unique identifier of the Cases domain. </p>
    pub fn set_domain_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_id(input);
        self
    }
    /// <p>The unique identifier of the Cases domain. </p>
    pub fn get_domain_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_id()
    }
    /// <p>The unique identifier of a field.</p>
    pub fn field_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.field_id(input.into());
        self
    }
    /// <p>The unique identifier of a field.</p>
    pub fn set_field_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_field_id(input);
        self
    }
    /// <p>The unique identifier of a field.</p>
    pub fn get_field_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_field_id()
    }
    /// Appends an item to `options`.
    ///
    /// To override the contents of this collection use [`set_options`](Self::set_options).
    ///
    /// <p>A list of <code>FieldOption</code> objects.</p>
    pub fn options(mut self, input: crate::types::FieldOption) -> Self {
        self.inner = self.inner.options(input);
        self
    }
    /// <p>A list of <code>FieldOption</code> objects.</p>
    pub fn set_options(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::FieldOption>>) -> Self {
        self.inner = self.inner.set_options(input);
        self
    }
    /// <p>A list of <code>FieldOption</code> objects.</p>
    pub fn get_options(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::FieldOption>> {
        self.inner.get_options()
    }
}
