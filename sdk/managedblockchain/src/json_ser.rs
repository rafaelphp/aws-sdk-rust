// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_accessor_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateAccessorInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.accessor_type {
        object.key("AccessorType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_2.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_member_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateMemberInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_3) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.invitation_id {
        object.key("InvitationId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.member_configuration {
        #[allow(unused_mut)]
        let mut object_6 = object.key("MemberConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_member_configuration(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_network_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateNetworkInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_7) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_7.as_str());
    }
    if let Some(var_8) = &input.description {
        object.key("Description").string(var_8.as_str());
    }
    if let Some(var_9) = &input.framework {
        object.key("Framework").string(var_9.as_str());
    }
    if let Some(var_10) = &input.framework_configuration {
        #[allow(unused_mut)]
        let mut object_11 = object.key("FrameworkConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_network_framework_configuration(&mut object_11, var_10)?;
        object_11.finish();
    }
    if let Some(var_12) = &input.framework_version {
        object.key("FrameworkVersion").string(var_12.as_str());
    }
    if let Some(var_13) = &input.member_configuration {
        #[allow(unused_mut)]
        let mut object_14 = object.key("MemberConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_member_configuration(&mut object_14, var_13)?;
        object_14.finish();
    }
    if let Some(var_15) = &input.name {
        object.key("Name").string(var_15.as_str());
    }
    if let Some(var_16) = &input.tags {
        #[allow(unused_mut)]
        let mut object_17 = object.key("Tags").start_object();
        for (key_18, value_19) in var_16 {
             {
                object_17.key(key_18.as_str()).string(value_19.as_str());
            }
        }
        object_17.finish();
    }
    if let Some(var_20) = &input.voting_policy {
        #[allow(unused_mut)]
        let mut object_21 = object.key("VotingPolicy").start_object();
        crate::json_ser::serialize_structure_crate_model_voting_policy(&mut object_21, var_20)?;
        object_21.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_node_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateNodeInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_22) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_22.as_str());
    }
    if let Some(var_23) = &input.member_id {
        object.key("MemberId").string(var_23.as_str());
    }
    if let Some(var_24) = &input.node_configuration {
        #[allow(unused_mut)]
        let mut object_25 = object.key("NodeConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_node_configuration(&mut object_25, var_24)?;
        object_25.finish();
    }
    if let Some(var_26) = &input.tags {
        #[allow(unused_mut)]
        let mut object_27 = object.key("Tags").start_object();
        for (key_28, value_29) in var_26 {
             {
                object_27.key(key_28.as_str()).string(value_29.as_str());
            }
        }
        object_27.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_proposal_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateProposalInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_30) = &input.actions {
        #[allow(unused_mut)]
        let mut object_31 = object.key("Actions").start_object();
        crate::json_ser::serialize_structure_crate_model_proposal_actions(&mut object_31, var_30)?;
        object_31.finish();
    }
    if let Some(var_32) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_32.as_str());
    }
    if let Some(var_33) = &input.description {
        object.key("Description").string(var_33.as_str());
    }
    if let Some(var_34) = &input.member_id {
        object.key("MemberId").string(var_34.as_str());
    }
    if let Some(var_35) = &input.tags {
        #[allow(unused_mut)]
        let mut object_36 = object.key("Tags").start_object();
        for (key_37, value_38) in var_35 {
             {
                object_36.key(key_37.as_str()).string(value_38.as_str());
            }
        }
        object_36.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_39) = &input.tags {
        #[allow(unused_mut)]
        let mut object_40 = object.key("Tags").start_object();
        for (key_41, value_42) in var_39 {
             {
                object_40.key(key_41.as_str()).string(value_42.as_str());
            }
        }
        object_40.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_member_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateMemberInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_43) = &input.log_publishing_configuration {
        #[allow(unused_mut)]
        let mut object_44 = object.key("LogPublishingConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_member_log_publishing_configuration(&mut object_44, var_43)?;
        object_44.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_node_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateNodeInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_45) = &input.log_publishing_configuration {
        #[allow(unused_mut)]
        let mut object_46 = object.key("LogPublishingConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_node_log_publishing_configuration(&mut object_46, var_45)?;
        object_46.finish();
    }
    if let Some(var_47) = &input.member_id {
        object.key("MemberId").string(var_47.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_vote_on_proposal_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::VoteOnProposalInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_48) = &input.vote {
        object.key("Vote").string(var_48.as_str());
    }
    if let Some(var_49) = &input.voter_member_id {
        object.key("VoterMemberId").string(var_49.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_member_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::MemberConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_50) = &input.name {
        object.key("Name").string(var_50.as_str());
    }
    if let Some(var_51) = &input.description {
        object.key("Description").string(var_51.as_str());
    }
    if let Some(var_52) = &input.framework_configuration {
        #[allow(unused_mut)]
        let mut object_53 = object.key("FrameworkConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_member_framework_configuration(&mut object_53, var_52)?;
        object_53.finish();
    }
    if let Some(var_54) = &input.log_publishing_configuration {
        #[allow(unused_mut)]
        let mut object_55 = object.key("LogPublishingConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_member_log_publishing_configuration(&mut object_55, var_54)?;
        object_55.finish();
    }
    if let Some(var_56) = &input.tags {
        #[allow(unused_mut)]
        let mut object_57 = object.key("Tags").start_object();
        for (key_58, value_59) in var_56 {
             {
                object_57.key(key_58.as_str()).string(value_59.as_str());
            }
        }
        object_57.finish();
    }
    if let Some(var_60) = &input.kms_key_arn {
        object.key("KmsKeyArn").string(var_60.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_network_framework_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::NetworkFrameworkConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_61) = &input.fabric {
        #[allow(unused_mut)]
        let mut object_62 = object.key("Fabric").start_object();
        crate::json_ser::serialize_structure_crate_model_network_fabric_configuration(&mut object_62, var_61)?;
        object_62.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_voting_policy(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::VotingPolicy) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_63) = &input.approval_threshold_policy {
        #[allow(unused_mut)]
        let mut object_64 = object.key("ApprovalThresholdPolicy").start_object();
        crate::json_ser::serialize_structure_crate_model_approval_threshold_policy(&mut object_64, var_63)?;
        object_64.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_node_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::NodeConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_65) = &input.instance_type {
        object.key("InstanceType").string(var_65.as_str());
    }
    if let Some(var_66) = &input.availability_zone {
        object.key("AvailabilityZone").string(var_66.as_str());
    }
    if let Some(var_67) = &input.log_publishing_configuration {
        #[allow(unused_mut)]
        let mut object_68 = object.key("LogPublishingConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_node_log_publishing_configuration(&mut object_68, var_67)?;
        object_68.finish();
    }
    if let Some(var_69) = &input.state_db {
        object.key("StateDB").string(var_69.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_proposal_actions(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ProposalActions) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_70) = &input.invitations {
        let mut array_71 = object.key("Invitations").start_array();
        for item_72 in var_70 {
             {
                #[allow(unused_mut)]
                let mut object_73 = array_71.value().start_object();
                crate::json_ser::serialize_structure_crate_model_invite_action(&mut object_73, item_72)?;
                object_73.finish();
            }
        }
        array_71.finish();
    }
    if let Some(var_74) = &input.removals {
        let mut array_75 = object.key("Removals").start_array();
        for item_76 in var_74 {
             {
                #[allow(unused_mut)]
                let mut object_77 = array_75.value().start_object();
                crate::json_ser::serialize_structure_crate_model_remove_action(&mut object_77, item_76)?;
                object_77.finish();
            }
        }
        array_75.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_member_log_publishing_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::MemberLogPublishingConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_78) = &input.fabric {
        #[allow(unused_mut)]
        let mut object_79 = object.key("Fabric").start_object();
        crate::json_ser::serialize_structure_crate_model_member_fabric_log_publishing_configuration(&mut object_79, var_78)?;
        object_79.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_node_log_publishing_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::NodeLogPublishingConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_80) = &input.fabric {
        #[allow(unused_mut)]
        let mut object_81 = object.key("Fabric").start_object();
        crate::json_ser::serialize_structure_crate_model_node_fabric_log_publishing_configuration(&mut object_81, var_80)?;
        object_81.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_member_framework_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::MemberFrameworkConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_82) = &input.fabric {
        #[allow(unused_mut)]
        let mut object_83 = object.key("Fabric").start_object();
        crate::json_ser::serialize_structure_crate_model_member_fabric_configuration(&mut object_83, var_82)?;
        object_83.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_network_fabric_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::NetworkFabricConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_84) = &input.edition {
        object.key("Edition").string(var_84.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_approval_threshold_policy(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ApprovalThresholdPolicy) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_85) = &input.threshold_percentage {
        object.key("ThresholdPercentage").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_85).into()));
    }
    if let Some(var_86) = &input.proposal_duration_in_hours {
        object.key("ProposalDurationInHours").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_86).into()));
    }
    if let Some(var_87) = &input.threshold_comparator {
        object.key("ThresholdComparator").string(var_87.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_invite_action(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::InviteAction) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_88) = &input.principal {
        object.key("Principal").string(var_88.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_remove_action(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::RemoveAction) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_89) = &input.member_id {
        object.key("MemberId").string(var_89.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_member_fabric_log_publishing_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::MemberFabricLogPublishingConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_90) = &input.ca_logs {
        #[allow(unused_mut)]
        let mut object_91 = object.key("CaLogs").start_object();
        crate::json_ser::serialize_structure_crate_model_log_configurations(&mut object_91, var_90)?;
        object_91.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_node_fabric_log_publishing_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::NodeFabricLogPublishingConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_92) = &input.chaincode_logs {
        #[allow(unused_mut)]
        let mut object_93 = object.key("ChaincodeLogs").start_object();
        crate::json_ser::serialize_structure_crate_model_log_configurations(&mut object_93, var_92)?;
        object_93.finish();
    }
    if let Some(var_94) = &input.peer_logs {
        #[allow(unused_mut)]
        let mut object_95 = object.key("PeerLogs").start_object();
        crate::json_ser::serialize_structure_crate_model_log_configurations(&mut object_95, var_94)?;
        object_95.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_member_fabric_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::MemberFabricConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_96) = &input.admin_username {
        object.key("AdminUsername").string(var_96.as_str());
    }
    if let Some(var_97) = &input.admin_password {
        object.key("AdminPassword").string(var_97.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_log_configurations(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::LogConfigurations) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_98) = &input.cloudwatch {
        #[allow(unused_mut)]
        let mut object_99 = object.key("Cloudwatch").start_object();
        crate::json_ser::serialize_structure_crate_model_log_configuration(&mut object_99, var_98)?;
        object_99.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_log_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::LogConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_100) = &input.enabled {
        object.key("Enabled").boolean(*var_100);
    }
    Ok(())
}

