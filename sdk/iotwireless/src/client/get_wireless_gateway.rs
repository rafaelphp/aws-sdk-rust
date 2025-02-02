// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetWirelessGateway`](crate::operation::get_wireless_gateway::builders::GetWirelessGatewayFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`identifier(impl Into<String>)`](crate::operation::get_wireless_gateway::builders::GetWirelessGatewayFluentBuilder::identifier) / [`set_identifier(Option<String>)`](crate::operation::get_wireless_gateway::builders::GetWirelessGatewayFluentBuilder::set_identifier): <p>The identifier of the wireless gateway to get.</p>
    ///   - [`identifier_type(WirelessGatewayIdType)`](crate::operation::get_wireless_gateway::builders::GetWirelessGatewayFluentBuilder::identifier_type) / [`set_identifier_type(Option<WirelessGatewayIdType>)`](crate::operation::get_wireless_gateway::builders::GetWirelessGatewayFluentBuilder::set_identifier_type): <p>The type of identifier used in <code>identifier</code>.</p>
    /// - On success, responds with [`GetWirelessGatewayOutput`](crate::operation::get_wireless_gateway::GetWirelessGatewayOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::operation::get_wireless_gateway::GetWirelessGatewayOutput::name): <p>The name of the resource.</p>
    ///   - [`id(Option<String>)`](crate::operation::get_wireless_gateway::GetWirelessGatewayOutput::id): <p>The ID of the wireless gateway.</p>
    ///   - [`description(Option<String>)`](crate::operation::get_wireless_gateway::GetWirelessGatewayOutput::description): <p>The description of the resource.</p>
    ///   - [`lo_ra_wan(Option<LoRaWanGateway>)`](crate::operation::get_wireless_gateway::GetWirelessGatewayOutput::lo_ra_wan): <p>Information about the wireless gateway.</p>
    ///   - [`arn(Option<String>)`](crate::operation::get_wireless_gateway::GetWirelessGatewayOutput::arn): <p>The Amazon Resource Name of the resource.</p>
    ///   - [`thing_name(Option<String>)`](crate::operation::get_wireless_gateway::GetWirelessGatewayOutput::thing_name): <p>The name of the thing associated with the wireless gateway. The value is empty if a thing isn't associated with the gateway.</p>
    ///   - [`thing_arn(Option<String>)`](crate::operation::get_wireless_gateway::GetWirelessGatewayOutput::thing_arn): <p>The ARN of the thing associated with the wireless gateway.</p>
    /// - On failure, responds with [`SdkError<GetWirelessGatewayError>`](crate::operation::get_wireless_gateway::GetWirelessGatewayError)
    pub fn get_wireless_gateway(&self) -> crate::operation::get_wireless_gateway::builders::GetWirelessGatewayFluentBuilder {
        crate::operation::get_wireless_gateway::builders::GetWirelessGatewayFluentBuilder::new(self.handle.clone())
    }
}
