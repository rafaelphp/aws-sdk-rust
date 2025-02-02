// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteIpGroup`](crate::operation::delete_ip_group::builders::DeleteIpGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`group_id(impl Into<String>)`](crate::operation::delete_ip_group::builders::DeleteIpGroupFluentBuilder::group_id) / [`set_group_id(Option<String>)`](crate::operation::delete_ip_group::builders::DeleteIpGroupFluentBuilder::set_group_id): <p>The identifier of the IP access control group.</p>
    /// - On success, responds with [`DeleteIpGroupOutput`](crate::operation::delete_ip_group::DeleteIpGroupOutput)
    /// - On failure, responds with [`SdkError<DeleteIpGroupError>`](crate::operation::delete_ip_group::DeleteIpGroupError)
    pub fn delete_ip_group(&self) -> crate::operation::delete_ip_group::builders::DeleteIpGroupFluentBuilder {
        crate::operation::delete_ip_group::builders::DeleteIpGroupFluentBuilder::new(self.handle.clone())
    }
}
