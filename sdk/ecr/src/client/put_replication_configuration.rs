// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutReplicationConfiguration`](crate::operation::put_replication_configuration::builders::PutReplicationConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`replication_configuration(ReplicationConfiguration)`](crate::operation::put_replication_configuration::builders::PutReplicationConfigurationFluentBuilder::replication_configuration) / [`set_replication_configuration(Option<ReplicationConfiguration>)`](crate::operation::put_replication_configuration::builders::PutReplicationConfigurationFluentBuilder::set_replication_configuration): <p>An object representing the replication configuration for a registry.</p>
    /// - On success, responds with [`PutReplicationConfigurationOutput`](crate::operation::put_replication_configuration::PutReplicationConfigurationOutput) with field(s):
    ///   - [`replication_configuration(Option<ReplicationConfiguration>)`](crate::operation::put_replication_configuration::PutReplicationConfigurationOutput::replication_configuration): <p>The contents of the replication configuration for the registry.</p>
    /// - On failure, responds with [`SdkError<PutReplicationConfigurationError>`](crate::operation::put_replication_configuration::PutReplicationConfigurationError)
    pub fn put_replication_configuration(
        &self,
    ) -> crate::operation::put_replication_configuration::builders::PutReplicationConfigurationFluentBuilder {
        crate::operation::put_replication_configuration::builders::PutReplicationConfigurationFluentBuilder::new(self.handle.clone())
    }
}
