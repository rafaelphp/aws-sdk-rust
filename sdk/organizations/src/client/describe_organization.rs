// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeOrganization`](crate::operation::describe_organization::builders::DescribeOrganizationFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::describe_organization::builders::DescribeOrganizationFluentBuilder::send) it.
    /// - On success, responds with [`DescribeOrganizationOutput`](crate::operation::describe_organization::DescribeOrganizationOutput) with field(s):
    ///   - [`organization(Option<Organization>)`](crate::operation::describe_organization::DescribeOrganizationOutput::organization): <p>A structure that contains information about the organization.</p> <important>   <p>The <code>AvailablePolicyTypes</code> part of the response is deprecated, and you shouldn't use it in your apps. It doesn't include any policy type supported by Organizations other than SCPs. To determine which policy types are enabled in your organization, use the <code> <code>ListRoots</code> </code> operation.</p>  </important>
    /// - On failure, responds with [`SdkError<DescribeOrganizationError>`](crate::operation::describe_organization::DescribeOrganizationError)
    pub fn describe_organization(&self) -> crate::operation::describe_organization::builders::DescribeOrganizationFluentBuilder {
        crate::operation::describe_organization::builders::DescribeOrganizationFluentBuilder::new(self.handle.clone())
    }
}
