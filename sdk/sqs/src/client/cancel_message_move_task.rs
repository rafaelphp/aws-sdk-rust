// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CancelMessageMoveTask`](crate::operation::cancel_message_move_task::builders::CancelMessageMoveTaskFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`task_handle(impl Into<String>)`](crate::operation::cancel_message_move_task::builders::CancelMessageMoveTaskFluentBuilder::task_handle) / [`set_task_handle(Option<String>)`](crate::operation::cancel_message_move_task::builders::CancelMessageMoveTaskFluentBuilder::set_task_handle): <p>An identifier associated with a message movement task.</p>
    /// - On success, responds with [`CancelMessageMoveTaskOutput`](crate::operation::cancel_message_move_task::CancelMessageMoveTaskOutput) with field(s):
    ///   - [`approximate_number_of_messages_moved(i64)`](crate::operation::cancel_message_move_task::CancelMessageMoveTaskOutput::approximate_number_of_messages_moved): <p>The approximate number of messages already moved to the destination queue.</p>
    /// - On failure, responds with [`SdkError<CancelMessageMoveTaskError>`](crate::operation::cancel_message_move_task::CancelMessageMoveTaskError)
    pub fn cancel_message_move_task(&self) -> crate::operation::cancel_message_move_task::builders::CancelMessageMoveTaskFluentBuilder {
        crate::operation::cancel_message_move_task::builders::CancelMessageMoveTaskFluentBuilder::new(self.handle.clone())
    }
}
