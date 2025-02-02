// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_low_latency_hls_manifest_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CreateLowLatencyHlsManifestConfiguration,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.manifest_name {
        object.key("ManifestName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.child_manifest_name {
        object.key("ChildManifestName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.scte_hls {
        #[allow(unused_mut)]
        let mut object_4 = object.key("ScteHls").start_object();
        crate::protocol_serde::shape_scte_hls::ser_scte_hls(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.manifest_window_seconds {
        object.key("ManifestWindowSeconds").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    if let Some(var_6) = &input.program_date_time_interval_seconds {
        object.key("ProgramDateTimeIntervalSeconds").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    Ok(())
}
