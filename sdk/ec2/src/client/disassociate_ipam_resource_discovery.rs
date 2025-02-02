// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisassociateIpamResourceDiscovery`](crate::operation::disassociate_ipam_resource_discovery::builders::DisassociateIpamResourceDiscoveryFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::disassociate_ipam_resource_discovery::builders::DisassociateIpamResourceDiscoveryFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::disassociate_ipam_resource_discovery::builders::DisassociateIpamResourceDiscoveryFluentBuilder::set_dry_run): <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`ipam_resource_discovery_association_id(impl Into<String>)`](crate::operation::disassociate_ipam_resource_discovery::builders::DisassociateIpamResourceDiscoveryFluentBuilder::ipam_resource_discovery_association_id) / [`set_ipam_resource_discovery_association_id(Option<String>)`](crate::operation::disassociate_ipam_resource_discovery::builders::DisassociateIpamResourceDiscoveryFluentBuilder::set_ipam_resource_discovery_association_id): <p>A resource discovery association ID.</p>
    /// - On success, responds with [`DisassociateIpamResourceDiscoveryOutput`](crate::operation::disassociate_ipam_resource_discovery::DisassociateIpamResourceDiscoveryOutput) with field(s):
    ///   - [`ipam_resource_discovery_association(Option<IpamResourceDiscoveryAssociation>)`](crate::operation::disassociate_ipam_resource_discovery::DisassociateIpamResourceDiscoveryOutput::ipam_resource_discovery_association): <p>A resource discovery association.</p>
    /// - On failure, responds with [`SdkError<DisassociateIpamResourceDiscoveryError>`](crate::operation::disassociate_ipam_resource_discovery::DisassociateIpamResourceDiscoveryError)
    pub fn disassociate_ipam_resource_discovery(
        &self,
    ) -> crate::operation::disassociate_ipam_resource_discovery::builders::DisassociateIpamResourceDiscoveryFluentBuilder {
        crate::operation::disassociate_ipam_resource_discovery::builders::DisassociateIpamResourceDiscoveryFluentBuilder::new(self.handle.clone())
    }
}
