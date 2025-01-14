// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about a turn in a test set.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TestSetTurnRecord {
    /// <p>The record number associated with the turn.</p>
    pub record_number: ::std::option::Option<i64>,
    /// <p>The unique identifier for the conversation associated with the turn.</p>
    pub conversation_id: ::std::option::Option<::std::string::String>,
    /// <p>The number of turns that has elapsed up to that turn.</p>
    pub turn_number: ::std::option::Option<i32>,
    /// <p>Contains information about the agent or user turn depending upon type of turn.</p>
    pub turn_specification: ::std::option::Option<crate::types::TurnSpecification>,
}
impl TestSetTurnRecord {
    /// <p>The record number associated with the turn.</p>
    pub fn record_number(&self) -> ::std::option::Option<i64> {
        self.record_number
    }
    /// <p>The unique identifier for the conversation associated with the turn.</p>
    pub fn conversation_id(&self) -> ::std::option::Option<&str> {
        self.conversation_id.as_deref()
    }
    /// <p>The number of turns that has elapsed up to that turn.</p>
    pub fn turn_number(&self) -> ::std::option::Option<i32> {
        self.turn_number
    }
    /// <p>Contains information about the agent or user turn depending upon type of turn.</p>
    pub fn turn_specification(&self) -> ::std::option::Option<&crate::types::TurnSpecification> {
        self.turn_specification.as_ref()
    }
}
impl TestSetTurnRecord {
    /// Creates a new builder-style object to manufacture [`TestSetTurnRecord`](crate::types::TestSetTurnRecord).
    pub fn builder() -> crate::types::builders::TestSetTurnRecordBuilder {
        crate::types::builders::TestSetTurnRecordBuilder::default()
    }
}

/// A builder for [`TestSetTurnRecord`](crate::types::TestSetTurnRecord).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct TestSetTurnRecordBuilder {
    pub(crate) record_number: ::std::option::Option<i64>,
    pub(crate) conversation_id: ::std::option::Option<::std::string::String>,
    pub(crate) turn_number: ::std::option::Option<i32>,
    pub(crate) turn_specification: ::std::option::Option<crate::types::TurnSpecification>,
}
impl TestSetTurnRecordBuilder {
    /// <p>The record number associated with the turn.</p>
    pub fn record_number(mut self, input: i64) -> Self {
        self.record_number = ::std::option::Option::Some(input);
        self
    }
    /// <p>The record number associated with the turn.</p>
    pub fn set_record_number(mut self, input: ::std::option::Option<i64>) -> Self {
        self.record_number = input;
        self
    }
    /// <p>The record number associated with the turn.</p>
    pub fn get_record_number(&self) -> &::std::option::Option<i64> {
        &self.record_number
    }
    /// <p>The unique identifier for the conversation associated with the turn.</p>
    pub fn conversation_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.conversation_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the conversation associated with the turn.</p>
    pub fn set_conversation_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.conversation_id = input;
        self
    }
    /// <p>The unique identifier for the conversation associated with the turn.</p>
    pub fn get_conversation_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.conversation_id
    }
    /// <p>The number of turns that has elapsed up to that turn.</p>
    pub fn turn_number(mut self, input: i32) -> Self {
        self.turn_number = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of turns that has elapsed up to that turn.</p>
    pub fn set_turn_number(mut self, input: ::std::option::Option<i32>) -> Self {
        self.turn_number = input;
        self
    }
    /// <p>The number of turns that has elapsed up to that turn.</p>
    pub fn get_turn_number(&self) -> &::std::option::Option<i32> {
        &self.turn_number
    }
    /// <p>Contains information about the agent or user turn depending upon type of turn.</p>
    pub fn turn_specification(mut self, input: crate::types::TurnSpecification) -> Self {
        self.turn_specification = ::std::option::Option::Some(input);
        self
    }
    /// <p>Contains information about the agent or user turn depending upon type of turn.</p>
    pub fn set_turn_specification(mut self, input: ::std::option::Option<crate::types::TurnSpecification>) -> Self {
        self.turn_specification = input;
        self
    }
    /// <p>Contains information about the agent or user turn depending upon type of turn.</p>
    pub fn get_turn_specification(&self) -> &::std::option::Option<crate::types::TurnSpecification> {
        &self.turn_specification
    }
    /// Consumes the builder and constructs a [`TestSetTurnRecord`](crate::types::TestSetTurnRecord).
    pub fn build(self) -> crate::types::TestSetTurnRecord {
        crate::types::TestSetTurnRecord {
            record_number: self.record_number,
            conversation_id: self.conversation_id,
            turn_number: self.turn_number,
            turn_specification: self.turn_specification,
        }
    }
}
