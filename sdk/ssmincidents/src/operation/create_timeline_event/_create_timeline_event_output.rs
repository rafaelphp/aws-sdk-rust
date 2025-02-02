// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateTimelineEventOutput {
    /// <p>The ARN of the incident record that you added the event to.</p>
    pub incident_record_arn: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the event for easy reference later. </p>
    pub event_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateTimelineEventOutput {
    /// <p>The ARN of the incident record that you added the event to.</p>
    pub fn incident_record_arn(&self) -> ::std::option::Option<&str> {
        self.incident_record_arn.as_deref()
    }
    /// <p>The ID of the event for easy reference later. </p>
    pub fn event_id(&self) -> ::std::option::Option<&str> {
        self.event_id.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for CreateTimelineEventOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateTimelineEventOutput {
    /// Creates a new builder-style object to manufacture [`CreateTimelineEventOutput`](crate::operation::create_timeline_event::CreateTimelineEventOutput).
    pub fn builder() -> crate::operation::create_timeline_event::builders::CreateTimelineEventOutputBuilder {
        crate::operation::create_timeline_event::builders::CreateTimelineEventOutputBuilder::default()
    }
}

/// A builder for [`CreateTimelineEventOutput`](crate::operation::create_timeline_event::CreateTimelineEventOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateTimelineEventOutputBuilder {
    pub(crate) incident_record_arn: ::std::option::Option<::std::string::String>,
    pub(crate) event_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateTimelineEventOutputBuilder {
    /// <p>The ARN of the incident record that you added the event to.</p>
    pub fn incident_record_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.incident_record_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the incident record that you added the event to.</p>
    pub fn set_incident_record_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.incident_record_arn = input;
        self
    }
    /// <p>The ARN of the incident record that you added the event to.</p>
    pub fn get_incident_record_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.incident_record_arn
    }
    /// <p>The ID of the event for easy reference later. </p>
    pub fn event_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.event_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the event for easy reference later. </p>
    pub fn set_event_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.event_id = input;
        self
    }
    /// <p>The ID of the event for easy reference later. </p>
    pub fn get_event_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.event_id
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateTimelineEventOutput`](crate::operation::create_timeline_event::CreateTimelineEventOutput).
    pub fn build(self) -> crate::operation::create_timeline_event::CreateTimelineEventOutput {
        crate::operation::create_timeline_event::CreateTimelineEventOutput {
            incident_record_arn: self.incident_record_arn,
            event_id: self.event_id,
            _request_id: self._request_id,
        }
    }
}
