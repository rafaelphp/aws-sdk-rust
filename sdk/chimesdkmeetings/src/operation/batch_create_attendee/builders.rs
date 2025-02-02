// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_create_attendee::_batch_create_attendee_output::BatchCreateAttendeeOutputBuilder;

pub use crate::operation::batch_create_attendee::_batch_create_attendee_input::BatchCreateAttendeeInputBuilder;

impl BatchCreateAttendeeInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::batch_create_attendee::BatchCreateAttendeeOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_create_attendee::BatchCreateAttendeeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.batch_create_attendee();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `BatchCreateAttendee`.
///
/// <p>Creates up to 100 attendees for an active Amazon Chime SDK meeting. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BatchCreateAttendeeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::batch_create_attendee::builders::BatchCreateAttendeeInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl BatchCreateAttendeeFluentBuilder {
    /// Creates a new `BatchCreateAttendee`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the BatchCreateAttendee as a reference.
    pub fn as_input(&self) -> &crate::operation::batch_create_attendee::builders::BatchCreateAttendeeInputBuilder {
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
        crate::operation::batch_create_attendee::BatchCreateAttendeeOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_create_attendee::BatchCreateAttendeeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::batch_create_attendee::BatchCreateAttendee::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::batch_create_attendee::BatchCreateAttendee::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::batch_create_attendee::BatchCreateAttendeeOutput,
            crate::operation::batch_create_attendee::BatchCreateAttendeeError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::batch_create_attendee::BatchCreateAttendeeError>,
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
    /// <p>The Amazon Chime SDK ID of the meeting to which you're adding attendees.</p>
    pub fn meeting_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.meeting_id(input.into());
        self
    }
    /// <p>The Amazon Chime SDK ID of the meeting to which you're adding attendees.</p>
    pub fn set_meeting_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_meeting_id(input);
        self
    }
    /// <p>The Amazon Chime SDK ID of the meeting to which you're adding attendees.</p>
    pub fn get_meeting_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_meeting_id()
    }
    /// Appends an item to `Attendees`.
    ///
    /// To override the contents of this collection use [`set_attendees`](Self::set_attendees).
    ///
    /// <p>The attendee information, including attendees' IDs and join tokens.</p>
    pub fn attendees(mut self, input: crate::types::CreateAttendeeRequestItem) -> Self {
        self.inner = self.inner.attendees(input);
        self
    }
    /// <p>The attendee information, including attendees' IDs and join tokens.</p>
    pub fn set_attendees(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::CreateAttendeeRequestItem>>) -> Self {
        self.inner = self.inner.set_attendees(input);
        self
    }
    /// <p>The attendee information, including attendees' IDs and join tokens.</p>
    pub fn get_attendees(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::CreateAttendeeRequestItem>> {
        self.inner.get_attendees()
    }
}
