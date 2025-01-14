// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeVirtualGateways`](crate::operation::describe_virtual_gateways::builders::DescribeVirtualGatewaysFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::describe_virtual_gateways::builders::DescribeVirtualGatewaysFluentBuilder::send) it.
    /// - On success, responds with [`DescribeVirtualGatewaysOutput`](crate::operation::describe_virtual_gateways::DescribeVirtualGatewaysOutput) with field(s):
    ///   - [`virtual_gateways(Option<Vec<VirtualGateway>>)`](crate::operation::describe_virtual_gateways::DescribeVirtualGatewaysOutput::virtual_gateways): <p>The virtual private gateways.</p>
    /// - On failure, responds with [`SdkError<DescribeVirtualGatewaysError>`](crate::operation::describe_virtual_gateways::DescribeVirtualGatewaysError)
    pub fn describe_virtual_gateways(&self) -> crate::operation::describe_virtual_gateways::builders::DescribeVirtualGatewaysFluentBuilder {
        crate::operation::describe_virtual_gateways::builders::DescribeVirtualGatewaysFluentBuilder::new(self.handle.clone())
    }
}
