// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateClusterConfig`](crate::operation::update_cluster_config::builders::UpdateClusterConfigFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::update_cluster_config::builders::UpdateClusterConfigFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_cluster_config::builders::UpdateClusterConfigFluentBuilder::set_name): <p>The name of the Amazon EKS cluster to update.</p>
    ///   - [`resources_vpc_config(VpcConfigRequest)`](crate::operation::update_cluster_config::builders::UpdateClusterConfigFluentBuilder::resources_vpc_config) / [`set_resources_vpc_config(Option<VpcConfigRequest>)`](crate::operation::update_cluster_config::builders::UpdateClusterConfigFluentBuilder::set_resources_vpc_config): <p>An object representing the VPC configuration to use for an Amazon EKS cluster.</p>
    ///   - [`logging(Logging)`](crate::operation::update_cluster_config::builders::UpdateClusterConfigFluentBuilder::logging) / [`set_logging(Option<Logging>)`](crate::operation::update_cluster_config::builders::UpdateClusterConfigFluentBuilder::set_logging): <p>Enable or disable exporting the Kubernetes control plane logs for your cluster to CloudWatch Logs. By default, cluster control plane logs aren't exported to CloudWatch Logs. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/control-plane-logs.html">Amazon EKS cluster control plane logs</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p> <note>   <p>CloudWatch Logs ingestion, archive storage, and data scanning rates apply to exported control plane logs. For more information, see <a href="http://aws.amazon.com/cloudwatch/pricing/">CloudWatch Pricing</a>.</p>  </note>
    ///   - [`client_request_token(impl Into<String>)`](crate::operation::update_cluster_config::builders::UpdateClusterConfigFluentBuilder::client_request_token) / [`set_client_request_token(Option<String>)`](crate::operation::update_cluster_config::builders::UpdateClusterConfigFluentBuilder::set_client_request_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    /// - On success, responds with [`UpdateClusterConfigOutput`](crate::operation::update_cluster_config::UpdateClusterConfigOutput) with field(s):
    ///   - [`update(Option<Update>)`](crate::operation::update_cluster_config::UpdateClusterConfigOutput::update): <p>An object representing an asynchronous update.</p>
    /// - On failure, responds with [`SdkError<UpdateClusterConfigError>`](crate::operation::update_cluster_config::UpdateClusterConfigError)
    pub fn update_cluster_config(&self) -> crate::operation::update_cluster_config::builders::UpdateClusterConfigFluentBuilder {
        crate::operation::update_cluster_config::builders::UpdateClusterConfigFluentBuilder::new(self.handle.clone())
    }
}
