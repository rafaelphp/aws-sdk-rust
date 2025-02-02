// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteService`](crate::operation::delete_service::builders::DeleteServiceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster(impl Into<String>)`](crate::operation::delete_service::builders::DeleteServiceFluentBuilder::cluster) / [`set_cluster(Option<String>)`](crate::operation::delete_service::builders::DeleteServiceFluentBuilder::set_cluster): <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the service to delete. If you do not specify a cluster, the default cluster is assumed.</p>
    ///   - [`service(impl Into<String>)`](crate::operation::delete_service::builders::DeleteServiceFluentBuilder::service) / [`set_service(Option<String>)`](crate::operation::delete_service::builders::DeleteServiceFluentBuilder::set_service): <p>The name of the service to delete.</p>
    ///   - [`force(bool)`](crate::operation::delete_service::builders::DeleteServiceFluentBuilder::force) / [`set_force(Option<bool>)`](crate::operation::delete_service::builders::DeleteServiceFluentBuilder::set_force): <p>If <code>true</code>, allows you to delete a service even if it wasn't scaled down to zero tasks. It's only necessary to use this if the service uses the <code>REPLICA</code> scheduling strategy.</p>
    /// - On success, responds with [`DeleteServiceOutput`](crate::operation::delete_service::DeleteServiceOutput) with field(s):
    ///   - [`service(Option<Service>)`](crate::operation::delete_service::DeleteServiceOutput::service): <p>The full description of the deleted service.</p>
    /// - On failure, responds with [`SdkError<DeleteServiceError>`](crate::operation::delete_service::DeleteServiceError)
    pub fn delete_service(&self) -> crate::operation::delete_service::builders::DeleteServiceFluentBuilder {
        crate::operation::delete_service::builders::DeleteServiceFluentBuilder::new(self.handle.clone())
    }
}
