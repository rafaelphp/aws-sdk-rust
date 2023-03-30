// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UpdateLifecyclePolicyOutput  {
}
/// See [`UpdateLifecyclePolicyOutput`](crate::output::UpdateLifecyclePolicyOutput).
pub mod update_lifecycle_policy_output {
    
    /// A builder for [`UpdateLifecyclePolicyOutput`](crate::output::UpdateLifecyclePolicyOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`UpdateLifecyclePolicyOutput`](crate::output::UpdateLifecyclePolicyOutput).
        pub fn build(self) -> crate::output::UpdateLifecyclePolicyOutput {
            crate::output::UpdateLifecyclePolicyOutput {
            }
        }
    }
    
    
}
impl UpdateLifecyclePolicyOutput {
    /// Creates a new builder-style object to manufacture [`UpdateLifecyclePolicyOutput`](crate::output::UpdateLifecyclePolicyOutput).
    pub fn builder() -> crate::output::update_lifecycle_policy_output::Builder {
        crate::output::update_lifecycle_policy_output::Builder::default()
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
pub struct ListTagsForResourceOutput  {
    /// <p>Information about the tags.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl ListTagsForResourceOutput {
    /// <p>Information about the tags.</p>
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
        /// <p>Information about the tags.</p>
        pub fn tags(mut self, k: impl Into<std::string::String>, v: impl Into<std::string::String>) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
                            hash_map.insert(k.into(), v.into());
                            self.tags = Some(hash_map);
                            self
        }
        /// <p>Information about the tags.</p>
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
pub struct GetLifecyclePolicyOutput  {
    /// <p>Detailed information about the lifecycle policy.</p>
    #[doc(hidden)]
    pub policy: std::option::Option<crate::model::LifecyclePolicy>,
}
impl GetLifecyclePolicyOutput {
    /// <p>Detailed information about the lifecycle policy.</p>
    pub fn policy(&self) -> std::option::Option<& crate::model::LifecyclePolicy> {
        self.policy.as_ref()
    }
}
/// See [`GetLifecyclePolicyOutput`](crate::output::GetLifecyclePolicyOutput).
pub mod get_lifecycle_policy_output {
    
    /// A builder for [`GetLifecyclePolicyOutput`](crate::output::GetLifecyclePolicyOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) policy: std::option::Option<crate::model::LifecyclePolicy>,
    }
    impl Builder {
        /// <p>Detailed information about the lifecycle policy.</p>
        pub fn policy(mut self, input: crate::model::LifecyclePolicy) -> Self {
            self.policy = Some(input);
            self
        }
        /// <p>Detailed information about the lifecycle policy.</p>
        pub fn set_policy(mut self, input: std::option::Option<crate::model::LifecyclePolicy>) -> Self {
            self.policy = input; self
        }
        /// Consumes the builder and constructs a [`GetLifecyclePolicyOutput`](crate::output::GetLifecyclePolicyOutput).
        pub fn build(self) -> crate::output::GetLifecyclePolicyOutput {
            crate::output::GetLifecyclePolicyOutput {
                policy: self.policy
                ,
            }
        }
    }
    
    
}
impl GetLifecyclePolicyOutput {
    /// Creates a new builder-style object to manufacture [`GetLifecyclePolicyOutput`](crate::output::GetLifecyclePolicyOutput).
    pub fn builder() -> crate::output::get_lifecycle_policy_output::Builder {
        crate::output::get_lifecycle_policy_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetLifecyclePoliciesOutput  {
    /// <p>Summary information about the lifecycle policies.</p>
    #[doc(hidden)]
    pub policies: std::option::Option<std::vec::Vec<crate::model::LifecyclePolicySummary>>,
}
impl GetLifecyclePoliciesOutput {
    /// <p>Summary information about the lifecycle policies.</p>
    pub fn policies(&self) -> std::option::Option<& [crate::model::LifecyclePolicySummary]> {
        self.policies.as_deref()
    }
}
/// See [`GetLifecyclePoliciesOutput`](crate::output::GetLifecyclePoliciesOutput).
pub mod get_lifecycle_policies_output {
    
    /// A builder for [`GetLifecyclePoliciesOutput`](crate::output::GetLifecyclePoliciesOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) policies: std::option::Option<std::vec::Vec<crate::model::LifecyclePolicySummary>>,
    }
    impl Builder {
        /// Appends an item to `policies`.
        ///
        /// To override the contents of this collection use [`set_policies`](Self::set_policies).
        ///
        /// <p>Summary information about the lifecycle policies.</p>
        pub fn policies(mut self, input: crate::model::LifecyclePolicySummary) -> Self {
            let mut v = self.policies.unwrap_or_default();
                            v.push(input);
                            self.policies = Some(v);
                            self
        }
        /// <p>Summary information about the lifecycle policies.</p>
        pub fn set_policies(mut self, input: std::option::Option<std::vec::Vec<crate::model::LifecyclePolicySummary>>) -> Self {
            self.policies = input; self
        }
        /// Consumes the builder and constructs a [`GetLifecyclePoliciesOutput`](crate::output::GetLifecyclePoliciesOutput).
        pub fn build(self) -> crate::output::GetLifecyclePoliciesOutput {
            crate::output::GetLifecyclePoliciesOutput {
                policies: self.policies
                ,
            }
        }
    }
    
    
}
impl GetLifecyclePoliciesOutput {
    /// Creates a new builder-style object to manufacture [`GetLifecyclePoliciesOutput`](crate::output::GetLifecyclePoliciesOutput).
    pub fn builder() -> crate::output::get_lifecycle_policies_output::Builder {
        crate::output::get_lifecycle_policies_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteLifecyclePolicyOutput  {
}
/// See [`DeleteLifecyclePolicyOutput`](crate::output::DeleteLifecyclePolicyOutput).
pub mod delete_lifecycle_policy_output {
    
    /// A builder for [`DeleteLifecyclePolicyOutput`](crate::output::DeleteLifecyclePolicyOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteLifecyclePolicyOutput`](crate::output::DeleteLifecyclePolicyOutput).
        pub fn build(self) -> crate::output::DeleteLifecyclePolicyOutput {
            crate::output::DeleteLifecyclePolicyOutput {
            }
        }
    }
    
    
}
impl DeleteLifecyclePolicyOutput {
    /// Creates a new builder-style object to manufacture [`DeleteLifecyclePolicyOutput`](crate::output::DeleteLifecyclePolicyOutput).
    pub fn builder() -> crate::output::delete_lifecycle_policy_output::Builder {
        crate::output::delete_lifecycle_policy_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateLifecyclePolicyOutput  {
    /// <p>The identifier of the lifecycle policy.</p>
    #[doc(hidden)]
    pub policy_id: std::option::Option<std::string::String>,
}
impl CreateLifecyclePolicyOutput {
    /// <p>The identifier of the lifecycle policy.</p>
    pub fn policy_id(&self) -> std::option::Option<& str> {
        self.policy_id.as_deref()
    }
}
/// See [`CreateLifecyclePolicyOutput`](crate::output::CreateLifecyclePolicyOutput).
pub mod create_lifecycle_policy_output {
    
    /// A builder for [`CreateLifecyclePolicyOutput`](crate::output::CreateLifecyclePolicyOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) policy_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The identifier of the lifecycle policy.</p>
        pub fn policy_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.policy_id = Some(input.into());
            self
        }
        /// <p>The identifier of the lifecycle policy.</p>
        pub fn set_policy_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.policy_id = input; self
        }
        /// Consumes the builder and constructs a [`CreateLifecyclePolicyOutput`](crate::output::CreateLifecyclePolicyOutput).
        pub fn build(self) -> crate::output::CreateLifecyclePolicyOutput {
            crate::output::CreateLifecyclePolicyOutput {
                policy_id: self.policy_id
                ,
            }
        }
    }
    
    
}
impl CreateLifecyclePolicyOutput {
    /// Creates a new builder-style object to manufacture [`CreateLifecyclePolicyOutput`](crate::output::CreateLifecyclePolicyOutput).
    pub fn builder() -> crate::output::create_lifecycle_policy_output::Builder {
        crate::output::create_lifecycle_policy_output::Builder::default()
    }
}

