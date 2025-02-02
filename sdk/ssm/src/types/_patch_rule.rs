// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Defines an approval rule for a patch baseline.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PatchRule {
    /// <p>The patch filter group that defines the criteria for the rule.</p>
    pub patch_filter_group: ::std::option::Option<crate::types::PatchFilterGroup>,
    /// <p>A compliance severity level for all approved patches in a patch baseline.</p>
    pub compliance_level: ::std::option::Option<crate::types::PatchComplianceLevel>,
    /// <p>The number of days after the release date of each patch matched by the rule that the patch is marked as approved in the patch baseline. For example, a value of <code>7</code> means that patches are approved seven days after they are released. Not supported on Debian Server or Ubuntu Server.</p>
    pub approve_after_days: ::std::option::Option<i32>,
    /// <p>The cutoff date for auto approval of released patches. Any patches released on or before this date are installed automatically. Not supported on Debian Server or Ubuntu Server.</p>
    /// <p>Enter dates in the format <code>YYYY-MM-DD</code>. For example, <code>2021-12-31</code>.</p>
    pub approve_until_date: ::std::option::Option<::std::string::String>,
    /// <p>For managed nodes identified by the approval rule filters, enables a patch baseline to apply non-security updates available in the specified repository. The default value is <code>false</code>. Applies to Linux managed nodes only.</p>
    pub enable_non_security: ::std::option::Option<bool>,
}
impl PatchRule {
    /// <p>The patch filter group that defines the criteria for the rule.</p>
    pub fn patch_filter_group(&self) -> ::std::option::Option<&crate::types::PatchFilterGroup> {
        self.patch_filter_group.as_ref()
    }
    /// <p>A compliance severity level for all approved patches in a patch baseline.</p>
    pub fn compliance_level(&self) -> ::std::option::Option<&crate::types::PatchComplianceLevel> {
        self.compliance_level.as_ref()
    }
    /// <p>The number of days after the release date of each patch matched by the rule that the patch is marked as approved in the patch baseline. For example, a value of <code>7</code> means that patches are approved seven days after they are released. Not supported on Debian Server or Ubuntu Server.</p>
    pub fn approve_after_days(&self) -> ::std::option::Option<i32> {
        self.approve_after_days
    }
    /// <p>The cutoff date for auto approval of released patches. Any patches released on or before this date are installed automatically. Not supported on Debian Server or Ubuntu Server.</p>
    /// <p>Enter dates in the format <code>YYYY-MM-DD</code>. For example, <code>2021-12-31</code>.</p>
    pub fn approve_until_date(&self) -> ::std::option::Option<&str> {
        self.approve_until_date.as_deref()
    }
    /// <p>For managed nodes identified by the approval rule filters, enables a patch baseline to apply non-security updates available in the specified repository. The default value is <code>false</code>. Applies to Linux managed nodes only.</p>
    pub fn enable_non_security(&self) -> ::std::option::Option<bool> {
        self.enable_non_security
    }
}
impl PatchRule {
    /// Creates a new builder-style object to manufacture [`PatchRule`](crate::types::PatchRule).
    pub fn builder() -> crate::types::builders::PatchRuleBuilder {
        crate::types::builders::PatchRuleBuilder::default()
    }
}

/// A builder for [`PatchRule`](crate::types::PatchRule).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PatchRuleBuilder {
    pub(crate) patch_filter_group: ::std::option::Option<crate::types::PatchFilterGroup>,
    pub(crate) compliance_level: ::std::option::Option<crate::types::PatchComplianceLevel>,
    pub(crate) approve_after_days: ::std::option::Option<i32>,
    pub(crate) approve_until_date: ::std::option::Option<::std::string::String>,
    pub(crate) enable_non_security: ::std::option::Option<bool>,
}
impl PatchRuleBuilder {
    /// <p>The patch filter group that defines the criteria for the rule.</p>
    pub fn patch_filter_group(mut self, input: crate::types::PatchFilterGroup) -> Self {
        self.patch_filter_group = ::std::option::Option::Some(input);
        self
    }
    /// <p>The patch filter group that defines the criteria for the rule.</p>
    pub fn set_patch_filter_group(mut self, input: ::std::option::Option<crate::types::PatchFilterGroup>) -> Self {
        self.patch_filter_group = input;
        self
    }
    /// <p>The patch filter group that defines the criteria for the rule.</p>
    pub fn get_patch_filter_group(&self) -> &::std::option::Option<crate::types::PatchFilterGroup> {
        &self.patch_filter_group
    }
    /// <p>A compliance severity level for all approved patches in a patch baseline.</p>
    pub fn compliance_level(mut self, input: crate::types::PatchComplianceLevel) -> Self {
        self.compliance_level = ::std::option::Option::Some(input);
        self
    }
    /// <p>A compliance severity level for all approved patches in a patch baseline.</p>
    pub fn set_compliance_level(mut self, input: ::std::option::Option<crate::types::PatchComplianceLevel>) -> Self {
        self.compliance_level = input;
        self
    }
    /// <p>A compliance severity level for all approved patches in a patch baseline.</p>
    pub fn get_compliance_level(&self) -> &::std::option::Option<crate::types::PatchComplianceLevel> {
        &self.compliance_level
    }
    /// <p>The number of days after the release date of each patch matched by the rule that the patch is marked as approved in the patch baseline. For example, a value of <code>7</code> means that patches are approved seven days after they are released. Not supported on Debian Server or Ubuntu Server.</p>
    pub fn approve_after_days(mut self, input: i32) -> Self {
        self.approve_after_days = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of days after the release date of each patch matched by the rule that the patch is marked as approved in the patch baseline. For example, a value of <code>7</code> means that patches are approved seven days after they are released. Not supported on Debian Server or Ubuntu Server.</p>
    pub fn set_approve_after_days(mut self, input: ::std::option::Option<i32>) -> Self {
        self.approve_after_days = input;
        self
    }
    /// <p>The number of days after the release date of each patch matched by the rule that the patch is marked as approved in the patch baseline. For example, a value of <code>7</code> means that patches are approved seven days after they are released. Not supported on Debian Server or Ubuntu Server.</p>
    pub fn get_approve_after_days(&self) -> &::std::option::Option<i32> {
        &self.approve_after_days
    }
    /// <p>The cutoff date for auto approval of released patches. Any patches released on or before this date are installed automatically. Not supported on Debian Server or Ubuntu Server.</p>
    /// <p>Enter dates in the format <code>YYYY-MM-DD</code>. For example, <code>2021-12-31</code>.</p>
    pub fn approve_until_date(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.approve_until_date = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The cutoff date for auto approval of released patches. Any patches released on or before this date are installed automatically. Not supported on Debian Server or Ubuntu Server.</p>
    /// <p>Enter dates in the format <code>YYYY-MM-DD</code>. For example, <code>2021-12-31</code>.</p>
    pub fn set_approve_until_date(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.approve_until_date = input;
        self
    }
    /// <p>The cutoff date for auto approval of released patches. Any patches released on or before this date are installed automatically. Not supported on Debian Server or Ubuntu Server.</p>
    /// <p>Enter dates in the format <code>YYYY-MM-DD</code>. For example, <code>2021-12-31</code>.</p>
    pub fn get_approve_until_date(&self) -> &::std::option::Option<::std::string::String> {
        &self.approve_until_date
    }
    /// <p>For managed nodes identified by the approval rule filters, enables a patch baseline to apply non-security updates available in the specified repository. The default value is <code>false</code>. Applies to Linux managed nodes only.</p>
    pub fn enable_non_security(mut self, input: bool) -> Self {
        self.enable_non_security = ::std::option::Option::Some(input);
        self
    }
    /// <p>For managed nodes identified by the approval rule filters, enables a patch baseline to apply non-security updates available in the specified repository. The default value is <code>false</code>. Applies to Linux managed nodes only.</p>
    pub fn set_enable_non_security(mut self, input: ::std::option::Option<bool>) -> Self {
        self.enable_non_security = input;
        self
    }
    /// <p>For managed nodes identified by the approval rule filters, enables a patch baseline to apply non-security updates available in the specified repository. The default value is <code>false</code>. Applies to Linux managed nodes only.</p>
    pub fn get_enable_non_security(&self) -> &::std::option::Option<bool> {
        &self.enable_non_security
    }
    /// Consumes the builder and constructs a [`PatchRule`](crate::types::PatchRule).
    pub fn build(self) -> crate::types::PatchRule {
        crate::types::PatchRule {
            patch_filter_group: self.patch_filter_group,
            compliance_level: self.compliance_level,
            approve_after_days: self.approve_after_days,
            approve_until_date: self.approve_until_date,
            enable_non_security: self.enable_non_security,
        }
    }
}
