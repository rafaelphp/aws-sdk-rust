// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies whether S3 on Outposts replicates delete markers. If you specify a <code>Filter</code> element in your replication configuration, you must also include a <code>DeleteMarkerReplication</code> element. If your <code>Filter</code> includes a <code>Tag</code> element, the <code>DeleteMarkerReplication</code> element's <code>Status</code> child element must be set to <code>Disabled</code>, because S3 on Outposts does not support replicating delete markers for tag-based rules.</p>
/// <p>For more information about delete marker replication, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3OutpostsReplication.html#outposts-replication-what-is-replicated">How delete operations affect replication</a> in the <i>Amazon S3 User Guide</i>. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteMarkerReplication {
    /// <p>Indicates whether to replicate delete markers.</p>
    pub status: ::std::option::Option<crate::types::DeleteMarkerReplicationStatus>,
}
impl DeleteMarkerReplication {
    /// <p>Indicates whether to replicate delete markers.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::DeleteMarkerReplicationStatus> {
        self.status.as_ref()
    }
}
impl DeleteMarkerReplication {
    /// Creates a new builder-style object to manufacture [`DeleteMarkerReplication`](crate::types::DeleteMarkerReplication).
    pub fn builder() -> crate::types::builders::DeleteMarkerReplicationBuilder {
        crate::types::builders::DeleteMarkerReplicationBuilder::default()
    }
}

/// A builder for [`DeleteMarkerReplication`](crate::types::DeleteMarkerReplication).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteMarkerReplicationBuilder {
    pub(crate) status: ::std::option::Option<crate::types::DeleteMarkerReplicationStatus>,
}
impl DeleteMarkerReplicationBuilder {
    /// <p>Indicates whether to replicate delete markers.</p>
    pub fn status(mut self, input: crate::types::DeleteMarkerReplicationStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether to replicate delete markers.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::DeleteMarkerReplicationStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>Indicates whether to replicate delete markers.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::DeleteMarkerReplicationStatus> {
        &self.status
    }
    /// Consumes the builder and constructs a [`DeleteMarkerReplication`](crate::types::DeleteMarkerReplication).
    pub fn build(self) -> crate::types::DeleteMarkerReplication {
        crate::types::DeleteMarkerReplication { status: self.status }
    }
}
