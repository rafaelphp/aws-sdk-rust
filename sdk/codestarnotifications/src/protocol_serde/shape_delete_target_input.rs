// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_target_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::delete_target::DeleteTargetInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if input.force_unsubscribe_all {
        object.key("ForceUnsubscribeAll").boolean(input.force_unsubscribe_all);
    }
    if let Some(var_1) = &input.target_address {
        object.key("TargetAddress").string(var_1.as_str());
    }
    Ok(())
}
