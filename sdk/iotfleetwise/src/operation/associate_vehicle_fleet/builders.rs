// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_vehicle_fleet::_associate_vehicle_fleet_output::AssociateVehicleFleetOutputBuilder;

pub use crate::operation::associate_vehicle_fleet::_associate_vehicle_fleet_input::AssociateVehicleFleetInputBuilder;

impl AssociateVehicleFleetInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::associate_vehicle_fleet::AssociateVehicleFleetOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::associate_vehicle_fleet::AssociateVehicleFleetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.associate_vehicle_fleet();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AssociateVehicleFleet`.
///
/// <p> Adds, or associates, a vehicle with a fleet. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AssociateVehicleFleetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::associate_vehicle_fleet::builders::AssociateVehicleFleetInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl AssociateVehicleFleetFluentBuilder {
    /// Creates a new `AssociateVehicleFleet`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AssociateVehicleFleet as a reference.
    pub fn as_input(&self) -> &crate::operation::associate_vehicle_fleet::builders::AssociateVehicleFleetInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::associate_vehicle_fleet::AssociateVehicleFleetOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::associate_vehicle_fleet::AssociateVehicleFleetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::associate_vehicle_fleet::AssociateVehicleFleet::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::associate_vehicle_fleet::AssociateVehicleFleet::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::associate_vehicle_fleet::AssociateVehicleFleetOutput,
            crate::operation::associate_vehicle_fleet::AssociateVehicleFleetError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::associate_vehicle_fleet::AssociateVehicleFleetError>,
    > {
        ::std::result::Result::Ok(crate::client::customize::orchestrator::CustomizableOperation {
            customizable_send: ::std::boxed::Box::new(move |config_override| {
                ::std::boxed::Box::pin(async { self.config_override(config_override).send().await })
            }),
            config_override: None,
            interceptors: vec![],
            runtime_plugins: vec![],
        })
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p> The unique ID of the vehicle to associate with the fleet. </p>
    pub fn vehicle_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vehicle_name(input.into());
        self
    }
    /// <p> The unique ID of the vehicle to associate with the fleet. </p>
    pub fn set_vehicle_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vehicle_name(input);
        self
    }
    /// <p> The unique ID of the vehicle to associate with the fleet. </p>
    pub fn get_vehicle_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_vehicle_name()
    }
    /// <p> The ID of a fleet. </p>
    pub fn fleet_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.fleet_id(input.into());
        self
    }
    /// <p> The ID of a fleet. </p>
    pub fn set_fleet_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_fleet_id(input);
        self
    }
    /// <p> The ID of a fleet. </p>
    pub fn get_fleet_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_fleet_id()
    }
}
