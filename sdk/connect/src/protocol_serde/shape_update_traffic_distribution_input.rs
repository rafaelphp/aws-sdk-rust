// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_traffic_distribution_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_traffic_distribution::UpdateTrafficDistributionInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.telephony_config {
        #[allow(unused_mut)]
        let mut object_2 = object.key("TelephonyConfig").start_object();
        crate::protocol_serde::shape_telephony_config::ser_telephony_config(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}
