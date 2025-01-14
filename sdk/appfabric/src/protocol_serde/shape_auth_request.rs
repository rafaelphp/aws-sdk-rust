// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_auth_request(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AuthRequest,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.redirect_uri {
        object.key("redirectUri").string(var_1.as_str());
    }
    if let Some(var_2) = &input.code {
        object.key("code").string(var_2.as_str());
    }
    Ok(())
}
