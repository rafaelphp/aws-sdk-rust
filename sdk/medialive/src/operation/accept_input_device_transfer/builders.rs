// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::accept_input_device_transfer::_accept_input_device_transfer_output::AcceptInputDeviceTransferOutputBuilder;

pub use crate::operation::accept_input_device_transfer::_accept_input_device_transfer_input::AcceptInputDeviceTransferInputBuilder;

impl AcceptInputDeviceTransferInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::accept_input_device_transfer::AcceptInputDeviceTransferOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::accept_input_device_transfer::AcceptInputDeviceTransferError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.accept_input_device_transfer();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AcceptInputDeviceTransfer`.
///
/// Accept an incoming input device transfer. The ownership of the device will transfer to your AWS account.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AcceptInputDeviceTransferFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::accept_input_device_transfer::builders::AcceptInputDeviceTransferInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl AcceptInputDeviceTransferFluentBuilder {
    /// Creates a new `AcceptInputDeviceTransfer`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AcceptInputDeviceTransfer as a reference.
    pub fn as_input(&self) -> &crate::operation::accept_input_device_transfer::builders::AcceptInputDeviceTransferInputBuilder {
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
        crate::operation::accept_input_device_transfer::AcceptInputDeviceTransferOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::accept_input_device_transfer::AcceptInputDeviceTransferError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::accept_input_device_transfer::AcceptInputDeviceTransfer::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::accept_input_device_transfer::AcceptInputDeviceTransfer::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::accept_input_device_transfer::AcceptInputDeviceTransferOutput,
            crate::operation::accept_input_device_transfer::AcceptInputDeviceTransferError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::accept_input_device_transfer::AcceptInputDeviceTransferError>,
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
    /// The unique ID of the input device to accept. For example, hd-123456789abcdef.
    pub fn input_device_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.input_device_id(input.into());
        self
    }
    /// The unique ID of the input device to accept. For example, hd-123456789abcdef.
    pub fn set_input_device_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_input_device_id(input);
        self
    }
    /// The unique ID of the input device to accept. For example, hd-123456789abcdef.
    pub fn get_input_device_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_input_device_id()
    }
}
