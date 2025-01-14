// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_virtual_object(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::VirtualObject,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.uri {
        object.key("Uri").string(var_1.as_str());
    }
    if let Some(var_2) = &input.e_tag {
        object.key("ETag").string(var_2.as_str());
    }
    Ok(())
}
