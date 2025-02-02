// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RemoveRegionsFromReplication`](crate::operation::remove_regions_from_replication::builders::RemoveRegionsFromReplicationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`secret_id(impl Into<String>)`](crate::operation::remove_regions_from_replication::builders::RemoveRegionsFromReplicationFluentBuilder::secret_id) / [`set_secret_id(Option<String>)`](crate::operation::remove_regions_from_replication::builders::RemoveRegionsFromReplicationFluentBuilder::set_secret_id): <p>The ARN or name of the secret.</p>
    ///   - [`remove_replica_regions(impl Into<String>)`](crate::operation::remove_regions_from_replication::builders::RemoveRegionsFromReplicationFluentBuilder::remove_replica_regions) / [`set_remove_replica_regions(Option<Vec<String>>)`](crate::operation::remove_regions_from_replication::builders::RemoveRegionsFromReplicationFluentBuilder::set_remove_replica_regions): <p>The Regions of the replicas to remove.</p>
    /// - On success, responds with [`RemoveRegionsFromReplicationOutput`](crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationOutput::arn): <p>The ARN of the primary secret.</p>
    ///   - [`replication_status(Option<Vec<ReplicationStatusType>>)`](crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationOutput::replication_status): <p>The status of replicas for this secret after you remove Regions.</p>
    /// - On failure, responds with [`SdkError<RemoveRegionsFromReplicationError>`](crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationError)
    pub fn remove_regions_from_replication(
        &self,
    ) -> crate::operation::remove_regions_from_replication::builders::RemoveRegionsFromReplicationFluentBuilder {
        crate::operation::remove_regions_from_replication::builders::RemoveRegionsFromReplicationFluentBuilder::new(self.handle.clone())
    }
}
