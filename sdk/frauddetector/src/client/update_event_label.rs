// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateEventLabel`](crate::operation::update_event_label::builders::UpdateEventLabelFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`event_id(impl Into<String>)`](crate::operation::update_event_label::builders::UpdateEventLabelFluentBuilder::event_id) / [`set_event_id(Option<String>)`](crate::operation::update_event_label::builders::UpdateEventLabelFluentBuilder::set_event_id): <p>The ID of the event associated with the label to update.</p>
    ///   - [`event_type_name(impl Into<String>)`](crate::operation::update_event_label::builders::UpdateEventLabelFluentBuilder::event_type_name) / [`set_event_type_name(Option<String>)`](crate::operation::update_event_label::builders::UpdateEventLabelFluentBuilder::set_event_type_name): <p>The event type of the event associated with the label to update.</p>
    ///   - [`assigned_label(impl Into<String>)`](crate::operation::update_event_label::builders::UpdateEventLabelFluentBuilder::assigned_label) / [`set_assigned_label(Option<String>)`](crate::operation::update_event_label::builders::UpdateEventLabelFluentBuilder::set_assigned_label): <p>The new label to assign to the event.</p>
    ///   - [`label_timestamp(impl Into<String>)`](crate::operation::update_event_label::builders::UpdateEventLabelFluentBuilder::label_timestamp) / [`set_label_timestamp(Option<String>)`](crate::operation::update_event_label::builders::UpdateEventLabelFluentBuilder::set_label_timestamp): <p>The timestamp associated with the label. The timestamp must be specified using ISO 8601 standard in UTC. </p>
    /// - On success, responds with [`UpdateEventLabelOutput`](crate::operation::update_event_label::UpdateEventLabelOutput)
    /// - On failure, responds with [`SdkError<UpdateEventLabelError>`](crate::operation::update_event_label::UpdateEventLabelError)
    pub fn update_event_label(&self) -> crate::operation::update_event_label::builders::UpdateEventLabelFluentBuilder {
        crate::operation::update_event_label::builders::UpdateEventLabelFluentBuilder::new(self.handle.clone())
    }
}
