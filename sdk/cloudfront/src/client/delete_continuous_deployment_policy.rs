// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteContinuousDeploymentPolicy`](crate::operation::delete_continuous_deployment_policy::builders::DeleteContinuousDeploymentPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::operation::delete_continuous_deployment_policy::builders::DeleteContinuousDeploymentPolicyFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::delete_continuous_deployment_policy::builders::DeleteContinuousDeploymentPolicyFluentBuilder::set_id): <p>The identifier of the continuous deployment policy that you are deleting.</p>
    ///   - [`if_match(impl Into<String>)`](crate::operation::delete_continuous_deployment_policy::builders::DeleteContinuousDeploymentPolicyFluentBuilder::if_match) / [`set_if_match(Option<String>)`](crate::operation::delete_continuous_deployment_policy::builders::DeleteContinuousDeploymentPolicyFluentBuilder::set_if_match): <p>The current version (<code>ETag</code> value) of the continuous deployment policy that you are deleting.</p>
    /// - On success, responds with [`DeleteContinuousDeploymentPolicyOutput`](crate::operation::delete_continuous_deployment_policy::DeleteContinuousDeploymentPolicyOutput)
    /// - On failure, responds with [`SdkError<DeleteContinuousDeploymentPolicyError>`](crate::operation::delete_continuous_deployment_policy::DeleteContinuousDeploymentPolicyError)
    pub fn delete_continuous_deployment_policy(
        &self,
    ) -> crate::operation::delete_continuous_deployment_policy::builders::DeleteContinuousDeploymentPolicyFluentBuilder {
        crate::operation::delete_continuous_deployment_policy::builders::DeleteContinuousDeploymentPolicyFluentBuilder::new(self.handle.clone())
    }
}
