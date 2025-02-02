// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_configuration(
    object_4: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::UpdateConfiguration,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::types::UpdateConfiguration::CognitoUserPoolConfiguration(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_4.key("cognitoUserPoolConfiguration").start_object();
            crate::protocol_serde::shape_update_cognito_user_pool_configuration::ser_update_cognito_user_pool_configuration(&mut object_1, inner)?;
            object_1.finish();
        }
        crate::types::UpdateConfiguration::Unknown => {
            return Err(::aws_smithy_http::operation::error::SerializationError::unknown_variant(
                "UpdateConfiguration",
            ))
        }
    }
    Ok(())
}
