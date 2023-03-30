// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
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
pub struct DescribeSavingsPlansOfferingsOutput  {
    /// <p>Information about the Savings Plans offerings.</p>
    #[doc(hidden)]
    pub search_results: std::option::Option<std::vec::Vec<crate::model::SavingsPlanOffering>>,
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl DescribeSavingsPlansOfferingsOutput {
    /// <p>Information about the Savings Plans offerings.</p>
    pub fn search_results(&self) -> std::option::Option<& [crate::model::SavingsPlanOffering]> {
        self.search_results.as_deref()
    }
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
    pub fn next_token(&self) -> std::option::Option<& str> {
        self.next_token.as_deref()
    }
}
/// See [`DescribeSavingsPlansOfferingsOutput`](crate::output::DescribeSavingsPlansOfferingsOutput).
pub mod describe_savings_plans_offerings_output {
    
    /// A builder for [`DescribeSavingsPlansOfferingsOutput`](crate::output::DescribeSavingsPlansOfferingsOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) search_results: std::option::Option<std::vec::Vec<crate::model::SavingsPlanOffering>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `search_results`.
        ///
        /// To override the contents of this collection use [`set_search_results`](Self::set_search_results).
        ///
        /// <p>Information about the Savings Plans offerings.</p>
        pub fn search_results(mut self, input: crate::model::SavingsPlanOffering) -> Self {
            let mut v = self.search_results.unwrap_or_default();
                            v.push(input);
                            self.search_results = Some(v);
                            self
        }
        /// <p>Information about the Savings Plans offerings.</p>
        pub fn set_search_results(mut self, input: std::option::Option<std::vec::Vec<crate::model::SavingsPlanOffering>>) -> Self {
            self.search_results = input; self
        }
        /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input; self
        }
        /// Consumes the builder and constructs a [`DescribeSavingsPlansOfferingsOutput`](crate::output::DescribeSavingsPlansOfferingsOutput).
        pub fn build(self) -> crate::output::DescribeSavingsPlansOfferingsOutput {
            crate::output::DescribeSavingsPlansOfferingsOutput {
                search_results: self.search_results
                ,
                next_token: self.next_token
                ,
            }
        }
    }
    
    
}
impl DescribeSavingsPlansOfferingsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeSavingsPlansOfferingsOutput`](crate::output::DescribeSavingsPlansOfferingsOutput).
    pub fn builder() -> crate::output::describe_savings_plans_offerings_output::Builder {
        crate::output::describe_savings_plans_offerings_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeSavingsPlansOfferingRatesOutput  {
    /// <p>Information about the Savings Plans offering rates.</p>
    #[doc(hidden)]
    pub search_results: std::option::Option<std::vec::Vec<crate::model::SavingsPlanOfferingRate>>,
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl DescribeSavingsPlansOfferingRatesOutput {
    /// <p>Information about the Savings Plans offering rates.</p>
    pub fn search_results(&self) -> std::option::Option<& [crate::model::SavingsPlanOfferingRate]> {
        self.search_results.as_deref()
    }
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
    pub fn next_token(&self) -> std::option::Option<& str> {
        self.next_token.as_deref()
    }
}
/// See [`DescribeSavingsPlansOfferingRatesOutput`](crate::output::DescribeSavingsPlansOfferingRatesOutput).
pub mod describe_savings_plans_offering_rates_output {
    
    /// A builder for [`DescribeSavingsPlansOfferingRatesOutput`](crate::output::DescribeSavingsPlansOfferingRatesOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) search_results: std::option::Option<std::vec::Vec<crate::model::SavingsPlanOfferingRate>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `search_results`.
        ///
        /// To override the contents of this collection use [`set_search_results`](Self::set_search_results).
        ///
        /// <p>Information about the Savings Plans offering rates.</p>
        pub fn search_results(mut self, input: crate::model::SavingsPlanOfferingRate) -> Self {
            let mut v = self.search_results.unwrap_or_default();
                            v.push(input);
                            self.search_results = Some(v);
                            self
        }
        /// <p>Information about the Savings Plans offering rates.</p>
        pub fn set_search_results(mut self, input: std::option::Option<std::vec::Vec<crate::model::SavingsPlanOfferingRate>>) -> Self {
            self.search_results = input; self
        }
        /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input; self
        }
        /// Consumes the builder and constructs a [`DescribeSavingsPlansOfferingRatesOutput`](crate::output::DescribeSavingsPlansOfferingRatesOutput).
        pub fn build(self) -> crate::output::DescribeSavingsPlansOfferingRatesOutput {
            crate::output::DescribeSavingsPlansOfferingRatesOutput {
                search_results: self.search_results
                ,
                next_token: self.next_token
                ,
            }
        }
    }
    
    
}
impl DescribeSavingsPlansOfferingRatesOutput {
    /// Creates a new builder-style object to manufacture [`DescribeSavingsPlansOfferingRatesOutput`](crate::output::DescribeSavingsPlansOfferingRatesOutput).
    pub fn builder() -> crate::output::describe_savings_plans_offering_rates_output::Builder {
        crate::output::describe_savings_plans_offering_rates_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeSavingsPlansOutput  {
    /// <p>Information about the Savings Plans.</p>
    #[doc(hidden)]
    pub savings_plans: std::option::Option<std::vec::Vec<crate::model::SavingsPlan>>,
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl DescribeSavingsPlansOutput {
    /// <p>Information about the Savings Plans.</p>
    pub fn savings_plans(&self) -> std::option::Option<& [crate::model::SavingsPlan]> {
        self.savings_plans.as_deref()
    }
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
    pub fn next_token(&self) -> std::option::Option<& str> {
        self.next_token.as_deref()
    }
}
/// See [`DescribeSavingsPlansOutput`](crate::output::DescribeSavingsPlansOutput).
pub mod describe_savings_plans_output {
    
    /// A builder for [`DescribeSavingsPlansOutput`](crate::output::DescribeSavingsPlansOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) savings_plans: std::option::Option<std::vec::Vec<crate::model::SavingsPlan>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `savings_plans`.
        ///
        /// To override the contents of this collection use [`set_savings_plans`](Self::set_savings_plans).
        ///
        /// <p>Information about the Savings Plans.</p>
        pub fn savings_plans(mut self, input: crate::model::SavingsPlan) -> Self {
            let mut v = self.savings_plans.unwrap_or_default();
                            v.push(input);
                            self.savings_plans = Some(v);
                            self
        }
        /// <p>Information about the Savings Plans.</p>
        pub fn set_savings_plans(mut self, input: std::option::Option<std::vec::Vec<crate::model::SavingsPlan>>) -> Self {
            self.savings_plans = input; self
        }
        /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input; self
        }
        /// Consumes the builder and constructs a [`DescribeSavingsPlansOutput`](crate::output::DescribeSavingsPlansOutput).
        pub fn build(self) -> crate::output::DescribeSavingsPlansOutput {
            crate::output::DescribeSavingsPlansOutput {
                savings_plans: self.savings_plans
                ,
                next_token: self.next_token
                ,
            }
        }
    }
    
    
}
impl DescribeSavingsPlansOutput {
    /// Creates a new builder-style object to manufacture [`DescribeSavingsPlansOutput`](crate::output::DescribeSavingsPlansOutput).
    pub fn builder() -> crate::output::describe_savings_plans_output::Builder {
        crate::output::describe_savings_plans_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeSavingsPlanRatesOutput  {
    /// <p>The ID of the Savings Plan.</p>
    #[doc(hidden)]
    pub savings_plan_id: std::option::Option<std::string::String>,
    /// <p>Information about the Savings Plans rates.</p>
    #[doc(hidden)]
    pub search_results: std::option::Option<std::vec::Vec<crate::model::SavingsPlanRate>>,
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl DescribeSavingsPlanRatesOutput {
    /// <p>The ID of the Savings Plan.</p>
    pub fn savings_plan_id(&self) -> std::option::Option<& str> {
        self.savings_plan_id.as_deref()
    }
    /// <p>Information about the Savings Plans rates.</p>
    pub fn search_results(&self) -> std::option::Option<& [crate::model::SavingsPlanRate]> {
        self.search_results.as_deref()
    }
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
    pub fn next_token(&self) -> std::option::Option<& str> {
        self.next_token.as_deref()
    }
}
/// See [`DescribeSavingsPlanRatesOutput`](crate::output::DescribeSavingsPlanRatesOutput).
pub mod describe_savings_plan_rates_output {
    
    /// A builder for [`DescribeSavingsPlanRatesOutput`](crate::output::DescribeSavingsPlanRatesOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) savings_plan_id: std::option::Option<std::string::String>,
        pub(crate) search_results: std::option::Option<std::vec::Vec<crate::model::SavingsPlanRate>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ID of the Savings Plan.</p>
        pub fn savings_plan_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.savings_plan_id = Some(input.into());
            self
        }
        /// <p>The ID of the Savings Plan.</p>
        pub fn set_savings_plan_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.savings_plan_id = input; self
        }
        /// Appends an item to `search_results`.
        ///
        /// To override the contents of this collection use [`set_search_results`](Self::set_search_results).
        ///
        /// <p>Information about the Savings Plans rates.</p>
        pub fn search_results(mut self, input: crate::model::SavingsPlanRate) -> Self {
            let mut v = self.search_results.unwrap_or_default();
                            v.push(input);
                            self.search_results = Some(v);
                            self
        }
        /// <p>Information about the Savings Plans rates.</p>
        pub fn set_search_results(mut self, input: std::option::Option<std::vec::Vec<crate::model::SavingsPlanRate>>) -> Self {
            self.search_results = input; self
        }
        /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input; self
        }
        /// Consumes the builder and constructs a [`DescribeSavingsPlanRatesOutput`](crate::output::DescribeSavingsPlanRatesOutput).
        pub fn build(self) -> crate::output::DescribeSavingsPlanRatesOutput {
            crate::output::DescribeSavingsPlanRatesOutput {
                savings_plan_id: self.savings_plan_id
                ,
                search_results: self.search_results
                ,
                next_token: self.next_token
                ,
            }
        }
    }
    
    
}
impl DescribeSavingsPlanRatesOutput {
    /// Creates a new builder-style object to manufacture [`DescribeSavingsPlanRatesOutput`](crate::output::DescribeSavingsPlanRatesOutput).
    pub fn builder() -> crate::output::describe_savings_plan_rates_output::Builder {
        crate::output::describe_savings_plan_rates_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteQueuedSavingsPlanOutput  {
}
/// See [`DeleteQueuedSavingsPlanOutput`](crate::output::DeleteQueuedSavingsPlanOutput).
pub mod delete_queued_savings_plan_output {
    
    /// A builder for [`DeleteQueuedSavingsPlanOutput`](crate::output::DeleteQueuedSavingsPlanOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteQueuedSavingsPlanOutput`](crate::output::DeleteQueuedSavingsPlanOutput).
        pub fn build(self) -> crate::output::DeleteQueuedSavingsPlanOutput {
            crate::output::DeleteQueuedSavingsPlanOutput {
            }
        }
    }
    
    
}
impl DeleteQueuedSavingsPlanOutput {
    /// Creates a new builder-style object to manufacture [`DeleteQueuedSavingsPlanOutput`](crate::output::DeleteQueuedSavingsPlanOutput).
    pub fn builder() -> crate::output::delete_queued_savings_plan_output::Builder {
        crate::output::delete_queued_savings_plan_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateSavingsPlanOutput  {
    /// <p>The ID of the Savings Plan.</p>
    #[doc(hidden)]
    pub savings_plan_id: std::option::Option<std::string::String>,
}
impl CreateSavingsPlanOutput {
    /// <p>The ID of the Savings Plan.</p>
    pub fn savings_plan_id(&self) -> std::option::Option<& str> {
        self.savings_plan_id.as_deref()
    }
}
/// See [`CreateSavingsPlanOutput`](crate::output::CreateSavingsPlanOutput).
pub mod create_savings_plan_output {
    
    /// A builder for [`CreateSavingsPlanOutput`](crate::output::CreateSavingsPlanOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) savings_plan_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ID of the Savings Plan.</p>
        pub fn savings_plan_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.savings_plan_id = Some(input.into());
            self
        }
        /// <p>The ID of the Savings Plan.</p>
        pub fn set_savings_plan_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.savings_plan_id = input; self
        }
        /// Consumes the builder and constructs a [`CreateSavingsPlanOutput`](crate::output::CreateSavingsPlanOutput).
        pub fn build(self) -> crate::output::CreateSavingsPlanOutput {
            crate::output::CreateSavingsPlanOutput {
                savings_plan_id: self.savings_plan_id
                ,
            }
        }
    }
    
    
}
impl CreateSavingsPlanOutput {
    /// Creates a new builder-style object to manufacture [`CreateSavingsPlanOutput`](crate::output::CreateSavingsPlanOutput).
    pub fn builder() -> crate::output::create_savings_plan_output::Builder {
        crate::output::create_savings_plan_output::Builder::default()
    }
}

