// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisassociateCustomerGatewayInput {
    /// <p>The ID of the global network.</p>
    pub global_network_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the customer gateway.</p>
    pub customer_gateway_arn: ::std::option::Option<::std::string::String>,
}
impl DisassociateCustomerGatewayInput {
    /// <p>The ID of the global network.</p>
    pub fn global_network_id(&self) -> ::std::option::Option<&str> {
        self.global_network_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the customer gateway.</p>
    pub fn customer_gateway_arn(&self) -> ::std::option::Option<&str> {
        self.customer_gateway_arn.as_deref()
    }
}
impl DisassociateCustomerGatewayInput {
    /// Creates a new builder-style object to manufacture [`DisassociateCustomerGatewayInput`](crate::operation::disassociate_customer_gateway::DisassociateCustomerGatewayInput).
    pub fn builder() -> crate::operation::disassociate_customer_gateway::builders::DisassociateCustomerGatewayInputBuilder {
        crate::operation::disassociate_customer_gateway::builders::DisassociateCustomerGatewayInputBuilder::default()
    }
}

/// A builder for [`DisassociateCustomerGatewayInput`](crate::operation::disassociate_customer_gateway::DisassociateCustomerGatewayInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DisassociateCustomerGatewayInputBuilder {
    pub(crate) global_network_id: ::std::option::Option<::std::string::String>,
    pub(crate) customer_gateway_arn: ::std::option::Option<::std::string::String>,
}
impl DisassociateCustomerGatewayInputBuilder {
    /// <p>The ID of the global network.</p>
    pub fn global_network_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.global_network_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the global network.</p>
    pub fn set_global_network_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.global_network_id = input;
        self
    }
    /// <p>The ID of the global network.</p>
    pub fn get_global_network_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.global_network_id
    }
    /// <p>The Amazon Resource Name (ARN) of the customer gateway.</p>
    pub fn customer_gateway_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.customer_gateway_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the customer gateway.</p>
    pub fn set_customer_gateway_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.customer_gateway_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the customer gateway.</p>
    pub fn get_customer_gateway_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.customer_gateway_arn
    }
    /// Consumes the builder and constructs a [`DisassociateCustomerGatewayInput`](crate::operation::disassociate_customer_gateway::DisassociateCustomerGatewayInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::disassociate_customer_gateway::DisassociateCustomerGatewayInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::disassociate_customer_gateway::DisassociateCustomerGatewayInput {
            global_network_id: self.global_network_id,
            customer_gateway_arn: self.customer_gateway_arn,
        })
    }
}
