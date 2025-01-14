// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_input_device_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_input_device::UpdateInputDeviceInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.hd_device_settings {
        #[allow(unused_mut)]
        let mut object_2 = object.key("hdDeviceSettings").start_object();
        crate::protocol_serde::shape_input_device_configurable_settings::ser_input_device_configurable_settings(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.name {
        object.key("name").string(var_3.as_str());
    }
    if let Some(var_4) = &input.uhd_device_settings {
        #[allow(unused_mut)]
        let mut object_5 = object.key("uhdDeviceSettings").start_object();
        crate::protocol_serde::shape_input_device_configurable_settings::ser_input_device_configurable_settings(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}
