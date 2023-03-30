// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_list_realtime_contact_analysis_segments_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListRealtimeContactAnalysisSegmentsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.contact_id {
        object.key("ContactId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.instance_id {
        object.key("InstanceId").string(var_2.as_str());
    }
    if input.max_results != 0 {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_results).into()));
    }
    if let Some(var_3) = &input.next_token {
        object.key("NextToken").string(var_3.as_str());
    }
    Ok(())
}

