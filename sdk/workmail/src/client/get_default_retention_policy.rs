// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetDefaultRetentionPolicy`](crate::operation::get_default_retention_policy::builders::GetDefaultRetentionPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`organization_id(impl Into<String>)`](crate::operation::get_default_retention_policy::builders::GetDefaultRetentionPolicyFluentBuilder::organization_id) / [`set_organization_id(Option<String>)`](crate::operation::get_default_retention_policy::builders::GetDefaultRetentionPolicyFluentBuilder::set_organization_id): <p>The organization ID.</p>
    /// - On success, responds with [`GetDefaultRetentionPolicyOutput`](crate::operation::get_default_retention_policy::GetDefaultRetentionPolicyOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::operation::get_default_retention_policy::GetDefaultRetentionPolicyOutput::id): <p>The retention policy ID.</p>
    ///   - [`name(Option<String>)`](crate::operation::get_default_retention_policy::GetDefaultRetentionPolicyOutput::name): <p>The retention policy name.</p>
    ///   - [`description(Option<String>)`](crate::operation::get_default_retention_policy::GetDefaultRetentionPolicyOutput::description): <p>The retention policy description.</p>
    ///   - [`folder_configurations(Option<Vec<FolderConfiguration>>)`](crate::operation::get_default_retention_policy::GetDefaultRetentionPolicyOutput::folder_configurations): <p>The retention policy folder configurations.</p>
    /// - On failure, responds with [`SdkError<GetDefaultRetentionPolicyError>`](crate::operation::get_default_retention_policy::GetDefaultRetentionPolicyError)
    pub fn get_default_retention_policy(&self) -> crate::operation::get_default_retention_policy::builders::GetDefaultRetentionPolicyFluentBuilder {
        crate::operation::get_default_retention_policy::builders::GetDefaultRetentionPolicyFluentBuilder::new(self.handle.clone())
    }
}
