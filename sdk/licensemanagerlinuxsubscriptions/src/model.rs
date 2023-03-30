// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `Status`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
/// 
/// Here is an example of how you can make a match expression forward-compatible:
/// 
/// ```text
/// # let status = unimplemented!();
/// match status {
///     Status::Completed => { /* ... */ },
///     Status::Failed => { /* ... */ },
///     Status::InProgress => { /* ... */ },
///     Status::Successful => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `status` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `Status::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `Status::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `Status::NewFeature` is defined.
/// Specifically, when `status` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `Status::NewFeature` also yielding `"NewFeature"`.
/// 
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::Eq, std::cmp::Ord, std::cmp::PartialEq, std::cmp::PartialOrd, std::fmt::Debug, std::hash::Hash)]
pub enum Status {
    /// Completed status
    Completed,
    /// Failed status
    Failed,
    /// InProgress status
    InProgress,
    /// Successful status
    Successful,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue)
}
impl std::convert::From<&str> for Status {
    fn from(s: &str) -> Self {
        match s {
            "Completed" => Status::Completed,
            "Failed" => Status::Failed,
            "InProgress" => Status::InProgress,
            "Successful" => Status::Successful,
            other => Status::Unknown(crate::types::UnknownVariantValue(other.to_owned()))
        }
    }
}
impl std::str::FromStr for Status {
                type Err = std::convert::Infallible;

                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    Ok(Status::from(s))
                }
            }
impl Status {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            Status::Completed => "Completed",
            Status::Failed => "Failed",
            Status::InProgress => "InProgress",
            Status::Successful => "Successful",
            Status::Unknown(value) => value.as_str()
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "Completed", "Failed", "InProgress", "Successful"
        ]
    }
}
impl AsRef<str> for Status {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Lists the settings defined for discovering Linux subscriptions.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct LinuxSubscriptionsDiscoverySettings  {
    /// <p>The Regions in which to discover data for Linux subscriptions.</p>
    #[doc(hidden)]
    pub source_regions: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>Details if you have enabled resource discovery across your accounts in Organizations.</p>
    #[doc(hidden)]
    pub organization_integration: std::option::Option<crate::model::OrganizationIntegration>,
}
impl LinuxSubscriptionsDiscoverySettings {
    /// <p>The Regions in which to discover data for Linux subscriptions.</p>
    pub fn source_regions(&self) -> std::option::Option<& [std::string::String]> {
        self.source_regions.as_deref()
    }
    /// <p>Details if you have enabled resource discovery across your accounts in Organizations.</p>
    pub fn organization_integration(&self) -> std::option::Option<& crate::model::OrganizationIntegration> {
        self.organization_integration.as_ref()
    }
}
/// See [`LinuxSubscriptionsDiscoverySettings`](crate::model::LinuxSubscriptionsDiscoverySettings).
pub mod linux_subscriptions_discovery_settings {
    
    /// A builder for [`LinuxSubscriptionsDiscoverySettings`](crate::model::LinuxSubscriptionsDiscoverySettings).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) source_regions: std::option::Option<std::vec::Vec<std::string::String>>,
        pub(crate) organization_integration: std::option::Option<crate::model::OrganizationIntegration>,
    }
    impl Builder {
        /// Appends an item to `source_regions`.
        ///
        /// To override the contents of this collection use [`set_source_regions`](Self::set_source_regions).
        ///
        /// <p>The Regions in which to discover data for Linux subscriptions.</p>
        pub fn source_regions(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.source_regions.unwrap_or_default();
                            v.push(input.into());
                            self.source_regions = Some(v);
                            self
        }
        /// <p>The Regions in which to discover data for Linux subscriptions.</p>
        pub fn set_source_regions(mut self, input: std::option::Option<std::vec::Vec<std::string::String>>) -> Self {
            self.source_regions = input; self
        }
        /// <p>Details if you have enabled resource discovery across your accounts in Organizations.</p>
        pub fn organization_integration(mut self, input: crate::model::OrganizationIntegration) -> Self {
            self.organization_integration = Some(input);
            self
        }
        /// <p>Details if you have enabled resource discovery across your accounts in Organizations.</p>
        pub fn set_organization_integration(mut self, input: std::option::Option<crate::model::OrganizationIntegration>) -> Self {
            self.organization_integration = input; self
        }
        /// Consumes the builder and constructs a [`LinuxSubscriptionsDiscoverySettings`](crate::model::LinuxSubscriptionsDiscoverySettings).
        pub fn build(self) -> crate::model::LinuxSubscriptionsDiscoverySettings {
            crate::model::LinuxSubscriptionsDiscoverySettings {
                source_regions: self.source_regions
                ,
                organization_integration: self.organization_integration
                ,
            }
        }
    }
    
    
}
impl LinuxSubscriptionsDiscoverySettings {
    /// Creates a new builder-style object to manufacture [`LinuxSubscriptionsDiscoverySettings`](crate::model::LinuxSubscriptionsDiscoverySettings).
    pub fn builder() -> crate::model::linux_subscriptions_discovery_settings::Builder {
        crate::model::linux_subscriptions_discovery_settings::Builder::default()
    }
}

/// When writing a match expression against `OrganizationIntegration`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
/// 
/// Here is an example of how you can make a match expression forward-compatible:
/// 
/// ```text
/// # let organizationintegration = unimplemented!();
/// match organizationintegration {
///     OrganizationIntegration::Disabled => { /* ... */ },
///     OrganizationIntegration::Enabled => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `organizationintegration` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `OrganizationIntegration::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `OrganizationIntegration::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `OrganizationIntegration::NewFeature` is defined.
/// Specifically, when `organizationintegration` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `OrganizationIntegration::NewFeature` also yielding `"NewFeature"`.
/// 
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::Eq, std::cmp::Ord, std::cmp::PartialEq, std::cmp::PartialOrd, std::fmt::Debug, std::hash::Hash)]
pub enum OrganizationIntegration {
    /// Disabled OrganizationIntegration
    Disabled,
    /// Enabled OrganizationIntegration
    Enabled,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue)
}
impl std::convert::From<&str> for OrganizationIntegration {
    fn from(s: &str) -> Self {
        match s {
            "Disabled" => OrganizationIntegration::Disabled,
            "Enabled" => OrganizationIntegration::Enabled,
            other => OrganizationIntegration::Unknown(crate::types::UnknownVariantValue(other.to_owned()))
        }
    }
}
impl std::str::FromStr for OrganizationIntegration {
                type Err = std::convert::Infallible;

                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    Ok(OrganizationIntegration::from(s))
                }
            }
impl OrganizationIntegration {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            OrganizationIntegration::Disabled => "Disabled",
            OrganizationIntegration::Enabled => "Enabled",
            OrganizationIntegration::Unknown(value) => value.as_str()
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "Disabled", "Enabled"
        ]
    }
}
impl AsRef<str> for OrganizationIntegration {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// When writing a match expression against `LinuxSubscriptionsDiscovery`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
/// 
/// Here is an example of how you can make a match expression forward-compatible:
/// 
/// ```text
/// # let linuxsubscriptionsdiscovery = unimplemented!();
/// match linuxsubscriptionsdiscovery {
///     LinuxSubscriptionsDiscovery::Disabled => { /* ... */ },
///     LinuxSubscriptionsDiscovery::Enabled => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `linuxsubscriptionsdiscovery` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `LinuxSubscriptionsDiscovery::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `LinuxSubscriptionsDiscovery::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `LinuxSubscriptionsDiscovery::NewFeature` is defined.
/// Specifically, when `linuxsubscriptionsdiscovery` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `LinuxSubscriptionsDiscovery::NewFeature` also yielding `"NewFeature"`.
/// 
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::Eq, std::cmp::Ord, std::cmp::PartialEq, std::cmp::PartialOrd, std::fmt::Debug, std::hash::Hash)]
pub enum LinuxSubscriptionsDiscovery {
    /// Disabled LinuxSubscriptionsDiscovery
    Disabled,
    /// Enabled LinuxSubscriptionsDiscovery
    Enabled,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue)
}
impl std::convert::From<&str> for LinuxSubscriptionsDiscovery {
    fn from(s: &str) -> Self {
        match s {
            "Disabled" => LinuxSubscriptionsDiscovery::Disabled,
            "Enabled" => LinuxSubscriptionsDiscovery::Enabled,
            other => LinuxSubscriptionsDiscovery::Unknown(crate::types::UnknownVariantValue(other.to_owned()))
        }
    }
}
impl std::str::FromStr for LinuxSubscriptionsDiscovery {
                type Err = std::convert::Infallible;

                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    Ok(LinuxSubscriptionsDiscovery::from(s))
                }
            }
impl LinuxSubscriptionsDiscovery {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            LinuxSubscriptionsDiscovery::Disabled => "Disabled",
            LinuxSubscriptionsDiscovery::Enabled => "Enabled",
            LinuxSubscriptionsDiscovery::Unknown(value) => value.as_str()
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "Disabled", "Enabled"
        ]
    }
}
impl AsRef<str> for LinuxSubscriptionsDiscovery {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>An object which details a discovered Linux subscription.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct Subscription  {
    /// <p>The name of the subscription.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// <p>The type of subscription. The type can be subscription-included with Amazon EC2, Bring Your Own Subscription model (BYOS), or from the Amazon Web Services Marketplace. Certain subscriptions may use licensing from the Amazon Web Services Marketplace as well as OS licensing from Amazon EC2 or BYOS.</p>
    #[doc(hidden)]
    pub r#type: std::option::Option<std::string::String>,
    /// <p>The total amount of running instances using this subscription.</p>
    #[doc(hidden)]
    pub instance_count: std::option::Option<i64>,
}
impl Subscription {
    /// <p>The name of the subscription.</p>
    pub fn name(&self) -> std::option::Option<& str> {
        self.name.as_deref()
    }
    /// <p>The type of subscription. The type can be subscription-included with Amazon EC2, Bring Your Own Subscription model (BYOS), or from the Amazon Web Services Marketplace. Certain subscriptions may use licensing from the Amazon Web Services Marketplace as well as OS licensing from Amazon EC2 or BYOS.</p>
    pub fn r#type(&self) -> std::option::Option<& str> {
        self.r#type.as_deref()
    }
    /// <p>The total amount of running instances using this subscription.</p>
    pub fn instance_count(&self) -> std::option::Option<i64> {
        self.instance_count
    }
}
/// See [`Subscription`](crate::model::Subscription).
pub mod subscription {
    
    /// A builder for [`Subscription`](crate::model::Subscription).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) r#type: std::option::Option<std::string::String>,
        pub(crate) instance_count: std::option::Option<i64>,
    }
    impl Builder {
        /// <p>The name of the subscription.</p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        /// <p>The name of the subscription.</p>
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input; self
        }
        /// <p>The type of subscription. The type can be subscription-included with Amazon EC2, Bring Your Own Subscription model (BYOS), or from the Amazon Web Services Marketplace. Certain subscriptions may use licensing from the Amazon Web Services Marketplace as well as OS licensing from Amazon EC2 or BYOS.</p>
        pub fn r#type(mut self, input: impl Into<std::string::String>) -> Self {
            self.r#type = Some(input.into());
            self
        }
        /// <p>The type of subscription. The type can be subscription-included with Amazon EC2, Bring Your Own Subscription model (BYOS), or from the Amazon Web Services Marketplace. Certain subscriptions may use licensing from the Amazon Web Services Marketplace as well as OS licensing from Amazon EC2 or BYOS.</p>
        pub fn set_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.r#type = input; self
        }
        /// <p>The total amount of running instances using this subscription.</p>
        pub fn instance_count(mut self, input: i64) -> Self {
            self.instance_count = Some(input);
            self
        }
        /// <p>The total amount of running instances using this subscription.</p>
        pub fn set_instance_count(mut self, input: std::option::Option<i64>) -> Self {
            self.instance_count = input; self
        }
        /// Consumes the builder and constructs a [`Subscription`](crate::model::Subscription).
        pub fn build(self) -> crate::model::Subscription {
            crate::model::Subscription {
                name: self.name
                ,
                r#type: self.r#type
                ,
                instance_count: self.instance_count
                ,
            }
        }
    }
    
    
}
impl Subscription {
    /// Creates a new builder-style object to manufacture [`Subscription`](crate::model::Subscription).
    pub fn builder() -> crate::model::subscription::Builder {
        crate::model::subscription::Builder::default()
    }
}

/// <p>A filter object that is used to return more specific results from a describe operation. Filters can be used to match a set of resources by specific criteria.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct Filter  {
    /// <p>The type of name to filter by.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// <p>One or more values for the name to filter by.</p>
    #[doc(hidden)]
    pub values: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>An operator for filtering results.</p>
    #[doc(hidden)]
    pub operator: std::option::Option<crate::model::Operator>,
}
impl Filter {
    /// <p>The type of name to filter by.</p>
    pub fn name(&self) -> std::option::Option<& str> {
        self.name.as_deref()
    }
    /// <p>One or more values for the name to filter by.</p>
    pub fn values(&self) -> std::option::Option<& [std::string::String]> {
        self.values.as_deref()
    }
    /// <p>An operator for filtering results.</p>
    pub fn operator(&self) -> std::option::Option<& crate::model::Operator> {
        self.operator.as_ref()
    }
}
/// See [`Filter`](crate::model::Filter).
pub mod filter {
    
    /// A builder for [`Filter`](crate::model::Filter).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) values: std::option::Option<std::vec::Vec<std::string::String>>,
        pub(crate) operator: std::option::Option<crate::model::Operator>,
    }
    impl Builder {
        /// <p>The type of name to filter by.</p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        /// <p>The type of name to filter by.</p>
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input; self
        }
        /// Appends an item to `values`.
        ///
        /// To override the contents of this collection use [`set_values`](Self::set_values).
        ///
        /// <p>One or more values for the name to filter by.</p>
        pub fn values(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.values.unwrap_or_default();
                            v.push(input.into());
                            self.values = Some(v);
                            self
        }
        /// <p>One or more values for the name to filter by.</p>
        pub fn set_values(mut self, input: std::option::Option<std::vec::Vec<std::string::String>>) -> Self {
            self.values = input; self
        }
        /// <p>An operator for filtering results.</p>
        pub fn operator(mut self, input: crate::model::Operator) -> Self {
            self.operator = Some(input);
            self
        }
        /// <p>An operator for filtering results.</p>
        pub fn set_operator(mut self, input: std::option::Option<crate::model::Operator>) -> Self {
            self.operator = input; self
        }
        /// Consumes the builder and constructs a [`Filter`](crate::model::Filter).
        pub fn build(self) -> crate::model::Filter {
            crate::model::Filter {
                name: self.name
                ,
                values: self.values
                ,
                operator: self.operator
                ,
            }
        }
    }
    
    
}
impl Filter {
    /// Creates a new builder-style object to manufacture [`Filter`](crate::model::Filter).
    pub fn builder() -> crate::model::filter::Builder {
        crate::model::filter::Builder::default()
    }
}

/// When writing a match expression against `Operator`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
/// 
/// Here is an example of how you can make a match expression forward-compatible:
/// 
/// ```text
/// # let operator = unimplemented!();
/// match operator {
///     Operator::Contains => { /* ... */ },
///     Operator::Equal => { /* ... */ },
///     Operator::NotEqual => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `operator` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `Operator::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `Operator::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `Operator::NewFeature` is defined.
/// Specifically, when `operator` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `Operator::NewFeature` also yielding `"NewFeature"`.
/// 
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::Eq, std::cmp::Ord, std::cmp::PartialEq, std::cmp::PartialOrd, std::fmt::Debug, std::hash::Hash)]
pub enum Operator {
    /// Contains operator
    Contains,
    /// Equal operator
    Equal,
    /// Not equal operator
    NotEqual,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue)
}
impl std::convert::From<&str> for Operator {
    fn from(s: &str) -> Self {
        match s {
            "Contains" => Operator::Contains,
            "Equal" => Operator::Equal,
            "NotEqual" => Operator::NotEqual,
            other => Operator::Unknown(crate::types::UnknownVariantValue(other.to_owned()))
        }
    }
}
impl std::str::FromStr for Operator {
                type Err = std::convert::Infallible;

                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    Ok(Operator::from(s))
                }
            }
impl Operator {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            Operator::Contains => "Contains",
            Operator::Equal => "Equal",
            Operator::NotEqual => "NotEqual",
            Operator::Unknown(value) => value.as_str()
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "Contains", "Equal", "NotEqual"
        ]
    }
}
impl AsRef<str> for Operator {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Details discovered information about a running instance using Linux subscriptions.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct Instance  {
    /// <p>The AMI ID used to launch the instance.</p>
    #[doc(hidden)]
    pub ami_id: std::option::Option<std::string::String>,
    /// <p>The instance ID of the resource.</p>
    #[doc(hidden)]
    pub instance_id: std::option::Option<std::string::String>,
    /// <p>The instance type of the resource.</p>
    #[doc(hidden)]
    pub instance_type: std::option::Option<std::string::String>,
    /// <p>The account ID which owns the instance.</p>
    #[doc(hidden)]
    pub account_id: std::option::Option<std::string::String>,
    /// <p>The status of the instance.</p>
    #[doc(hidden)]
    pub status: std::option::Option<std::string::String>,
    /// <p>The Region the instance is running in.</p>
    #[doc(hidden)]
    pub region: std::option::Option<std::string::String>,
    /// <p>The usage operation of the instance. For more information, see For more information, see <a href="https://docs.aws.amazon.com/license-manager/latest/userguide/linux-subscriptions-usage-operation.html">Usage operation values</a> in the <i>License Manager User Guide</i>.</p>
    #[doc(hidden)]
    pub usage_operation: std::option::Option<std::string::String>,
    /// <p>The product code for the instance. For more information, see <a href="https://docs.aws.amazon.com/license-manager/latest/userguide/linux-subscriptions-usage-operation.html">Usage operation values</a> in the <i>License Manager User Guide</i> .</p>
    #[doc(hidden)]
    pub product_code: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The time in which the last discovery updated the instance details.</p>
    #[doc(hidden)]
    pub last_updated_time: std::option::Option<std::string::String>,
    /// <p>The name of the subscription being used by the instance.</p>
    #[doc(hidden)]
    pub subscription_name: std::option::Option<std::string::String>,
}
impl Instance {
    /// <p>The AMI ID used to launch the instance.</p>
    pub fn ami_id(&self) -> std::option::Option<& str> {
        self.ami_id.as_deref()
    }
    /// <p>The instance ID of the resource.</p>
    pub fn instance_id(&self) -> std::option::Option<& str> {
        self.instance_id.as_deref()
    }
    /// <p>The instance type of the resource.</p>
    pub fn instance_type(&self) -> std::option::Option<& str> {
        self.instance_type.as_deref()
    }
    /// <p>The account ID which owns the instance.</p>
    pub fn account_id(&self) -> std::option::Option<& str> {
        self.account_id.as_deref()
    }
    /// <p>The status of the instance.</p>
    pub fn status(&self) -> std::option::Option<& str> {
        self.status.as_deref()
    }
    /// <p>The Region the instance is running in.</p>
    pub fn region(&self) -> std::option::Option<& str> {
        self.region.as_deref()
    }
    /// <p>The usage operation of the instance. For more information, see For more information, see <a href="https://docs.aws.amazon.com/license-manager/latest/userguide/linux-subscriptions-usage-operation.html">Usage operation values</a> in the <i>License Manager User Guide</i>.</p>
    pub fn usage_operation(&self) -> std::option::Option<& str> {
        self.usage_operation.as_deref()
    }
    /// <p>The product code for the instance. For more information, see <a href="https://docs.aws.amazon.com/license-manager/latest/userguide/linux-subscriptions-usage-operation.html">Usage operation values</a> in the <i>License Manager User Guide</i> .</p>
    pub fn product_code(&self) -> std::option::Option<& [std::string::String]> {
        self.product_code.as_deref()
    }
    /// <p>The time in which the last discovery updated the instance details.</p>
    pub fn last_updated_time(&self) -> std::option::Option<& str> {
        self.last_updated_time.as_deref()
    }
    /// <p>The name of the subscription being used by the instance.</p>
    pub fn subscription_name(&self) -> std::option::Option<& str> {
        self.subscription_name.as_deref()
    }
}
/// See [`Instance`](crate::model::Instance).
pub mod instance {
    
    /// A builder for [`Instance`](crate::model::Instance).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) ami_id: std::option::Option<std::string::String>,
        pub(crate) instance_id: std::option::Option<std::string::String>,
        pub(crate) instance_type: std::option::Option<std::string::String>,
        pub(crate) account_id: std::option::Option<std::string::String>,
        pub(crate) status: std::option::Option<std::string::String>,
        pub(crate) region: std::option::Option<std::string::String>,
        pub(crate) usage_operation: std::option::Option<std::string::String>,
        pub(crate) product_code: std::option::Option<std::vec::Vec<std::string::String>>,
        pub(crate) last_updated_time: std::option::Option<std::string::String>,
        pub(crate) subscription_name: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The AMI ID used to launch the instance.</p>
        pub fn ami_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.ami_id = Some(input.into());
            self
        }
        /// <p>The AMI ID used to launch the instance.</p>
        pub fn set_ami_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.ami_id = input; self
        }
        /// <p>The instance ID of the resource.</p>
        pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.instance_id = Some(input.into());
            self
        }
        /// <p>The instance ID of the resource.</p>
        pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.instance_id = input; self
        }
        /// <p>The instance type of the resource.</p>
        pub fn instance_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.instance_type = Some(input.into());
            self
        }
        /// <p>The instance type of the resource.</p>
        pub fn set_instance_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.instance_type = input; self
        }
        /// <p>The account ID which owns the instance.</p>
        pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.account_id = Some(input.into());
            self
        }
        /// <p>The account ID which owns the instance.</p>
        pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.account_id = input; self
        }
        /// <p>The status of the instance.</p>
        pub fn status(mut self, input: impl Into<std::string::String>) -> Self {
            self.status = Some(input.into());
            self
        }
        /// <p>The status of the instance.</p>
        pub fn set_status(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.status = input; self
        }
        /// <p>The Region the instance is running in.</p>
        pub fn region(mut self, input: impl Into<std::string::String>) -> Self {
            self.region = Some(input.into());
            self
        }
        /// <p>The Region the instance is running in.</p>
        pub fn set_region(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.region = input; self
        }
        /// <p>The usage operation of the instance. For more information, see For more information, see <a href="https://docs.aws.amazon.com/license-manager/latest/userguide/linux-subscriptions-usage-operation.html">Usage operation values</a> in the <i>License Manager User Guide</i>.</p>
        pub fn usage_operation(mut self, input: impl Into<std::string::String>) -> Self {
            self.usage_operation = Some(input.into());
            self
        }
        /// <p>The usage operation of the instance. For more information, see For more information, see <a href="https://docs.aws.amazon.com/license-manager/latest/userguide/linux-subscriptions-usage-operation.html">Usage operation values</a> in the <i>License Manager User Guide</i>.</p>
        pub fn set_usage_operation(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.usage_operation = input; self
        }
        /// Appends an item to `product_code`.
        ///
        /// To override the contents of this collection use [`set_product_code`](Self::set_product_code).
        ///
        /// <p>The product code for the instance. For more information, see <a href="https://docs.aws.amazon.com/license-manager/latest/userguide/linux-subscriptions-usage-operation.html">Usage operation values</a> in the <i>License Manager User Guide</i> .</p>
        pub fn product_code(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.product_code.unwrap_or_default();
                            v.push(input.into());
                            self.product_code = Some(v);
                            self
        }
        /// <p>The product code for the instance. For more information, see <a href="https://docs.aws.amazon.com/license-manager/latest/userguide/linux-subscriptions-usage-operation.html">Usage operation values</a> in the <i>License Manager User Guide</i> .</p>
        pub fn set_product_code(mut self, input: std::option::Option<std::vec::Vec<std::string::String>>) -> Self {
            self.product_code = input; self
        }
        /// <p>The time in which the last discovery updated the instance details.</p>
        pub fn last_updated_time(mut self, input: impl Into<std::string::String>) -> Self {
            self.last_updated_time = Some(input.into());
            self
        }
        /// <p>The time in which the last discovery updated the instance details.</p>
        pub fn set_last_updated_time(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.last_updated_time = input; self
        }
        /// <p>The name of the subscription being used by the instance.</p>
        pub fn subscription_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.subscription_name = Some(input.into());
            self
        }
        /// <p>The name of the subscription being used by the instance.</p>
        pub fn set_subscription_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.subscription_name = input; self
        }
        /// Consumes the builder and constructs a [`Instance`](crate::model::Instance).
        pub fn build(self) -> crate::model::Instance {
            crate::model::Instance {
                ami_id: self.ami_id
                ,
                instance_id: self.instance_id
                ,
                instance_type: self.instance_type
                ,
                account_id: self.account_id
                ,
                status: self.status
                ,
                region: self.region
                ,
                usage_operation: self.usage_operation
                ,
                product_code: self.product_code
                ,
                last_updated_time: self.last_updated_time
                ,
                subscription_name: self.subscription_name
                ,
            }
        }
    }
    
    
}
impl Instance {
    /// Creates a new builder-style object to manufacture [`Instance`](crate::model::Instance).
    pub fn builder() -> crate::model::instance::Builder {
        crate::model::instance::Builder::default()
    }
}

