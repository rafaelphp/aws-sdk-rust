// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_workspace_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateWorkspaceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.account_access_type {
        object.key("accountAccessType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.authentication_providers {
        let mut array_3 = object.key("authenticationProviders").start_array();
        for item_4 in var_2 {
             {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.client_token {
        object.key("clientToken").string(var_5.as_str());
    }
    if let Some(var_6) = &input.configuration {
        object.key("configuration").string(var_6.as_str());
    }
    if let Some(var_7) = &input.organization_role_name {
        object.key("organizationRoleName").string(var_7.as_str());
    }
    if let Some(var_8) = &input.permission_type {
        object.key("permissionType").string(var_8.as_str());
    }
    if let Some(var_9) = &input.stack_set_name {
        object.key("stackSetName").string(var_9.as_str());
    }
    if let Some(var_10) = &input.tags {
        #[allow(unused_mut)]
        let mut object_11 = object.key("tags").start_object();
        for (key_12, value_13) in var_10 {
             {
                object_11.key(key_12.as_str()).string(value_13.as_str());
            }
        }
        object_11.finish();
    }
    if let Some(var_14) = &input.vpc_configuration {
        #[allow(unused_mut)]
        let mut object_15 = object.key("vpcConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_vpc_configuration(&mut object_15, var_14)?;
        object_15.finish();
    }
    if let Some(var_16) = &input.workspace_data_sources {
        let mut array_17 = object.key("workspaceDataSources").start_array();
        for item_18 in var_16 {
             {
                array_17.value().string(item_18.as_str());
            }
        }
        array_17.finish();
    }
    if let Some(var_19) = &input.workspace_description {
        object.key("workspaceDescription").string(var_19.as_str());
    }
    if let Some(var_20) = &input.workspace_name {
        object.key("workspaceName").string(var_20.as_str());
    }
    if let Some(var_21) = &input.workspace_notification_destinations {
        let mut array_22 = object.key("workspaceNotificationDestinations").start_array();
        for item_23 in var_21 {
             {
                array_22.value().string(item_23.as_str());
            }
        }
        array_22.finish();
    }
    if let Some(var_24) = &input.workspace_organizational_units {
        let mut array_25 = object.key("workspaceOrganizationalUnits").start_array();
        for item_26 in var_24 {
             {
                array_25.value().string(item_26.as_str());
            }
        }
        array_25.finish();
    }
    if let Some(var_27) = &input.workspace_role_arn {
        object.key("workspaceRoleArn").string(var_27.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_workspace_api_key_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateWorkspaceApiKeyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_28) = &input.key_name {
        object.key("keyName").string(var_28.as_str());
    }
    if let Some(var_29) = &input.key_role {
        object.key("keyRole").string(var_29.as_str());
    }
    if let Some(var_30) = &input.seconds_to_live {
        object.key("secondsToLive").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_30).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_31) = &input.tags {
        #[allow(unused_mut)]
        let mut object_32 = object.key("tags").start_object();
        for (key_33, value_34) in var_31 {
             {
                object_32.key(key_33.as_str()).string(value_34.as_str());
            }
        }
        object_32.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_permissions_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdatePermissionsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_35) = &input.update_instruction_batch {
        let mut array_36 = object.key("updateInstructionBatch").start_array();
        for item_37 in var_35 {
             {
                #[allow(unused_mut)]
                let mut object_38 = array_36.value().start_object();
                crate::json_ser::serialize_structure_crate_model_update_instruction(&mut object_38, item_37)?;
                object_38.finish();
            }
        }
        array_36.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_workspace_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateWorkspaceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_39) = &input.account_access_type {
        object.key("accountAccessType").string(var_39.as_str());
    }
    if let Some(var_40) = &input.organization_role_name {
        object.key("organizationRoleName").string(var_40.as_str());
    }
    if let Some(var_41) = &input.permission_type {
        object.key("permissionType").string(var_41.as_str());
    }
    if let Some(var_42) = &input.remove_vpc_configuration {
        object.key("removeVpcConfiguration").boolean(*var_42);
    }
    if let Some(var_43) = &input.stack_set_name {
        object.key("stackSetName").string(var_43.as_str());
    }
    if let Some(var_44) = &input.vpc_configuration {
        #[allow(unused_mut)]
        let mut object_45 = object.key("vpcConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_vpc_configuration(&mut object_45, var_44)?;
        object_45.finish();
    }
    if let Some(var_46) = &input.workspace_data_sources {
        let mut array_47 = object.key("workspaceDataSources").start_array();
        for item_48 in var_46 {
             {
                array_47.value().string(item_48.as_str());
            }
        }
        array_47.finish();
    }
    if let Some(var_49) = &input.workspace_description {
        object.key("workspaceDescription").string(var_49.as_str());
    }
    if let Some(var_50) = &input.workspace_name {
        object.key("workspaceName").string(var_50.as_str());
    }
    if let Some(var_51) = &input.workspace_notification_destinations {
        let mut array_52 = object.key("workspaceNotificationDestinations").start_array();
        for item_53 in var_51 {
             {
                array_52.value().string(item_53.as_str());
            }
        }
        array_52.finish();
    }
    if let Some(var_54) = &input.workspace_organizational_units {
        let mut array_55 = object.key("workspaceOrganizationalUnits").start_array();
        for item_56 in var_54 {
             {
                array_55.value().string(item_56.as_str());
            }
        }
        array_55.finish();
    }
    if let Some(var_57) = &input.workspace_role_arn {
        object.key("workspaceRoleArn").string(var_57.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_workspace_authentication_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateWorkspaceAuthenticationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_58) = &input.authentication_providers {
        let mut array_59 = object.key("authenticationProviders").start_array();
        for item_60 in var_58 {
             {
                array_59.value().string(item_60.as_str());
            }
        }
        array_59.finish();
    }
    if let Some(var_61) = &input.saml_configuration {
        #[allow(unused_mut)]
        let mut object_62 = object.key("samlConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_saml_configuration(&mut object_62, var_61)?;
        object_62.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_workspace_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateWorkspaceConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_63) = &input.configuration {
        object.key("configuration").string(var_63.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_vpc_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::VpcConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_64) = &input.security_group_ids {
        let mut array_65 = object.key("securityGroupIds").start_array();
        for item_66 in var_64 {
             {
                array_65.value().string(item_66.as_str());
            }
        }
        array_65.finish();
    }
    if let Some(var_67) = &input.subnet_ids {
        let mut array_68 = object.key("subnetIds").start_array();
        for item_69 in var_67 {
             {
                array_68.value().string(item_69.as_str());
            }
        }
        array_68.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_update_instruction(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::UpdateInstruction) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_70) = &input.action {
        object.key("action").string(var_70.as_str());
    }
    if let Some(var_71) = &input.role {
        object.key("role").string(var_71.as_str());
    }
    if let Some(var_72) = &input.users {
        let mut array_73 = object.key("users").start_array();
        for item_74 in var_72 {
             {
                #[allow(unused_mut)]
                let mut object_75 = array_73.value().start_object();
                crate::json_ser::serialize_structure_crate_model_user(&mut object_75, item_74)?;
                object_75.finish();
            }
        }
        array_73.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_saml_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SamlConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_76) = &input.idp_metadata {
        #[allow(unused_mut)]
        let mut object_77 = object.key("idpMetadata").start_object();
        crate::json_ser::serialize_union_crate_model_idp_metadata(&mut object_77, var_76)?;
        object_77.finish();
    }
    if let Some(var_78) = &input.assertion_attributes {
        #[allow(unused_mut)]
        let mut object_79 = object.key("assertionAttributes").start_object();
        crate::json_ser::serialize_structure_crate_model_assertion_attributes(&mut object_79, var_78)?;
        object_79.finish();
    }
    if let Some(var_80) = &input.role_values {
        #[allow(unused_mut)]
        let mut object_81 = object.key("roleValues").start_object();
        crate::json_ser::serialize_structure_crate_model_role_values(&mut object_81, var_80)?;
        object_81.finish();
    }
    if let Some(var_82) = &input.allowed_organizations {
        let mut array_83 = object.key("allowedOrganizations").start_array();
        for item_84 in var_82 {
             {
                array_83.value().string(item_84.as_str());
            }
        }
        array_83.finish();
    }
    if input.login_validity_duration != 0 {
        object.key("loginValidityDuration").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.login_validity_duration).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_model_user(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::User) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_85) = &input.id {
        object.key("id").string(var_85.as_str());
    }
    if let Some(var_86) = &input.r#type {
        object.key("type").string(var_86.as_str());
    }
    Ok(())
}

pub fn serialize_union_crate_model_idp_metadata(object_77: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::IdpMetadata) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::model::IdpMetadata::Url(inner) => {
             {
                object_77.key("url").string(inner.as_str());
            }
        },
        crate::model::IdpMetadata::Xml(inner) => {
             {
                object_77.key("xml").string(inner.as_str());
            }
        },
        crate::model::IdpMetadata::Unknown => return Err(aws_smithy_http::operation::error::SerializationError::unknown_variant("IdpMetadata"))
    }
    Ok(())
}

pub fn serialize_structure_crate_model_assertion_attributes(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AssertionAttributes) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_87) = &input.name {
        object.key("name").string(var_87.as_str());
    }
    if let Some(var_88) = &input.login {
        object.key("login").string(var_88.as_str());
    }
    if let Some(var_89) = &input.email {
        object.key("email").string(var_89.as_str());
    }
    if let Some(var_90) = &input.groups {
        object.key("groups").string(var_90.as_str());
    }
    if let Some(var_91) = &input.role {
        object.key("role").string(var_91.as_str());
    }
    if let Some(var_92) = &input.org {
        object.key("org").string(var_92.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_role_values(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::RoleValues) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_93) = &input.editor {
        let mut array_94 = object.key("editor").start_array();
        for item_95 in var_93 {
             {
                array_94.value().string(item_95.as_str());
            }
        }
        array_94.finish();
    }
    if let Some(var_96) = &input.admin {
        let mut array_97 = object.key("admin").start_array();
        for item_98 in var_96 {
             {
                array_97.value().string(item_98.as_str());
            }
        }
        array_97.finish();
    }
    Ok(())
}

