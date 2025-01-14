// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateDefaultVpc`](crate::operation::create_default_vpc::builders::CreateDefaultVpcFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::create_default_vpc::builders::CreateDefaultVpcFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_default_vpc::builders::CreateDefaultVpcFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`CreateDefaultVpcOutput`](crate::operation::create_default_vpc::CreateDefaultVpcOutput) with field(s):
    ///   - [`vpc(Option<Vpc>)`](crate::operation::create_default_vpc::CreateDefaultVpcOutput::vpc): <p>Information about the VPC.</p>
    /// - On failure, responds with [`SdkError<CreateDefaultVpcError>`](crate::operation::create_default_vpc::CreateDefaultVpcError)
    pub fn create_default_vpc(&self) -> crate::operation::create_default_vpc::builders::CreateDefaultVpcFluentBuilder {
        crate::operation::create_default_vpc::builders::CreateDefaultVpcFluentBuilder::new(self.handle.clone())
    }
}
