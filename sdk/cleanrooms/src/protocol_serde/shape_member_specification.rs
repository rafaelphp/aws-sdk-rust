// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_member_specification(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::MemberSpecification,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.account_id {
        object.key("accountId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.member_abilities {
        let mut array_3 = object.key("memberAbilities").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.display_name {
        object.key("displayName").string(var_5.as_str());
    }
    Ok(())
}
