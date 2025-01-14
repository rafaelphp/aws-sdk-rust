// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A structure that contains the information about one evaluation event or custom event sent to Evidently. This is a JSON payload. If this event specifies a pre-defined event type, the payload must follow the defined event schema.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Event {
    /// <p>The timestamp of the event.</p>
    pub timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p> <code>aws.evidently.evaluation</code> specifies an evaluation event, which determines which feature variation that a user sees. <code>aws.evidently.custom</code> specifies a custom event, which generates metrics from user actions such as clicks and checkouts.</p>
    pub r#type: ::std::option::Option<crate::types::EventType>,
    /// <p>The event data.</p>
    pub data: ::std::option::Option<::std::string::String>,
}
impl Event {
    /// <p>The timestamp of the event.</p>
    pub fn timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.timestamp.as_ref()
    }
    /// <p> <code>aws.evidently.evaluation</code> specifies an evaluation event, which determines which feature variation that a user sees. <code>aws.evidently.custom</code> specifies a custom event, which generates metrics from user actions such as clicks and checkouts.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::EventType> {
        self.r#type.as_ref()
    }
    /// <p>The event data.</p>
    pub fn data(&self) -> ::std::option::Option<&str> {
        self.data.as_deref()
    }
}
impl Event {
    /// Creates a new builder-style object to manufacture [`Event`](crate::types::Event).
    pub fn builder() -> crate::types::builders::EventBuilder {
        crate::types::builders::EventBuilder::default()
    }
}

/// A builder for [`Event`](crate::types::Event).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct EventBuilder {
    pub(crate) timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) r#type: ::std::option::Option<crate::types::EventType>,
    pub(crate) data: ::std::option::Option<::std::string::String>,
}
impl EventBuilder {
    /// <p>The timestamp of the event.</p>
    pub fn timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp of the event.</p>
    pub fn set_timestamp(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.timestamp = input;
        self
    }
    /// <p>The timestamp of the event.</p>
    pub fn get_timestamp(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.timestamp
    }
    /// <p> <code>aws.evidently.evaluation</code> specifies an evaluation event, which determines which feature variation that a user sees. <code>aws.evidently.custom</code> specifies a custom event, which generates metrics from user actions such as clicks and checkouts.</p>
    pub fn r#type(mut self, input: crate::types::EventType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p> <code>aws.evidently.evaluation</code> specifies an evaluation event, which determines which feature variation that a user sees. <code>aws.evidently.custom</code> specifies a custom event, which generates metrics from user actions such as clicks and checkouts.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::EventType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p> <code>aws.evidently.evaluation</code> specifies an evaluation event, which determines which feature variation that a user sees. <code>aws.evidently.custom</code> specifies a custom event, which generates metrics from user actions such as clicks and checkouts.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::EventType> {
        &self.r#type
    }
    /// <p>The event data.</p>
    pub fn data(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.data = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The event data.</p>
    pub fn set_data(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.data = input;
        self
    }
    /// <p>The event data.</p>
    pub fn get_data(&self) -> &::std::option::Option<::std::string::String> {
        &self.data
    }
    /// Consumes the builder and constructs a [`Event`](crate::types::Event).
    pub fn build(self) -> crate::types::Event {
        crate::types::Event {
            timestamp: self.timestamp,
            r#type: self.r#type,
            data: self.data,
        }
    }
}
