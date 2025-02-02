// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutTraceSegments`](crate::operation::put_trace_segments::builders::PutTraceSegmentsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`trace_segment_documents(impl Into<String>)`](crate::operation::put_trace_segments::builders::PutTraceSegmentsFluentBuilder::trace_segment_documents) / [`set_trace_segment_documents(Option<Vec<String>>)`](crate::operation::put_trace_segments::builders::PutTraceSegmentsFluentBuilder::set_trace_segment_documents): <p>A string containing a JSON document defining one or more segments or subsegments.</p>
    /// - On success, responds with [`PutTraceSegmentsOutput`](crate::operation::put_trace_segments::PutTraceSegmentsOutput) with field(s):
    ///   - [`unprocessed_trace_segments(Option<Vec<UnprocessedTraceSegment>>)`](crate::operation::put_trace_segments::PutTraceSegmentsOutput::unprocessed_trace_segments): <p>Segments that failed processing.</p>
    /// - On failure, responds with [`SdkError<PutTraceSegmentsError>`](crate::operation::put_trace_segments::PutTraceSegmentsError)
    pub fn put_trace_segments(&self) -> crate::operation::put_trace_segments::builders::PutTraceSegmentsFluentBuilder {
        crate::operation::put_trace_segments::builders::PutTraceSegmentsFluentBuilder::new(self.handle.clone())
    }
}
