// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchStart`](crate::operation::batch_start::builders::BatchStartFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`channel_ids(impl Into<String>)`](crate::operation::batch_start::builders::BatchStartFluentBuilder::channel_ids) / [`set_channel_ids(Option<Vec<String>>)`](crate::operation::batch_start::builders::BatchStartFluentBuilder::set_channel_ids): List of channel IDs
    ///   - [`multiplex_ids(impl Into<String>)`](crate::operation::batch_start::builders::BatchStartFluentBuilder::multiplex_ids) / [`set_multiplex_ids(Option<Vec<String>>)`](crate::operation::batch_start::builders::BatchStartFluentBuilder::set_multiplex_ids): List of multiplex IDs
    /// - On success, responds with [`BatchStartOutput`](crate::operation::batch_start::BatchStartOutput) with field(s):
    ///   - [`failed(Option<Vec<BatchFailedResultModel>>)`](crate::operation::batch_start::BatchStartOutput::failed): List of failed operations
    ///   - [`successful(Option<Vec<BatchSuccessfulResultModel>>)`](crate::operation::batch_start::BatchStartOutput::successful): List of successful operations
    /// - On failure, responds with [`SdkError<BatchStartError>`](crate::operation::batch_start::BatchStartError)
    pub fn batch_start(&self) -> crate::operation::batch_start::builders::BatchStartFluentBuilder {
        crate::operation::batch_start::builders::BatchStartFluentBuilder::new(self.handle.clone())
    }
}
