// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_accept_eulas_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AcceptEulasInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.eula_ids {
        let mut array_2 = object.key("eulaIds").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_launch_profile_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateLaunchProfileInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_4) = &input.description {
        object.key("description").string(var_4.as_str());
    }
    if let Some(var_5) = &input.ec2_subnet_ids {
        let mut array_6 = object.key("ec2SubnetIds").start_array();
        for item_7 in var_5 {
             {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.launch_profile_protocol_versions {
        let mut array_9 = object.key("launchProfileProtocolVersions").start_array();
        for item_10 in var_8 {
             {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
    if let Some(var_11) = &input.name {
        object.key("name").string(var_11.as_str());
    }
    if let Some(var_12) = &input.stream_configuration {
        #[allow(unused_mut)]
        let mut object_13 = object.key("streamConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_stream_configuration_create(&mut object_13, var_12)?;
        object_13.finish();
    }
    if let Some(var_14) = &input.studio_component_ids {
        let mut array_15 = object.key("studioComponentIds").start_array();
        for item_16 in var_14 {
             {
                array_15.value().string(item_16.as_str());
            }
        }
        array_15.finish();
    }
    if let Some(var_17) = &input.tags {
        #[allow(unused_mut)]
        let mut object_18 = object.key("tags").start_object();
        for (key_19, value_20) in var_17 {
             {
                object_18.key(key_19.as_str()).string(value_20.as_str());
            }
        }
        object_18.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_streaming_image_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateStreamingImageInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_21) = &input.description {
        object.key("description").string(var_21.as_str());
    }
    if let Some(var_22) = &input.ec2_image_id {
        object.key("ec2ImageId").string(var_22.as_str());
    }
    if let Some(var_23) = &input.name {
        object.key("name").string(var_23.as_str());
    }
    if let Some(var_24) = &input.tags {
        #[allow(unused_mut)]
        let mut object_25 = object.key("tags").start_object();
        for (key_26, value_27) in var_24 {
             {
                object_25.key(key_26.as_str()).string(value_27.as_str());
            }
        }
        object_25.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_streaming_session_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateStreamingSessionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_28) = &input.ec2_instance_type {
        object.key("ec2InstanceType").string(var_28.as_str());
    }
    if let Some(var_29) = &input.launch_profile_id {
        object.key("launchProfileId").string(var_29.as_str());
    }
    if let Some(var_30) = &input.owned_by {
        object.key("ownedBy").string(var_30.as_str());
    }
    if let Some(var_31) = &input.streaming_image_id {
        object.key("streamingImageId").string(var_31.as_str());
    }
    if let Some(var_32) = &input.tags {
        #[allow(unused_mut)]
        let mut object_33 = object.key("tags").start_object();
        for (key_34, value_35) in var_32 {
             {
                object_33.key(key_34.as_str()).string(value_35.as_str());
            }
        }
        object_33.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_streaming_session_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateStreamingSessionStreamInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_36) = &input.expiration_in_seconds {
        object.key("expirationInSeconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_36).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_studio_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateStudioInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_37) = &input.admin_role_arn {
        object.key("adminRoleArn").string(var_37.as_str());
    }
    if let Some(var_38) = &input.display_name {
        object.key("displayName").string(var_38.as_str());
    }
    if let Some(var_39) = &input.studio_encryption_configuration {
        #[allow(unused_mut)]
        let mut object_40 = object.key("studioEncryptionConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_studio_encryption_configuration(&mut object_40, var_39)?;
        object_40.finish();
    }
    if let Some(var_41) = &input.studio_name {
        object.key("studioName").string(var_41.as_str());
    }
    if let Some(var_42) = &input.tags {
        #[allow(unused_mut)]
        let mut object_43 = object.key("tags").start_object();
        for (key_44, value_45) in var_42 {
             {
                object_43.key(key_44.as_str()).string(value_45.as_str());
            }
        }
        object_43.finish();
    }
    if let Some(var_46) = &input.user_role_arn {
        object.key("userRoleArn").string(var_46.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_studio_component_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateStudioComponentInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_47) = &input.configuration {
        #[allow(unused_mut)]
        let mut object_48 = object.key("configuration").start_object();
        crate::json_ser::serialize_union_crate_model_studio_component_configuration(&mut object_48, var_47)?;
        object_48.finish();
    }
    if let Some(var_49) = &input.description {
        object.key("description").string(var_49.as_str());
    }
    if let Some(var_50) = &input.ec2_security_group_ids {
        let mut array_51 = object.key("ec2SecurityGroupIds").start_array();
        for item_52 in var_50 {
             {
                array_51.value().string(item_52.as_str());
            }
        }
        array_51.finish();
    }
    if let Some(var_53) = &input.initialization_scripts {
        let mut array_54 = object.key("initializationScripts").start_array();
        for item_55 in var_53 {
             {
                #[allow(unused_mut)]
                let mut object_56 = array_54.value().start_object();
                crate::json_ser::serialize_structure_crate_model_studio_component_initialization_script(&mut object_56, item_55)?;
                object_56.finish();
            }
        }
        array_54.finish();
    }
    if let Some(var_57) = &input.name {
        object.key("name").string(var_57.as_str());
    }
    if let Some(var_58) = &input.runtime_role_arn {
        object.key("runtimeRoleArn").string(var_58.as_str());
    }
    if let Some(var_59) = &input.script_parameters {
        let mut array_60 = object.key("scriptParameters").start_array();
        for item_61 in var_59 {
             {
                #[allow(unused_mut)]
                let mut object_62 = array_60.value().start_object();
                crate::json_ser::serialize_structure_crate_model_script_parameter_key_value(&mut object_62, item_61)?;
                object_62.finish();
            }
        }
        array_60.finish();
    }
    if let Some(var_63) = &input.secure_initialization_role_arn {
        object.key("secureInitializationRoleArn").string(var_63.as_str());
    }
    if let Some(var_64) = &input.subtype {
        object.key("subtype").string(var_64.as_str());
    }
    if let Some(var_65) = &input.tags {
        #[allow(unused_mut)]
        let mut object_66 = object.key("tags").start_object();
        for (key_67, value_68) in var_65 {
             {
                object_66.key(key_67.as_str()).string(value_68.as_str());
            }
        }
        object_66.finish();
    }
    if let Some(var_69) = &input.r#type {
        object.key("type").string(var_69.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_launch_profile_members_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutLaunchProfileMembersInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_70) = &input.identity_store_id {
        object.key("identityStoreId").string(var_70.as_str());
    }
    if let Some(var_71) = &input.members {
        let mut array_72 = object.key("members").start_array();
        for item_73 in var_71 {
             {
                #[allow(unused_mut)]
                let mut object_74 = array_72.value().start_object();
                crate::json_ser::serialize_structure_crate_model_new_launch_profile_member(&mut object_74, item_73)?;
                object_74.finish();
            }
        }
        array_72.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_studio_members_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutStudioMembersInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_75) = &input.identity_store_id {
        object.key("identityStoreId").string(var_75.as_str());
    }
    if let Some(var_76) = &input.members {
        let mut array_77 = object.key("members").start_array();
        for item_78 in var_76 {
             {
                #[allow(unused_mut)]
                let mut object_79 = array_77.value().start_object();
                crate::json_ser::serialize_structure_crate_model_new_studio_member(&mut object_79, item_78)?;
                object_79.finish();
            }
        }
        array_77.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_streaming_session_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartStreamingSessionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_80) = &input.backup_id {
        object.key("backupId").string(var_80.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_stop_streaming_session_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StopStreamingSessionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_81) = &input.volume_retention_mode {
        object.key("volumeRetentionMode").string(var_81.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_82) = &input.tags {
        #[allow(unused_mut)]
        let mut object_83 = object.key("tags").start_object();
        for (key_84, value_85) in var_82 {
             {
                object_83.key(key_84.as_str()).string(value_85.as_str());
            }
        }
        object_83.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_launch_profile_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateLaunchProfileInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_86) = &input.description {
        object.key("description").string(var_86.as_str());
    }
    if let Some(var_87) = &input.launch_profile_protocol_versions {
        let mut array_88 = object.key("launchProfileProtocolVersions").start_array();
        for item_89 in var_87 {
             {
                array_88.value().string(item_89.as_str());
            }
        }
        array_88.finish();
    }
    if let Some(var_90) = &input.name {
        object.key("name").string(var_90.as_str());
    }
    if let Some(var_91) = &input.stream_configuration {
        #[allow(unused_mut)]
        let mut object_92 = object.key("streamConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_stream_configuration_create(&mut object_92, var_91)?;
        object_92.finish();
    }
    if let Some(var_93) = &input.studio_component_ids {
        let mut array_94 = object.key("studioComponentIds").start_array();
        for item_95 in var_93 {
             {
                array_94.value().string(item_95.as_str());
            }
        }
        array_94.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_launch_profile_member_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateLaunchProfileMemberInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_96) = &input.persona {
        object.key("persona").string(var_96.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_streaming_image_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateStreamingImageInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_97) = &input.description {
        object.key("description").string(var_97.as_str());
    }
    if let Some(var_98) = &input.name {
        object.key("name").string(var_98.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_studio_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateStudioInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_99) = &input.admin_role_arn {
        object.key("adminRoleArn").string(var_99.as_str());
    }
    if let Some(var_100) = &input.display_name {
        object.key("displayName").string(var_100.as_str());
    }
    if let Some(var_101) = &input.user_role_arn {
        object.key("userRoleArn").string(var_101.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_studio_component_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateStudioComponentInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_102) = &input.configuration {
        #[allow(unused_mut)]
        let mut object_103 = object.key("configuration").start_object();
        crate::json_ser::serialize_union_crate_model_studio_component_configuration(&mut object_103, var_102)?;
        object_103.finish();
    }
    if let Some(var_104) = &input.description {
        object.key("description").string(var_104.as_str());
    }
    if let Some(var_105) = &input.ec2_security_group_ids {
        let mut array_106 = object.key("ec2SecurityGroupIds").start_array();
        for item_107 in var_105 {
             {
                array_106.value().string(item_107.as_str());
            }
        }
        array_106.finish();
    }
    if let Some(var_108) = &input.initialization_scripts {
        let mut array_109 = object.key("initializationScripts").start_array();
        for item_110 in var_108 {
             {
                #[allow(unused_mut)]
                let mut object_111 = array_109.value().start_object();
                crate::json_ser::serialize_structure_crate_model_studio_component_initialization_script(&mut object_111, item_110)?;
                object_111.finish();
            }
        }
        array_109.finish();
    }
    if let Some(var_112) = &input.name {
        object.key("name").string(var_112.as_str());
    }
    if let Some(var_113) = &input.runtime_role_arn {
        object.key("runtimeRoleArn").string(var_113.as_str());
    }
    if let Some(var_114) = &input.script_parameters {
        let mut array_115 = object.key("scriptParameters").start_array();
        for item_116 in var_114 {
             {
                #[allow(unused_mut)]
                let mut object_117 = array_115.value().start_object();
                crate::json_ser::serialize_structure_crate_model_script_parameter_key_value(&mut object_117, item_116)?;
                object_117.finish();
            }
        }
        array_115.finish();
    }
    if let Some(var_118) = &input.secure_initialization_role_arn {
        object.key("secureInitializationRoleArn").string(var_118.as_str());
    }
    if let Some(var_119) = &input.subtype {
        object.key("subtype").string(var_119.as_str());
    }
    if let Some(var_120) = &input.r#type {
        object.key("type").string(var_120.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_stream_configuration_create(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::StreamConfigurationCreate) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_121) = &input.clipboard_mode {
        object.key("clipboardMode").string(var_121.as_str());
    }
    if let Some(var_122) = &input.ec2_instance_types {
        let mut array_123 = object.key("ec2InstanceTypes").start_array();
        for item_124 in var_122 {
             {
                array_123.value().string(item_124.as_str());
            }
        }
        array_123.finish();
    }
    if let Some(var_125) = &input.max_session_length_in_minutes {
        object.key("maxSessionLengthInMinutes").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_125).into()));
    }
    if let Some(var_126) = &input.streaming_image_ids {
        let mut array_127 = object.key("streamingImageIds").start_array();
        for item_128 in var_126 {
             {
                array_127.value().string(item_128.as_str());
            }
        }
        array_127.finish();
    }
    if input.max_stopped_session_length_in_minutes != 0 {
        object.key("maxStoppedSessionLengthInMinutes").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_stopped_session_length_in_minutes).into()));
    }
    if let Some(var_129) = &input.session_storage {
        #[allow(unused_mut)]
        let mut object_130 = object.key("sessionStorage").start_object();
        crate::json_ser::serialize_structure_crate_model_stream_configuration_session_storage(&mut object_130, var_129)?;
        object_130.finish();
    }
    if let Some(var_131) = &input.session_backup {
        #[allow(unused_mut)]
        let mut object_132 = object.key("sessionBackup").start_object();
        crate::json_ser::serialize_structure_crate_model_stream_configuration_session_backup(&mut object_132, var_131)?;
        object_132.finish();
    }
    if let Some(var_133) = &input.session_persistence_mode {
        object.key("sessionPersistenceMode").string(var_133.as_str());
    }
    if let Some(var_134) = &input.volume_configuration {
        #[allow(unused_mut)]
        let mut object_135 = object.key("volumeConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_volume_configuration(&mut object_135, var_134)?;
        object_135.finish();
    }
    if let Some(var_136) = &input.automatic_termination_mode {
        object.key("automaticTerminationMode").string(var_136.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_studio_encryption_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::StudioEncryptionConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_137) = &input.key_arn {
        object.key("keyArn").string(var_137.as_str());
    }
    if let Some(var_138) = &input.key_type {
        object.key("keyType").string(var_138.as_str());
    }
    Ok(())
}

pub fn serialize_union_crate_model_studio_component_configuration(object_48: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::StudioComponentConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::model::StudioComponentConfiguration::ActiveDirectoryConfiguration(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_139 = object_48.key("activeDirectoryConfiguration").start_object();
                crate::json_ser::serialize_structure_crate_model_active_directory_configuration(&mut object_139, inner)?;
                object_139.finish();
            }
        },
        crate::model::StudioComponentConfiguration::ComputeFarmConfiguration(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_140 = object_48.key("computeFarmConfiguration").start_object();
                crate::json_ser::serialize_structure_crate_model_compute_farm_configuration(&mut object_140, inner)?;
                object_140.finish();
            }
        },
        crate::model::StudioComponentConfiguration::LicenseServiceConfiguration(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_141 = object_48.key("licenseServiceConfiguration").start_object();
                crate::json_ser::serialize_structure_crate_model_license_service_configuration(&mut object_141, inner)?;
                object_141.finish();
            }
        },
        crate::model::StudioComponentConfiguration::SharedFileSystemConfiguration(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_142 = object_48.key("sharedFileSystemConfiguration").start_object();
                crate::json_ser::serialize_structure_crate_model_shared_file_system_configuration(&mut object_142, inner)?;
                object_142.finish();
            }
        },
        crate::model::StudioComponentConfiguration::Unknown => return Err(aws_smithy_http::operation::error::SerializationError::unknown_variant("StudioComponentConfiguration"))
    }
    Ok(())
}

pub fn serialize_structure_crate_model_studio_component_initialization_script(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::StudioComponentInitializationScript) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_143) = &input.launch_profile_protocol_version {
        object.key("launchProfileProtocolVersion").string(var_143.as_str());
    }
    if let Some(var_144) = &input.platform {
        object.key("platform").string(var_144.as_str());
    }
    if let Some(var_145) = &input.run_context {
        object.key("runContext").string(var_145.as_str());
    }
    if let Some(var_146) = &input.script {
        object.key("script").string(var_146.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_script_parameter_key_value(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ScriptParameterKeyValue) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_147) = &input.key {
        object.key("key").string(var_147.as_str());
    }
    if let Some(var_148) = &input.value {
        object.key("value").string(var_148.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_new_launch_profile_member(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::NewLaunchProfileMember) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_149) = &input.persona {
        object.key("persona").string(var_149.as_str());
    }
    if let Some(var_150) = &input.principal_id {
        object.key("principalId").string(var_150.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_new_studio_member(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::NewStudioMember) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_151) = &input.persona {
        object.key("persona").string(var_151.as_str());
    }
    if let Some(var_152) = &input.principal_id {
        object.key("principalId").string(var_152.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_stream_configuration_session_storage(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::StreamConfigurationSessionStorage) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_153) = &input.root {
        #[allow(unused_mut)]
        let mut object_154 = object.key("root").start_object();
        crate::json_ser::serialize_structure_crate_model_streaming_session_storage_root(&mut object_154, var_153)?;
        object_154.finish();
    }
    if let Some(var_155) = &input.mode {
        let mut array_156 = object.key("mode").start_array();
        for item_157 in var_155 {
             {
                array_156.value().string(item_157.as_str());
            }
        }
        array_156.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_stream_configuration_session_backup(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::StreamConfigurationSessionBackup) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_158) = &input.mode {
        object.key("mode").string(var_158.as_str());
    }
    if input.max_backups_to_retain != 0 {
        object.key("maxBackupsToRetain").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_backups_to_retain).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_model_volume_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::VolumeConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_159) = &input.size {
        object.key("size").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_159).into()));
    }
    if let Some(var_160) = &input.throughput {
        object.key("throughput").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_160).into()));
    }
    if let Some(var_161) = &input.iops {
        object.key("iops").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_161).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_model_active_directory_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ActiveDirectoryConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_162) = &input.computer_attributes {
        let mut array_163 = object.key("computerAttributes").start_array();
        for item_164 in var_162 {
             {
                #[allow(unused_mut)]
                let mut object_165 = array_163.value().start_object();
                crate::json_ser::serialize_structure_crate_model_active_directory_computer_attribute(&mut object_165, item_164)?;
                object_165.finish();
            }
        }
        array_163.finish();
    }
    if let Some(var_166) = &input.directory_id {
        object.key("directoryId").string(var_166.as_str());
    }
    if let Some(var_167) = &input.organizational_unit_distinguished_name {
        object.key("organizationalUnitDistinguishedName").string(var_167.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_compute_farm_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ComputeFarmConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_168) = &input.active_directory_user {
        object.key("activeDirectoryUser").string(var_168.as_str());
    }
    if let Some(var_169) = &input.endpoint {
        object.key("endpoint").string(var_169.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_license_service_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::LicenseServiceConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_170) = &input.endpoint {
        object.key("endpoint").string(var_170.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_shared_file_system_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SharedFileSystemConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_171) = &input.endpoint {
        object.key("endpoint").string(var_171.as_str());
    }
    if let Some(var_172) = &input.file_system_id {
        object.key("fileSystemId").string(var_172.as_str());
    }
    if let Some(var_173) = &input.linux_mount_point {
        object.key("linuxMountPoint").string(var_173.as_str());
    }
    if let Some(var_174) = &input.share_name {
        object.key("shareName").string(var_174.as_str());
    }
    if let Some(var_175) = &input.windows_mount_drive {
        object.key("windowsMountDrive").string(var_175.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_streaming_session_storage_root(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::StreamingSessionStorageRoot) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_176) = &input.linux {
        object.key("linux").string(var_176.as_str());
    }
    if let Some(var_177) = &input.windows {
        object.key("windows").string(var_177.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_active_directory_computer_attribute(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ActiveDirectoryComputerAttribute) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_178) = &input.name {
        object.key("name").string(var_178.as_str());
    }
    if let Some(var_179) = &input.value {
        object.key("value").string(var_179.as_str());
    }
    Ok(())
}

