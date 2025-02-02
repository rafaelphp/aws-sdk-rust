// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a log event, which is a record of activity that was recorded by the application or resource being monitored.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InputLogEvent {
    /// <p>The time the event occurred, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>.</p>
    pub timestamp: ::std::option::Option<i64>,
    /// <p>The raw event message. Each log event can be no larger than 256 KB.</p>
    pub message: ::std::option::Option<::std::string::String>,
}
impl InputLogEvent {
    /// <p>The time the event occurred, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>.</p>
    pub fn timestamp(&self) -> ::std::option::Option<i64> {
        self.timestamp
    }
    /// <p>The raw event message. Each log event can be no larger than 256 KB.</p>
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl InputLogEvent {
    /// Creates a new builder-style object to manufacture [`InputLogEvent`](crate::types::InputLogEvent).
    pub fn builder() -> crate::types::builders::InputLogEventBuilder {
        crate::types::builders::InputLogEventBuilder::default()
    }
}

/// A builder for [`InputLogEvent`](crate::types::InputLogEvent).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct InputLogEventBuilder {
    pub(crate) timestamp: ::std::option::Option<i64>,
    pub(crate) message: ::std::option::Option<::std::string::String>,
}
impl InputLogEventBuilder {
    /// <p>The time the event occurred, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>.</p>
    pub fn timestamp(mut self, input: i64) -> Self {
        self.timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time the event occurred, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>.</p>
    pub fn set_timestamp(mut self, input: ::std::option::Option<i64>) -> Self {
        self.timestamp = input;
        self
    }
    /// <p>The time the event occurred, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>.</p>
    pub fn get_timestamp(&self) -> &::std::option::Option<i64> {
        &self.timestamp
    }
    /// <p>The raw event message. Each log event can be no larger than 256 KB.</p>
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The raw event message. Each log event can be no larger than 256 KB.</p>
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// <p>The raw event message. Each log event can be no larger than 256 KB.</p>
    pub fn get_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.message
    }
    /// Consumes the builder and constructs a [`InputLogEvent`](crate::types::InputLogEvent).
    pub fn build(self) -> crate::types::InputLogEvent {
        crate::types::InputLogEvent {
            timestamp: self.timestamp,
            message: self.message,
        }
    }
}
