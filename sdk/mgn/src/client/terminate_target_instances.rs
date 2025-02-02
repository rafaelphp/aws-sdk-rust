// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`TerminateTargetInstances`](crate::operation::terminate_target_instances::builders::TerminateTargetInstancesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`source_server_i_ds(impl Into<String>)`](crate::operation::terminate_target_instances::builders::TerminateTargetInstancesFluentBuilder::source_server_i_ds) / [`set_source_server_i_ds(Option<Vec<String>>)`](crate::operation::terminate_target_instances::builders::TerminateTargetInstancesFluentBuilder::set_source_server_i_ds): <p>Terminate Target instance by Source Server IDs.</p>
    ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::terminate_target_instances::builders::TerminateTargetInstancesFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::terminate_target_instances::builders::TerminateTargetInstancesFluentBuilder::set_tags): <p>Terminate Target instance by Tags.</p>
    ///   - [`account_id(impl Into<String>)`](crate::operation::terminate_target_instances::builders::TerminateTargetInstancesFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::terminate_target_instances::builders::TerminateTargetInstancesFluentBuilder::set_account_id): <p>Terminate Target instance by Account ID</p>
    /// - On success, responds with [`TerminateTargetInstancesOutput`](crate::operation::terminate_target_instances::TerminateTargetInstancesOutput) with field(s):
    ///   - [`job(Option<Job>)`](crate::operation::terminate_target_instances::TerminateTargetInstancesOutput::job): <p>Terminate Target instance Job response.</p>
    /// - On failure, responds with [`SdkError<TerminateTargetInstancesError>`](crate::operation::terminate_target_instances::TerminateTargetInstancesError)
    pub fn terminate_target_instances(&self) -> crate::operation::terminate_target_instances::builders::TerminateTargetInstancesFluentBuilder {
        crate::operation::terminate_target_instances::builders::TerminateTargetInstancesFluentBuilder::new(self.handle.clone())
    }
}
