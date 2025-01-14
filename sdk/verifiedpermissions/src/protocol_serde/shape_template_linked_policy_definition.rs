// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_template_linked_policy_definition(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TemplateLinkedPolicyDefinition,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.policy_template_id {
        object.key("policyTemplateId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.principal {
        #[allow(unused_mut)]
        let mut object_3 = object.key("principal").start_object();
        crate::protocol_serde::shape_entity_identifier::ser_entity_identifier(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.resource {
        #[allow(unused_mut)]
        let mut object_5 = object.key("resource").start_object();
        crate::protocol_serde::shape_entity_identifier::ser_entity_identifier(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}
