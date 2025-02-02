// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_evaluation_request(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::EvaluationRequest,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.feature {
        object.key("feature").string(var_1.as_str());
    }
    if let Some(var_2) = &input.entity_id {
        object.key("entityId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.evaluation_context {
        object.key("evaluationContext").string(var_3.as_str());
    }
    Ok(())
}
