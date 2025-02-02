// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteFuotaTask`](crate::operation::delete_fuota_task::builders::DeleteFuotaTaskFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::operation::delete_fuota_task::builders::DeleteFuotaTaskFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::delete_fuota_task::builders::DeleteFuotaTaskFluentBuilder::set_id): <p>The ID of a FUOTA task.</p>
    /// - On success, responds with [`DeleteFuotaTaskOutput`](crate::operation::delete_fuota_task::DeleteFuotaTaskOutput)
    /// - On failure, responds with [`SdkError<DeleteFuotaTaskError>`](crate::operation::delete_fuota_task::DeleteFuotaTaskError)
    pub fn delete_fuota_task(&self) -> crate::operation::delete_fuota_task::builders::DeleteFuotaTaskFluentBuilder {
        crate::operation::delete_fuota_task::builders::DeleteFuotaTaskFluentBuilder::new(self.handle.clone())
    }
}
