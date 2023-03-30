// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_application_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateApplicationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.author {
        object.key("author").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.home_page_url {
        object.key("homePageUrl").string(var_3.as_str());
    }
    if let Some(var_4) = &input.labels {
        let mut array_5 = object.key("labels").start_array();
        for item_6 in var_4 {
             {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.license_body {
        object.key("licenseBody").string(var_7.as_str());
    }
    if let Some(var_8) = &input.license_url {
        object.key("licenseUrl").string(var_8.as_str());
    }
    if let Some(var_9) = &input.name {
        object.key("name").string(var_9.as_str());
    }
    if let Some(var_10) = &input.readme_body {
        object.key("readmeBody").string(var_10.as_str());
    }
    if let Some(var_11) = &input.readme_url {
        object.key("readmeUrl").string(var_11.as_str());
    }
    if let Some(var_12) = &input.semantic_version {
        object.key("semanticVersion").string(var_12.as_str());
    }
    if let Some(var_13) = &input.source_code_archive_url {
        object.key("sourceCodeArchiveUrl").string(var_13.as_str());
    }
    if let Some(var_14) = &input.source_code_url {
        object.key("sourceCodeUrl").string(var_14.as_str());
    }
    if let Some(var_15) = &input.spdx_license_id {
        object.key("spdxLicenseId").string(var_15.as_str());
    }
    if let Some(var_16) = &input.template_body {
        object.key("templateBody").string(var_16.as_str());
    }
    if let Some(var_17) = &input.template_url {
        object.key("templateUrl").string(var_17.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_application_version_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateApplicationVersionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_18) = &input.source_code_archive_url {
        object.key("sourceCodeArchiveUrl").string(var_18.as_str());
    }
    if let Some(var_19) = &input.source_code_url {
        object.key("sourceCodeUrl").string(var_19.as_str());
    }
    if let Some(var_20) = &input.template_body {
        object.key("templateBody").string(var_20.as_str());
    }
    if let Some(var_21) = &input.template_url {
        object.key("templateUrl").string(var_21.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_cloud_formation_change_set_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateCloudFormationChangeSetInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_22) = &input.capabilities {
        let mut array_23 = object.key("capabilities").start_array();
        for item_24 in var_22 {
             {
                array_23.value().string(item_24.as_str());
            }
        }
        array_23.finish();
    }
    if let Some(var_25) = &input.change_set_name {
        object.key("changeSetName").string(var_25.as_str());
    }
    if let Some(var_26) = &input.client_token {
        object.key("clientToken").string(var_26.as_str());
    }
    if let Some(var_27) = &input.description {
        object.key("description").string(var_27.as_str());
    }
    if let Some(var_28) = &input.notification_arns {
        let mut array_29 = object.key("notificationArns").start_array();
        for item_30 in var_28 {
             {
                array_29.value().string(item_30.as_str());
            }
        }
        array_29.finish();
    }
    if let Some(var_31) = &input.parameter_overrides {
        let mut array_32 = object.key("parameterOverrides").start_array();
        for item_33 in var_31 {
             {
                #[allow(unused_mut)]
                let mut object_34 = array_32.value().start_object();
                crate::json_ser::serialize_structure_crate_model_parameter_value(&mut object_34, item_33)?;
                object_34.finish();
            }
        }
        array_32.finish();
    }
    if let Some(var_35) = &input.resource_types {
        let mut array_36 = object.key("resourceTypes").start_array();
        for item_37 in var_35 {
             {
                array_36.value().string(item_37.as_str());
            }
        }
        array_36.finish();
    }
    if let Some(var_38) = &input.rollback_configuration {
        #[allow(unused_mut)]
        let mut object_39 = object.key("rollbackConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_rollback_configuration(&mut object_39, var_38)?;
        object_39.finish();
    }
    if let Some(var_40) = &input.semantic_version {
        object.key("semanticVersion").string(var_40.as_str());
    }
    if let Some(var_41) = &input.stack_name {
        object.key("stackName").string(var_41.as_str());
    }
    if let Some(var_42) = &input.tags {
        let mut array_43 = object.key("tags").start_array();
        for item_44 in var_42 {
             {
                #[allow(unused_mut)]
                let mut object_45 = array_43.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_45, item_44)?;
                object_45.finish();
            }
        }
        array_43.finish();
    }
    if let Some(var_46) = &input.template_id {
        object.key("templateId").string(var_46.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_cloud_formation_template_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateCloudFormationTemplateInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_47) = &input.semantic_version {
        object.key("semanticVersion").string(var_47.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_application_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutApplicationPolicyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_48) = &input.statements {
        let mut array_49 = object.key("statements").start_array();
        for item_50 in var_48 {
             {
                #[allow(unused_mut)]
                let mut object_51 = array_49.value().start_object();
                crate::json_ser::serialize_structure_crate_model_application_policy_statement(&mut object_51, item_50)?;
                object_51.finish();
            }
        }
        array_49.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_unshare_application_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UnshareApplicationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_52) = &input.organization_id {
        object.key("organizationId").string(var_52.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_application_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateApplicationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_53) = &input.author {
        object.key("author").string(var_53.as_str());
    }
    if let Some(var_54) = &input.description {
        object.key("description").string(var_54.as_str());
    }
    if let Some(var_55) = &input.home_page_url {
        object.key("homePageUrl").string(var_55.as_str());
    }
    if let Some(var_56) = &input.labels {
        let mut array_57 = object.key("labels").start_array();
        for item_58 in var_56 {
             {
                array_57.value().string(item_58.as_str());
            }
        }
        array_57.finish();
    }
    if let Some(var_59) = &input.readme_body {
        object.key("readmeBody").string(var_59.as_str());
    }
    if let Some(var_60) = &input.readme_url {
        object.key("readmeUrl").string(var_60.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_parameter_value(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ParameterValue) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_61) = &input.name {
        object.key("name").string(var_61.as_str());
    }
    if let Some(var_62) = &input.value {
        object.key("value").string(var_62.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_rollback_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::RollbackConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.monitoring_time_in_minutes != 0 {
        object.key("monitoringTimeInMinutes").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.monitoring_time_in_minutes).into()));
    }
    if let Some(var_63) = &input.rollback_triggers {
        let mut array_64 = object.key("rollbackTriggers").start_array();
        for item_65 in var_63 {
             {
                #[allow(unused_mut)]
                let mut object_66 = array_64.value().start_object();
                crate::json_ser::serialize_structure_crate_model_rollback_trigger(&mut object_66, item_65)?;
                object_66.finish();
            }
        }
        array_64.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Tag) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_67) = &input.key {
        object.key("key").string(var_67.as_str());
    }
    if let Some(var_68) = &input.value {
        object.key("value").string(var_68.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_application_policy_statement(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ApplicationPolicyStatement) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_69) = &input.actions {
        let mut array_70 = object.key("actions").start_array();
        for item_71 in var_69 {
             {
                array_70.value().string(item_71.as_str());
            }
        }
        array_70.finish();
    }
    if let Some(var_72) = &input.principal_org_i_ds {
        let mut array_73 = object.key("principalOrgIDs").start_array();
        for item_74 in var_72 {
             {
                array_73.value().string(item_74.as_str());
            }
        }
        array_73.finish();
    }
    if let Some(var_75) = &input.principals {
        let mut array_76 = object.key("principals").start_array();
        for item_77 in var_75 {
             {
                array_76.value().string(item_77.as_str());
            }
        }
        array_76.finish();
    }
    if let Some(var_78) = &input.statement_id {
        object.key("statementId").string(var_78.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_rollback_trigger(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::RollbackTrigger) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_79) = &input.arn {
        object.key("arn").string(var_79.as_str());
    }
    if let Some(var_80) = &input.r#type {
        object.key("type").string(var_80.as_str());
    }
    Ok(())
}

