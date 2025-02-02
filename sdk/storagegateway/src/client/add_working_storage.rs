// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AddWorkingStorage`](crate::operation::add_working_storage::builders::AddWorkingStorageFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`gateway_arn(impl Into<String>)`](crate::operation::add_working_storage::builders::AddWorkingStorageFluentBuilder::gateway_arn) / [`set_gateway_arn(Option<String>)`](crate::operation::add_working_storage::builders::AddWorkingStorageFluentBuilder::set_gateway_arn): <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    ///   - [`disk_ids(impl Into<String>)`](crate::operation::add_working_storage::builders::AddWorkingStorageFluentBuilder::disk_ids) / [`set_disk_ids(Option<Vec<String>>)`](crate::operation::add_working_storage::builders::AddWorkingStorageFluentBuilder::set_disk_ids): <p>An array of strings that identify disks that are to be configured as working storage. Each string has a minimum length of 1 and maximum length of 300. You can get the disk IDs from the <code>ListLocalDisks</code> API.</p>
    /// - On success, responds with [`AddWorkingStorageOutput`](crate::operation::add_working_storage::AddWorkingStorageOutput) with field(s):
    ///   - [`gateway_arn(Option<String>)`](crate::operation::add_working_storage::AddWorkingStorageOutput::gateway_arn): <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    /// - On failure, responds with [`SdkError<AddWorkingStorageError>`](crate::operation::add_working_storage::AddWorkingStorageError)
    pub fn add_working_storage(&self) -> crate::operation::add_working_storage::builders::AddWorkingStorageFluentBuilder {
        crate::operation::add_working_storage::builders::AddWorkingStorageFluentBuilder::new(self.handle.clone())
    }
}
