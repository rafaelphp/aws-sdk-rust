// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An update to a profile question.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ProfileQuestionUpdate {
    /// <p>The ID of the question.</p>
    pub question_id: ::std::option::Option<::std::string::String>,
    /// <p>The selected choices.</p>
    pub selected_choice_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ProfileQuestionUpdate {
    /// <p>The ID of the question.</p>
    pub fn question_id(&self) -> ::std::option::Option<&str> {
        self.question_id.as_deref()
    }
    /// <p>The selected choices.</p>
    pub fn selected_choice_ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.selected_choice_ids.as_deref()
    }
}
impl ProfileQuestionUpdate {
    /// Creates a new builder-style object to manufacture [`ProfileQuestionUpdate`](crate::types::ProfileQuestionUpdate).
    pub fn builder() -> crate::types::builders::ProfileQuestionUpdateBuilder {
        crate::types::builders::ProfileQuestionUpdateBuilder::default()
    }
}

/// A builder for [`ProfileQuestionUpdate`](crate::types::ProfileQuestionUpdate).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ProfileQuestionUpdateBuilder {
    pub(crate) question_id: ::std::option::Option<::std::string::String>,
    pub(crate) selected_choice_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ProfileQuestionUpdateBuilder {
    /// <p>The ID of the question.</p>
    pub fn question_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.question_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the question.</p>
    pub fn set_question_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.question_id = input;
        self
    }
    /// <p>The ID of the question.</p>
    pub fn get_question_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.question_id
    }
    /// Appends an item to `selected_choice_ids`.
    ///
    /// To override the contents of this collection use [`set_selected_choice_ids`](Self::set_selected_choice_ids).
    ///
    /// <p>The selected choices.</p>
    pub fn selected_choice_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.selected_choice_ids.unwrap_or_default();
        v.push(input.into());
        self.selected_choice_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The selected choices.</p>
    pub fn set_selected_choice_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.selected_choice_ids = input;
        self
    }
    /// <p>The selected choices.</p>
    pub fn get_selected_choice_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.selected_choice_ids
    }
    /// Consumes the builder and constructs a [`ProfileQuestionUpdate`](crate::types::ProfileQuestionUpdate).
    pub fn build(self) -> crate::types::ProfileQuestionUpdate {
        crate::types::ProfileQuestionUpdate {
            question_id: self.question_id,
            selected_choice_ids: self.selected_choice_ids,
        }
    }
}
