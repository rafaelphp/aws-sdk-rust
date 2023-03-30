// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UpdateClusterOutput  {
    /// <p>Returns information about the updated Elastic DocumentDB cluster.</p>
    #[doc(hidden)]
    pub cluster: std::option::Option<crate::model::Cluster>,
}
impl UpdateClusterOutput {
    /// <p>Returns information about the updated Elastic DocumentDB cluster.</p>
    pub fn cluster(&self) -> std::option::Option<& crate::model::Cluster> {
        self.cluster.as_ref()
    }
}
/// See [`UpdateClusterOutput`](crate::output::UpdateClusterOutput).
pub mod update_cluster_output {
    
    /// A builder for [`UpdateClusterOutput`](crate::output::UpdateClusterOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) cluster: std::option::Option<crate::model::Cluster>,
    }
    impl Builder {
        /// <p>Returns information about the updated Elastic DocumentDB cluster.</p>
        pub fn cluster(mut self, input: crate::model::Cluster) -> Self {
            self.cluster = Some(input);
            self
        }
        /// <p>Returns information about the updated Elastic DocumentDB cluster.</p>
        pub fn set_cluster(mut self, input: std::option::Option<crate::model::Cluster>) -> Self {
            self.cluster = input; self
        }
        /// Consumes the builder and constructs a [`UpdateClusterOutput`](crate::output::UpdateClusterOutput).
        pub fn build(self) -> crate::output::UpdateClusterOutput {
            crate::output::UpdateClusterOutput {
                cluster: self.cluster
                ,
            }
        }
    }
    
    
}
impl UpdateClusterOutput {
    /// Creates a new builder-style object to manufacture [`UpdateClusterOutput`](crate::output::UpdateClusterOutput).
    pub fn builder() -> crate::output::update_cluster_output::Builder {
        crate::output::update_cluster_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UntagResourceOutput  {
}
/// See [`UntagResourceOutput`](crate::output::UntagResourceOutput).
pub mod untag_resource_output {
    
    /// A builder for [`UntagResourceOutput`](crate::output::UntagResourceOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`UntagResourceOutput`](crate::output::UntagResourceOutput).
        pub fn build(self) -> crate::output::UntagResourceOutput {
            crate::output::UntagResourceOutput {
            }
        }
    }
    
    
}
impl UntagResourceOutput {
    /// Creates a new builder-style object to manufacture [`UntagResourceOutput`](crate::output::UntagResourceOutput).
    pub fn builder() -> crate::output::untag_resource_output::Builder {
        crate::output::untag_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct TagResourceOutput  {
}
/// See [`TagResourceOutput`](crate::output::TagResourceOutput).
pub mod tag_resource_output {
    
    /// A builder for [`TagResourceOutput`](crate::output::TagResourceOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`TagResourceOutput`](crate::output::TagResourceOutput).
        pub fn build(self) -> crate::output::TagResourceOutput {
            crate::output::TagResourceOutput {
            }
        }
    }
    
    
}
impl TagResourceOutput {
    /// Creates a new builder-style object to manufacture [`TagResourceOutput`](crate::output::TagResourceOutput).
    pub fn builder() -> crate::output::tag_resource_output::Builder {
        crate::output::tag_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct RestoreClusterFromSnapshotOutput  {
    /// <p>Returns information about a the restored Elastic DocumentDB cluster.</p>
    #[doc(hidden)]
    pub cluster: std::option::Option<crate::model::Cluster>,
}
impl RestoreClusterFromSnapshotOutput {
    /// <p>Returns information about a the restored Elastic DocumentDB cluster.</p>
    pub fn cluster(&self) -> std::option::Option<& crate::model::Cluster> {
        self.cluster.as_ref()
    }
}
/// See [`RestoreClusterFromSnapshotOutput`](crate::output::RestoreClusterFromSnapshotOutput).
pub mod restore_cluster_from_snapshot_output {
    
    /// A builder for [`RestoreClusterFromSnapshotOutput`](crate::output::RestoreClusterFromSnapshotOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) cluster: std::option::Option<crate::model::Cluster>,
    }
    impl Builder {
        /// <p>Returns information about a the restored Elastic DocumentDB cluster.</p>
        pub fn cluster(mut self, input: crate::model::Cluster) -> Self {
            self.cluster = Some(input);
            self
        }
        /// <p>Returns information about a the restored Elastic DocumentDB cluster.</p>
        pub fn set_cluster(mut self, input: std::option::Option<crate::model::Cluster>) -> Self {
            self.cluster = input; self
        }
        /// Consumes the builder and constructs a [`RestoreClusterFromSnapshotOutput`](crate::output::RestoreClusterFromSnapshotOutput).
        pub fn build(self) -> crate::output::RestoreClusterFromSnapshotOutput {
            crate::output::RestoreClusterFromSnapshotOutput {
                cluster: self.cluster
                ,
            }
        }
    }
    
    
}
impl RestoreClusterFromSnapshotOutput {
    /// Creates a new builder-style object to manufacture [`RestoreClusterFromSnapshotOutput`](crate::output::RestoreClusterFromSnapshotOutput).
    pub fn builder() -> crate::output::restore_cluster_from_snapshot_output::Builder {
        crate::output::restore_cluster_from_snapshot_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListTagsForResourceOutput  {
    /// <p>The list of tags for the specified Elastic DocumentDB resource.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl ListTagsForResourceOutput {
    /// <p>The list of tags for the specified Elastic DocumentDB resource.</p>
    pub fn tags(&self) -> std::option::Option<& std::collections::HashMap<std::string::String, std::string::String>> {
        self.tags.as_ref()
    }
}
/// See [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
pub mod list_tags_for_resource_output {
    
    /// A builder for [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) tags: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    }
    impl Builder {
        /// Adds a key-value pair to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>The list of tags for the specified Elastic DocumentDB resource.</p>
        pub fn tags(mut self, k: impl Into<std::string::String>, v: impl Into<std::string::String>) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
                            hash_map.insert(k.into(), v.into());
                            self.tags = Some(hash_map);
                            self
        }
        /// <p>The list of tags for the specified Elastic DocumentDB resource.</p>
        pub fn set_tags(mut self, input: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>) -> Self {
            self.tags = input; self
        }
        /// Consumes the builder and constructs a [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
        pub fn build(self) -> crate::output::ListTagsForResourceOutput {
            crate::output::ListTagsForResourceOutput {
                tags: self.tags
                ,
            }
        }
    }
    
    
}
impl ListTagsForResourceOutput {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
    pub fn builder() -> crate::output::list_tags_for_resource_output::Builder {
        crate::output::list_tags_for_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListClusterSnapshotsOutput  {
    /// <p>A list of Elastic DocumentDB snapshots for a specified cluster.</p>
    #[doc(hidden)]
    pub snapshots: std::option::Option<std::vec::Vec<crate::model::ClusterSnapshotInList>>,
    /// <p>The response will provide a nextToken if there is more data beyond the maxResults.</p> 
    /// <p>If there is no more data in the responce, the nextToken will not be returned.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl ListClusterSnapshotsOutput {
    /// <p>A list of Elastic DocumentDB snapshots for a specified cluster.</p>
    pub fn snapshots(&self) -> std::option::Option<& [crate::model::ClusterSnapshotInList]> {
        self.snapshots.as_deref()
    }
    /// <p>The response will provide a nextToken if there is more data beyond the maxResults.</p> 
    /// <p>If there is no more data in the responce, the nextToken will not be returned.</p>
    pub fn next_token(&self) -> std::option::Option<& str> {
        self.next_token.as_deref()
    }
}
/// See [`ListClusterSnapshotsOutput`](crate::output::ListClusterSnapshotsOutput).
pub mod list_cluster_snapshots_output {
    
    /// A builder for [`ListClusterSnapshotsOutput`](crate::output::ListClusterSnapshotsOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) snapshots: std::option::Option<std::vec::Vec<crate::model::ClusterSnapshotInList>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `snapshots`.
        ///
        /// To override the contents of this collection use [`set_snapshots`](Self::set_snapshots).
        ///
        /// <p>A list of Elastic DocumentDB snapshots for a specified cluster.</p>
        pub fn snapshots(mut self, input: crate::model::ClusterSnapshotInList) -> Self {
            let mut v = self.snapshots.unwrap_or_default();
                            v.push(input);
                            self.snapshots = Some(v);
                            self
        }
        /// <p>A list of Elastic DocumentDB snapshots for a specified cluster.</p>
        pub fn set_snapshots(mut self, input: std::option::Option<std::vec::Vec<crate::model::ClusterSnapshotInList>>) -> Self {
            self.snapshots = input; self
        }
        /// <p>The response will provide a nextToken if there is more data beyond the maxResults.</p> 
        /// <p>If there is no more data in the responce, the nextToken will not be returned.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The response will provide a nextToken if there is more data beyond the maxResults.</p> 
        /// <p>If there is no more data in the responce, the nextToken will not be returned.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input; self
        }
        /// Consumes the builder and constructs a [`ListClusterSnapshotsOutput`](crate::output::ListClusterSnapshotsOutput).
        pub fn build(self) -> crate::output::ListClusterSnapshotsOutput {
            crate::output::ListClusterSnapshotsOutput {
                snapshots: self.snapshots
                ,
                next_token: self.next_token
                ,
            }
        }
    }
    
    
}
impl ListClusterSnapshotsOutput {
    /// Creates a new builder-style object to manufacture [`ListClusterSnapshotsOutput`](crate::output::ListClusterSnapshotsOutput).
    pub fn builder() -> crate::output::list_cluster_snapshots_output::Builder {
        crate::output::list_cluster_snapshots_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListClustersOutput  {
    /// <p>A list of Elastic DocumentDB cluster.</p>
    #[doc(hidden)]
    pub clusters: std::option::Option<std::vec::Vec<crate::model::ClusterInList>>,
    /// <p>The response will provide a nextToken if there is more data beyond the maxResults.</p> 
    /// <p>If there is no more data in the responce, the nextToken will not be returned.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl ListClustersOutput {
    /// <p>A list of Elastic DocumentDB cluster.</p>
    pub fn clusters(&self) -> std::option::Option<& [crate::model::ClusterInList]> {
        self.clusters.as_deref()
    }
    /// <p>The response will provide a nextToken if there is more data beyond the maxResults.</p> 
    /// <p>If there is no more data in the responce, the nextToken will not be returned.</p>
    pub fn next_token(&self) -> std::option::Option<& str> {
        self.next_token.as_deref()
    }
}
/// See [`ListClustersOutput`](crate::output::ListClustersOutput).
pub mod list_clusters_output {
    
    /// A builder for [`ListClustersOutput`](crate::output::ListClustersOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) clusters: std::option::Option<std::vec::Vec<crate::model::ClusterInList>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `clusters`.
        ///
        /// To override the contents of this collection use [`set_clusters`](Self::set_clusters).
        ///
        /// <p>A list of Elastic DocumentDB cluster.</p>
        pub fn clusters(mut self, input: crate::model::ClusterInList) -> Self {
            let mut v = self.clusters.unwrap_or_default();
                            v.push(input);
                            self.clusters = Some(v);
                            self
        }
        /// <p>A list of Elastic DocumentDB cluster.</p>
        pub fn set_clusters(mut self, input: std::option::Option<std::vec::Vec<crate::model::ClusterInList>>) -> Self {
            self.clusters = input; self
        }
        /// <p>The response will provide a nextToken if there is more data beyond the maxResults.</p> 
        /// <p>If there is no more data in the responce, the nextToken will not be returned.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The response will provide a nextToken if there is more data beyond the maxResults.</p> 
        /// <p>If there is no more data in the responce, the nextToken will not be returned.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input; self
        }
        /// Consumes the builder and constructs a [`ListClustersOutput`](crate::output::ListClustersOutput).
        pub fn build(self) -> crate::output::ListClustersOutput {
            crate::output::ListClustersOutput {
                clusters: self.clusters
                ,
                next_token: self.next_token
                ,
            }
        }
    }
    
    
}
impl ListClustersOutput {
    /// Creates a new builder-style object to manufacture [`ListClustersOutput`](crate::output::ListClustersOutput).
    pub fn builder() -> crate::output::list_clusters_output::Builder {
        crate::output::list_clusters_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetClusterSnapshotOutput  {
    /// <p>Returns information about a specific Elastic DocumentDB snapshot.</p>
    #[doc(hidden)]
    pub snapshot: std::option::Option<crate::model::ClusterSnapshot>,
}
impl GetClusterSnapshotOutput {
    /// <p>Returns information about a specific Elastic DocumentDB snapshot.</p>
    pub fn snapshot(&self) -> std::option::Option<& crate::model::ClusterSnapshot> {
        self.snapshot.as_ref()
    }
}
/// See [`GetClusterSnapshotOutput`](crate::output::GetClusterSnapshotOutput).
pub mod get_cluster_snapshot_output {
    
    /// A builder for [`GetClusterSnapshotOutput`](crate::output::GetClusterSnapshotOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) snapshot: std::option::Option<crate::model::ClusterSnapshot>,
    }
    impl Builder {
        /// <p>Returns information about a specific Elastic DocumentDB snapshot.</p>
        pub fn snapshot(mut self, input: crate::model::ClusterSnapshot) -> Self {
            self.snapshot = Some(input);
            self
        }
        /// <p>Returns information about a specific Elastic DocumentDB snapshot.</p>
        pub fn set_snapshot(mut self, input: std::option::Option<crate::model::ClusterSnapshot>) -> Self {
            self.snapshot = input; self
        }
        /// Consumes the builder and constructs a [`GetClusterSnapshotOutput`](crate::output::GetClusterSnapshotOutput).
        pub fn build(self) -> crate::output::GetClusterSnapshotOutput {
            crate::output::GetClusterSnapshotOutput {
                snapshot: self.snapshot
                ,
            }
        }
    }
    
    
}
impl GetClusterSnapshotOutput {
    /// Creates a new builder-style object to manufacture [`GetClusterSnapshotOutput`](crate::output::GetClusterSnapshotOutput).
    pub fn builder() -> crate::output::get_cluster_snapshot_output::Builder {
        crate::output::get_cluster_snapshot_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetClusterOutput  {
    /// <p>Returns information about a specific Elastic DocumentDB cluster.</p>
    #[doc(hidden)]
    pub cluster: std::option::Option<crate::model::Cluster>,
}
impl GetClusterOutput {
    /// <p>Returns information about a specific Elastic DocumentDB cluster.</p>
    pub fn cluster(&self) -> std::option::Option<& crate::model::Cluster> {
        self.cluster.as_ref()
    }
}
/// See [`GetClusterOutput`](crate::output::GetClusterOutput).
pub mod get_cluster_output {
    
    /// A builder for [`GetClusterOutput`](crate::output::GetClusterOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) cluster: std::option::Option<crate::model::Cluster>,
    }
    impl Builder {
        /// <p>Returns information about a specific Elastic DocumentDB cluster.</p>
        pub fn cluster(mut self, input: crate::model::Cluster) -> Self {
            self.cluster = Some(input);
            self
        }
        /// <p>Returns information about a specific Elastic DocumentDB cluster.</p>
        pub fn set_cluster(mut self, input: std::option::Option<crate::model::Cluster>) -> Self {
            self.cluster = input; self
        }
        /// Consumes the builder and constructs a [`GetClusterOutput`](crate::output::GetClusterOutput).
        pub fn build(self) -> crate::output::GetClusterOutput {
            crate::output::GetClusterOutput {
                cluster: self.cluster
                ,
            }
        }
    }
    
    
}
impl GetClusterOutput {
    /// Creates a new builder-style object to manufacture [`GetClusterOutput`](crate::output::GetClusterOutput).
    pub fn builder() -> crate::output::get_cluster_output::Builder {
        crate::output::get_cluster_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteClusterSnapshotOutput  {
    /// <p>Returns information about the newly deleted Elastic DocumentDB snapshot.</p>
    #[doc(hidden)]
    pub snapshot: std::option::Option<crate::model::ClusterSnapshot>,
}
impl DeleteClusterSnapshotOutput {
    /// <p>Returns information about the newly deleted Elastic DocumentDB snapshot.</p>
    pub fn snapshot(&self) -> std::option::Option<& crate::model::ClusterSnapshot> {
        self.snapshot.as_ref()
    }
}
/// See [`DeleteClusterSnapshotOutput`](crate::output::DeleteClusterSnapshotOutput).
pub mod delete_cluster_snapshot_output {
    
    /// A builder for [`DeleteClusterSnapshotOutput`](crate::output::DeleteClusterSnapshotOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) snapshot: std::option::Option<crate::model::ClusterSnapshot>,
    }
    impl Builder {
        /// <p>Returns information about the newly deleted Elastic DocumentDB snapshot.</p>
        pub fn snapshot(mut self, input: crate::model::ClusterSnapshot) -> Self {
            self.snapshot = Some(input);
            self
        }
        /// <p>Returns information about the newly deleted Elastic DocumentDB snapshot.</p>
        pub fn set_snapshot(mut self, input: std::option::Option<crate::model::ClusterSnapshot>) -> Self {
            self.snapshot = input; self
        }
        /// Consumes the builder and constructs a [`DeleteClusterSnapshotOutput`](crate::output::DeleteClusterSnapshotOutput).
        pub fn build(self) -> crate::output::DeleteClusterSnapshotOutput {
            crate::output::DeleteClusterSnapshotOutput {
                snapshot: self.snapshot
                ,
            }
        }
    }
    
    
}
impl DeleteClusterSnapshotOutput {
    /// Creates a new builder-style object to manufacture [`DeleteClusterSnapshotOutput`](crate::output::DeleteClusterSnapshotOutput).
    pub fn builder() -> crate::output::delete_cluster_snapshot_output::Builder {
        crate::output::delete_cluster_snapshot_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteClusterOutput  {
    /// <p>Returns information about the newly deleted Elastic DocumentDB cluster.</p>
    #[doc(hidden)]
    pub cluster: std::option::Option<crate::model::Cluster>,
}
impl DeleteClusterOutput {
    /// <p>Returns information about the newly deleted Elastic DocumentDB cluster.</p>
    pub fn cluster(&self) -> std::option::Option<& crate::model::Cluster> {
        self.cluster.as_ref()
    }
}
/// See [`DeleteClusterOutput`](crate::output::DeleteClusterOutput).
pub mod delete_cluster_output {
    
    /// A builder for [`DeleteClusterOutput`](crate::output::DeleteClusterOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) cluster: std::option::Option<crate::model::Cluster>,
    }
    impl Builder {
        /// <p>Returns information about the newly deleted Elastic DocumentDB cluster.</p>
        pub fn cluster(mut self, input: crate::model::Cluster) -> Self {
            self.cluster = Some(input);
            self
        }
        /// <p>Returns information about the newly deleted Elastic DocumentDB cluster.</p>
        pub fn set_cluster(mut self, input: std::option::Option<crate::model::Cluster>) -> Self {
            self.cluster = input; self
        }
        /// Consumes the builder and constructs a [`DeleteClusterOutput`](crate::output::DeleteClusterOutput).
        pub fn build(self) -> crate::output::DeleteClusterOutput {
            crate::output::DeleteClusterOutput {
                cluster: self.cluster
                ,
            }
        }
    }
    
    
}
impl DeleteClusterOutput {
    /// Creates a new builder-style object to manufacture [`DeleteClusterOutput`](crate::output::DeleteClusterOutput).
    pub fn builder() -> crate::output::delete_cluster_output::Builder {
        crate::output::delete_cluster_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateClusterSnapshotOutput  {
    /// <p>Returns information about the new Elastic DocumentDB snapshot.</p>
    #[doc(hidden)]
    pub snapshot: std::option::Option<crate::model::ClusterSnapshot>,
}
impl CreateClusterSnapshotOutput {
    /// <p>Returns information about the new Elastic DocumentDB snapshot.</p>
    pub fn snapshot(&self) -> std::option::Option<& crate::model::ClusterSnapshot> {
        self.snapshot.as_ref()
    }
}
/// See [`CreateClusterSnapshotOutput`](crate::output::CreateClusterSnapshotOutput).
pub mod create_cluster_snapshot_output {
    
    /// A builder for [`CreateClusterSnapshotOutput`](crate::output::CreateClusterSnapshotOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) snapshot: std::option::Option<crate::model::ClusterSnapshot>,
    }
    impl Builder {
        /// <p>Returns information about the new Elastic DocumentDB snapshot.</p>
        pub fn snapshot(mut self, input: crate::model::ClusterSnapshot) -> Self {
            self.snapshot = Some(input);
            self
        }
        /// <p>Returns information about the new Elastic DocumentDB snapshot.</p>
        pub fn set_snapshot(mut self, input: std::option::Option<crate::model::ClusterSnapshot>) -> Self {
            self.snapshot = input; self
        }
        /// Consumes the builder and constructs a [`CreateClusterSnapshotOutput`](crate::output::CreateClusterSnapshotOutput).
        pub fn build(self) -> crate::output::CreateClusterSnapshotOutput {
            crate::output::CreateClusterSnapshotOutput {
                snapshot: self.snapshot
                ,
            }
        }
    }
    
    
}
impl CreateClusterSnapshotOutput {
    /// Creates a new builder-style object to manufacture [`CreateClusterSnapshotOutput`](crate::output::CreateClusterSnapshotOutput).
    pub fn builder() -> crate::output::create_cluster_snapshot_output::Builder {
        crate::output::create_cluster_snapshot_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateClusterOutput  {
    /// <p>The new Elastic DocumentDB cluster that has been created.</p>
    #[doc(hidden)]
    pub cluster: std::option::Option<crate::model::Cluster>,
}
impl CreateClusterOutput {
    /// <p>The new Elastic DocumentDB cluster that has been created.</p>
    pub fn cluster(&self) -> std::option::Option<& crate::model::Cluster> {
        self.cluster.as_ref()
    }
}
/// See [`CreateClusterOutput`](crate::output::CreateClusterOutput).
pub mod create_cluster_output {
    
    /// A builder for [`CreateClusterOutput`](crate::output::CreateClusterOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) cluster: std::option::Option<crate::model::Cluster>,
    }
    impl Builder {
        /// <p>The new Elastic DocumentDB cluster that has been created.</p>
        pub fn cluster(mut self, input: crate::model::Cluster) -> Self {
            self.cluster = Some(input);
            self
        }
        /// <p>The new Elastic DocumentDB cluster that has been created.</p>
        pub fn set_cluster(mut self, input: std::option::Option<crate::model::Cluster>) -> Self {
            self.cluster = input; self
        }
        /// Consumes the builder and constructs a [`CreateClusterOutput`](crate::output::CreateClusterOutput).
        pub fn build(self) -> crate::output::CreateClusterOutput {
            crate::output::CreateClusterOutput {
                cluster: self.cluster
                ,
            }
        }
    }
    
    
}
impl CreateClusterOutput {
    /// Creates a new builder-style object to manufacture [`CreateClusterOutput`](crate::output::CreateClusterOutput).
    pub fn builder() -> crate::output::create_cluster_output::Builder {
        crate::output::create_cluster_output::Builder::default()
    }
}

