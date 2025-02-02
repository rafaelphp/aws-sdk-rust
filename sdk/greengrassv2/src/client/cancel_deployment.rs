// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CancelDeployment`](crate::operation::cancel_deployment::builders::CancelDeploymentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`deployment_id(impl Into<String>)`](crate::operation::cancel_deployment::builders::CancelDeploymentFluentBuilder::deployment_id) / [`set_deployment_id(Option<String>)`](crate::operation::cancel_deployment::builders::CancelDeploymentFluentBuilder::set_deployment_id): <p>The ID of the deployment.</p>
    /// - On success, responds with [`CancelDeploymentOutput`](crate::operation::cancel_deployment::CancelDeploymentOutput) with field(s):
    ///   - [`message(Option<String>)`](crate::operation::cancel_deployment::CancelDeploymentOutput::message): <p>A message that communicates if the cancel was successful.</p>
    /// - On failure, responds with [`SdkError<CancelDeploymentError>`](crate::operation::cancel_deployment::CancelDeploymentError)
    pub fn cancel_deployment(&self) -> crate::operation::cancel_deployment::builders::CancelDeploymentFluentBuilder {
        crate::operation::cancel_deployment::builders::CancelDeploymentFluentBuilder::new(self.handle.clone())
    }
}
