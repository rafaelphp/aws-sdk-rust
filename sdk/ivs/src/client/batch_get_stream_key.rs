// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchGetStreamKey`](crate::operation::batch_get_stream_key::builders::BatchGetStreamKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`arns(impl Into<String>)`](crate::operation::batch_get_stream_key::builders::BatchGetStreamKeyFluentBuilder::arns) / [`set_arns(Option<Vec<String>>)`](crate::operation::batch_get_stream_key::builders::BatchGetStreamKeyFluentBuilder::set_arns): <p>Array of ARNs, one per stream key.</p>
    /// - On success, responds with [`BatchGetStreamKeyOutput`](crate::operation::batch_get_stream_key::BatchGetStreamKeyOutput) with field(s):
    ///   - [`stream_keys(Option<Vec<StreamKey>>)`](crate::operation::batch_get_stream_key::BatchGetStreamKeyOutput::stream_keys): <p></p>
    ///   - [`errors(Option<Vec<BatchError>>)`](crate::operation::batch_get_stream_key::BatchGetStreamKeyOutput::errors): <p></p>
    /// - On failure, responds with [`SdkError<BatchGetStreamKeyError>`](crate::operation::batch_get_stream_key::BatchGetStreamKeyError)
    pub fn batch_get_stream_key(&self) -> crate::operation::batch_get_stream_key::builders::BatchGetStreamKeyFluentBuilder {
        crate::operation::batch_get_stream_key::builders::BatchGetStreamKeyFluentBuilder::new(self.handle.clone())
    }
}
