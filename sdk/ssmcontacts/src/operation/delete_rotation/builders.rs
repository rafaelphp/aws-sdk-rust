// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_rotation::_delete_rotation_output::DeleteRotationOutputBuilder;

pub use crate::operation::delete_rotation::_delete_rotation_input::DeleteRotationInputBuilder;

impl DeleteRotationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_rotation::DeleteRotationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_rotation::DeleteRotationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_rotation();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteRotation`.
///
/// <p>Deletes a rotation from the system. If a rotation belongs to more than one on-call schedule, this operation deletes it from all of them.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteRotationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_rotation::builders::DeleteRotationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DeleteRotationFluentBuilder {
    /// Creates a new `DeleteRotation`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteRotation as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_rotation::builders::DeleteRotationInputBuilder {
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
        crate::operation::delete_rotation::DeleteRotationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_rotation::DeleteRotationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_rotation::DeleteRotation::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_rotation::DeleteRotation::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::delete_rotation::DeleteRotationOutput,
            crate::operation::delete_rotation::DeleteRotationError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::delete_rotation::DeleteRotationError>,
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
    /// <p>The Amazon Resource Name (ARN) of the on-call rotation to delete.</p>
    pub fn rotation_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.rotation_id(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the on-call rotation to delete.</p>
    pub fn set_rotation_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_rotation_id(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the on-call rotation to delete.</p>
    pub fn get_rotation_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_rotation_id()
    }
}
