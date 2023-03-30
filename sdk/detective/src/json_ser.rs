// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_accept_invitation_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AcceptInvitationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.graph_arn {
        object.key("GraphArn").string(var_1.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_get_graph_member_datasources_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::BatchGetGraphMemberDatasourcesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_2) = &input.account_ids {
        let mut array_3 = object.key("AccountIds").start_array();
        for item_4 in var_2 {
             {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.graph_arn {
        object.key("GraphArn").string(var_5.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_get_membership_datasources_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::BatchGetMembershipDatasourcesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_6) = &input.graph_arns {
        let mut array_7 = object.key("GraphArns").start_array();
        for item_8 in var_6 {
             {
                array_7.value().string(item_8.as_str());
            }
        }
        array_7.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_graph_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateGraphInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_9) = &input.tags {
        #[allow(unused_mut)]
        let mut object_10 = object.key("Tags").start_object();
        for (key_11, value_12) in var_9 {
             {
                object_10.key(key_11.as_str()).string(value_12.as_str());
            }
        }
        object_10.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_members_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateMembersInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_13) = &input.accounts {
        let mut array_14 = object.key("Accounts").start_array();
        for item_15 in var_13 {
             {
                #[allow(unused_mut)]
                let mut object_16 = array_14.value().start_object();
                crate::json_ser::serialize_structure_crate_model_account(&mut object_16, item_15)?;
                object_16.finish();
            }
        }
        array_14.finish();
    }
    if input.disable_email_notification {
        object.key("DisableEmailNotification").boolean(input.disable_email_notification);
    }
    if let Some(var_17) = &input.graph_arn {
        object.key("GraphArn").string(var_17.as_str());
    }
    if let Some(var_18) = &input.message {
        object.key("Message").string(var_18.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_graph_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteGraphInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_19) = &input.graph_arn {
        object.key("GraphArn").string(var_19.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_members_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteMembersInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_20) = &input.account_ids {
        let mut array_21 = object.key("AccountIds").start_array();
        for item_22 in var_20 {
             {
                array_21.value().string(item_22.as_str());
            }
        }
        array_21.finish();
    }
    if let Some(var_23) = &input.graph_arn {
        object.key("GraphArn").string(var_23.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_organization_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeOrganizationConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_24) = &input.graph_arn {
        object.key("GraphArn").string(var_24.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_disassociate_membership_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DisassociateMembershipInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_25) = &input.graph_arn {
        object.key("GraphArn").string(var_25.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_enable_organization_admin_account_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::EnableOrganizationAdminAccountInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_26) = &input.account_id {
        object.key("AccountId").string(var_26.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_members_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetMembersInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_27) = &input.account_ids {
        let mut array_28 = object.key("AccountIds").start_array();
        for item_29 in var_27 {
             {
                array_28.value().string(item_29.as_str());
            }
        }
        array_28.finish();
    }
    if let Some(var_30) = &input.graph_arn {
        object.key("GraphArn").string(var_30.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_datasource_packages_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListDatasourcePackagesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_31) = &input.graph_arn {
        object.key("GraphArn").string(var_31.as_str());
    }
    if let Some(var_32) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_32).into()));
    }
    if let Some(var_33) = &input.next_token {
        object.key("NextToken").string(var_33.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_graphs_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListGraphsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_34) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_34).into()));
    }
    if let Some(var_35) = &input.next_token {
        object.key("NextToken").string(var_35.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_invitations_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListInvitationsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_36) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_36).into()));
    }
    if let Some(var_37) = &input.next_token {
        object.key("NextToken").string(var_37.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_members_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListMembersInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_38) = &input.graph_arn {
        object.key("GraphArn").string(var_38.as_str());
    }
    if let Some(var_39) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_39).into()));
    }
    if let Some(var_40) = &input.next_token {
        object.key("NextToken").string(var_40.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_organization_admin_accounts_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListOrganizationAdminAccountsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_41) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_41).into()));
    }
    if let Some(var_42) = &input.next_token {
        object.key("NextToken").string(var_42.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_reject_invitation_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::RejectInvitationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_43) = &input.graph_arn {
        object.key("GraphArn").string(var_43.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_monitoring_member_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartMonitoringMemberInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_44) = &input.account_id {
        object.key("AccountId").string(var_44.as_str());
    }
    if let Some(var_45) = &input.graph_arn {
        object.key("GraphArn").string(var_45.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_46) = &input.tags {
        #[allow(unused_mut)]
        let mut object_47 = object.key("Tags").start_object();
        for (key_48, value_49) in var_46 {
             {
                object_47.key(key_48.as_str()).string(value_49.as_str());
            }
        }
        object_47.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_datasource_packages_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateDatasourcePackagesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_50) = &input.datasource_packages {
        let mut array_51 = object.key("DatasourcePackages").start_array();
        for item_52 in var_50 {
             {
                array_51.value().string(item_52.as_str());
            }
        }
        array_51.finish();
    }
    if let Some(var_53) = &input.graph_arn {
        object.key("GraphArn").string(var_53.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_organization_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateOrganizationConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.auto_enable {
        object.key("AutoEnable").boolean(input.auto_enable);
    }
    if let Some(var_54) = &input.graph_arn {
        object.key("GraphArn").string(var_54.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_account(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Account) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_55) = &input.account_id {
        object.key("AccountId").string(var_55.as_str());
    }
    if let Some(var_56) = &input.email_address {
        object.key("EmailAddress").string(var_56.as_str());
    }
    Ok(())
}

