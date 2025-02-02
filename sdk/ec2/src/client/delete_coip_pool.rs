// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteCoipPool`](crate::operation::delete_coip_pool::builders::DeleteCoipPoolFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`coip_pool_id(impl Into<String>)`](crate::operation::delete_coip_pool::builders::DeleteCoipPoolFluentBuilder::coip_pool_id) / [`set_coip_pool_id(Option<String>)`](crate::operation::delete_coip_pool::builders::DeleteCoipPoolFluentBuilder::set_coip_pool_id): <p>The ID of the CoIP pool that you want to delete. </p>
    ///   - [`dry_run(bool)`](crate::operation::delete_coip_pool::builders::DeleteCoipPoolFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::delete_coip_pool::builders::DeleteCoipPoolFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`DeleteCoipPoolOutput`](crate::operation::delete_coip_pool::DeleteCoipPoolOutput) with field(s):
    ///   - [`coip_pool(Option<CoipPool>)`](crate::operation::delete_coip_pool::DeleteCoipPoolOutput::coip_pool): <p>Information about the CoIP address pool.</p>
    /// - On failure, responds with [`SdkError<DeleteCoipPoolError>`](crate::operation::delete_coip_pool::DeleteCoipPoolError)
    pub fn delete_coip_pool(&self) -> crate::operation::delete_coip_pool::builders::DeleteCoipPoolFluentBuilder {
        crate::operation::delete_coip_pool::builders::DeleteCoipPoolFluentBuilder::new(self.handle.clone())
    }
}
