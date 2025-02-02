// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StopAccessLogging`](crate::operation::stop_access_logging::builders::StopAccessLoggingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`container_name(impl Into<String>)`](crate::operation::stop_access_logging::builders::StopAccessLoggingFluentBuilder::container_name) / [`set_container_name(Option<String>)`](crate::operation::stop_access_logging::builders::StopAccessLoggingFluentBuilder::set_container_name): <p>The name of the container that you want to stop access logging on.</p>
    /// - On success, responds with [`StopAccessLoggingOutput`](crate::operation::stop_access_logging::StopAccessLoggingOutput)
    /// - On failure, responds with [`SdkError<StopAccessLoggingError>`](crate::operation::stop_access_logging::StopAccessLoggingError)
    pub fn stop_access_logging(&self) -> crate::operation::stop_access_logging::builders::StopAccessLoggingFluentBuilder {
        crate::operation::stop_access_logging::builders::StopAccessLoggingFluentBuilder::new(self.handle.clone())
    }
}
