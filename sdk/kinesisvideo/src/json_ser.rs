// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_signaling_channel_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateSignalingChannelInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.channel_name {
        object.key("ChannelName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.channel_type {
        object.key("ChannelType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.single_master_configuration {
        #[allow(unused_mut)]
        let mut object_4 = object.key("SingleMasterConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_single_master_configuration(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.tags {
        let mut array_6 = object.key("Tags").start_array();
        for item_7 in var_5 {
             {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateStreamInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_9) = &input.data_retention_in_hours {
        object.key("DataRetentionInHours").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_9).into()));
    }
    if let Some(var_10) = &input.device_name {
        object.key("DeviceName").string(var_10.as_str());
    }
    if let Some(var_11) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_11.as_str());
    }
    if let Some(var_12) = &input.media_type {
        object.key("MediaType").string(var_12.as_str());
    }
    if let Some(var_13) = &input.stream_name {
        object.key("StreamName").string(var_13.as_str());
    }
    if let Some(var_14) = &input.tags {
        #[allow(unused_mut)]
        let mut object_15 = object.key("Tags").start_object();
        for (key_16, value_17) in var_14 {
             {
                object_15.key(key_16.as_str()).string(value_17.as_str());
            }
        }
        object_15.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_signaling_channel_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteSignalingChannelInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_18) = &input.channel_arn {
        object.key("ChannelARN").string(var_18.as_str());
    }
    if let Some(var_19) = &input.current_version {
        object.key("CurrentVersion").string(var_19.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteStreamInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_20) = &input.current_version {
        object.key("CurrentVersion").string(var_20.as_str());
    }
    if let Some(var_21) = &input.stream_arn {
        object.key("StreamARN").string(var_21.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_edge_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeEdgeConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_22) = &input.stream_arn {
        object.key("StreamARN").string(var_22.as_str());
    }
    if let Some(var_23) = &input.stream_name {
        object.key("StreamName").string(var_23.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_image_generation_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeImageGenerationConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_24) = &input.stream_arn {
        object.key("StreamARN").string(var_24.as_str());
    }
    if let Some(var_25) = &input.stream_name {
        object.key("StreamName").string(var_25.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_mapped_resource_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeMappedResourceConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_26) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_26).into()));
    }
    if let Some(var_27) = &input.next_token {
        object.key("NextToken").string(var_27.as_str());
    }
    if let Some(var_28) = &input.stream_arn {
        object.key("StreamARN").string(var_28.as_str());
    }
    if let Some(var_29) = &input.stream_name {
        object.key("StreamName").string(var_29.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_media_storage_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeMediaStorageConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_30) = &input.channel_arn {
        object.key("ChannelARN").string(var_30.as_str());
    }
    if let Some(var_31) = &input.channel_name {
        object.key("ChannelName").string(var_31.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_notification_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeNotificationConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_32) = &input.stream_arn {
        object.key("StreamARN").string(var_32.as_str());
    }
    if let Some(var_33) = &input.stream_name {
        object.key("StreamName").string(var_33.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_signaling_channel_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeSignalingChannelInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_34) = &input.channel_arn {
        object.key("ChannelARN").string(var_34.as_str());
    }
    if let Some(var_35) = &input.channel_name {
        object.key("ChannelName").string(var_35.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeStreamInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_36) = &input.stream_arn {
        object.key("StreamARN").string(var_36.as_str());
    }
    if let Some(var_37) = &input.stream_name {
        object.key("StreamName").string(var_37.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_data_endpoint_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetDataEndpointInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_38) = &input.api_name {
        object.key("APIName").string(var_38.as_str());
    }
    if let Some(var_39) = &input.stream_arn {
        object.key("StreamARN").string(var_39.as_str());
    }
    if let Some(var_40) = &input.stream_name {
        object.key("StreamName").string(var_40.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_signaling_channel_endpoint_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetSignalingChannelEndpointInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_41) = &input.channel_arn {
        object.key("ChannelARN").string(var_41.as_str());
    }
    if let Some(var_42) = &input.single_master_channel_endpoint_configuration {
        #[allow(unused_mut)]
        let mut object_43 = object.key("SingleMasterChannelEndpointConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_single_master_channel_endpoint_configuration(&mut object_43, var_42)?;
        object_43.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_signaling_channels_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListSignalingChannelsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_44) = &input.channel_name_condition {
        #[allow(unused_mut)]
        let mut object_45 = object.key("ChannelNameCondition").start_object();
        crate::json_ser::serialize_structure_crate_model_channel_name_condition(&mut object_45, var_44)?;
        object_45.finish();
    }
    if let Some(var_46) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_46).into()));
    }
    if let Some(var_47) = &input.next_token {
        object.key("NextToken").string(var_47.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_streams_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListStreamsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_48) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_48).into()));
    }
    if let Some(var_49) = &input.next_token {
        object.key("NextToken").string(var_49.as_str());
    }
    if let Some(var_50) = &input.stream_name_condition {
        #[allow(unused_mut)]
        let mut object_51 = object.key("StreamNameCondition").start_object();
        crate::json_ser::serialize_structure_crate_model_stream_name_condition(&mut object_51, var_50)?;
        object_51.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListTagsForResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_52) = &input.next_token {
        object.key("NextToken").string(var_52.as_str());
    }
    if let Some(var_53) = &input.resource_arn {
        object.key("ResourceARN").string(var_53.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListTagsForStreamInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_54) = &input.next_token {
        object.key("NextToken").string(var_54.as_str());
    }
    if let Some(var_55) = &input.stream_arn {
        object.key("StreamARN").string(var_55.as_str());
    }
    if let Some(var_56) = &input.stream_name {
        object.key("StreamName").string(var_56.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_edge_configuration_update_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartEdgeConfigurationUpdateInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_57) = &input.edge_config {
        #[allow(unused_mut)]
        let mut object_58 = object.key("EdgeConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_edge_config(&mut object_58, var_57)?;
        object_58.finish();
    }
    if let Some(var_59) = &input.stream_arn {
        object.key("StreamARN").string(var_59.as_str());
    }
    if let Some(var_60) = &input.stream_name {
        object.key("StreamName").string(var_60.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_61) = &input.resource_arn {
        object.key("ResourceARN").string(var_61.as_str());
    }
    if let Some(var_62) = &input.tags {
        let mut array_63 = object.key("Tags").start_array();
        for item_64 in var_62 {
             {
                #[allow(unused_mut)]
                let mut object_65 = array_63.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_65, item_64)?;
                object_65.finish();
            }
        }
        array_63.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagStreamInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_66) = &input.stream_arn {
        object.key("StreamARN").string(var_66.as_str());
    }
    if let Some(var_67) = &input.stream_name {
        object.key("StreamName").string(var_67.as_str());
    }
    if let Some(var_68) = &input.tags {
        #[allow(unused_mut)]
        let mut object_69 = object.key("Tags").start_object();
        for (key_70, value_71) in var_68 {
             {
                object_69.key(key_70.as_str()).string(value_71.as_str());
            }
        }
        object_69.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UntagResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_72) = &input.resource_arn {
        object.key("ResourceARN").string(var_72.as_str());
    }
    if let Some(var_73) = &input.tag_key_list {
        let mut array_74 = object.key("TagKeyList").start_array();
        for item_75 in var_73 {
             {
                array_74.value().string(item_75.as_str());
            }
        }
        array_74.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UntagStreamInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_76) = &input.stream_arn {
        object.key("StreamARN").string(var_76.as_str());
    }
    if let Some(var_77) = &input.stream_name {
        object.key("StreamName").string(var_77.as_str());
    }
    if let Some(var_78) = &input.tag_key_list {
        let mut array_79 = object.key("TagKeyList").start_array();
        for item_80 in var_78 {
             {
                array_79.value().string(item_80.as_str());
            }
        }
        array_79.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_data_retention_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateDataRetentionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_81) = &input.current_version {
        object.key("CurrentVersion").string(var_81.as_str());
    }
    if let Some(var_82) = &input.data_retention_change_in_hours {
        object.key("DataRetentionChangeInHours").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_82).into()));
    }
    if let Some(var_83) = &input.operation {
        object.key("Operation").string(var_83.as_str());
    }
    if let Some(var_84) = &input.stream_arn {
        object.key("StreamARN").string(var_84.as_str());
    }
    if let Some(var_85) = &input.stream_name {
        object.key("StreamName").string(var_85.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_image_generation_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateImageGenerationConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_86) = &input.image_generation_configuration {
        #[allow(unused_mut)]
        let mut object_87 = object.key("ImageGenerationConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_image_generation_configuration(&mut object_87, var_86)?;
        object_87.finish();
    }
    if let Some(var_88) = &input.stream_arn {
        object.key("StreamARN").string(var_88.as_str());
    }
    if let Some(var_89) = &input.stream_name {
        object.key("StreamName").string(var_89.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_media_storage_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateMediaStorageConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_90) = &input.channel_arn {
        object.key("ChannelARN").string(var_90.as_str());
    }
    if let Some(var_91) = &input.media_storage_configuration {
        #[allow(unused_mut)]
        let mut object_92 = object.key("MediaStorageConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_media_storage_configuration(&mut object_92, var_91)?;
        object_92.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_notification_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateNotificationConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_93) = &input.notification_configuration {
        #[allow(unused_mut)]
        let mut object_94 = object.key("NotificationConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_notification_configuration(&mut object_94, var_93)?;
        object_94.finish();
    }
    if let Some(var_95) = &input.stream_arn {
        object.key("StreamARN").string(var_95.as_str());
    }
    if let Some(var_96) = &input.stream_name {
        object.key("StreamName").string(var_96.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_signaling_channel_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateSignalingChannelInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_97) = &input.channel_arn {
        object.key("ChannelARN").string(var_97.as_str());
    }
    if let Some(var_98) = &input.current_version {
        object.key("CurrentVersion").string(var_98.as_str());
    }
    if let Some(var_99) = &input.single_master_configuration {
        #[allow(unused_mut)]
        let mut object_100 = object.key("SingleMasterConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_single_master_configuration(&mut object_100, var_99)?;
        object_100.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateStreamInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_101) = &input.current_version {
        object.key("CurrentVersion").string(var_101.as_str());
    }
    if let Some(var_102) = &input.device_name {
        object.key("DeviceName").string(var_102.as_str());
    }
    if let Some(var_103) = &input.media_type {
        object.key("MediaType").string(var_103.as_str());
    }
    if let Some(var_104) = &input.stream_arn {
        object.key("StreamARN").string(var_104.as_str());
    }
    if let Some(var_105) = &input.stream_name {
        object.key("StreamName").string(var_105.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_single_master_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SingleMasterConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_106) = &input.message_ttl_seconds {
        object.key("MessageTtlSeconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_106).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Tag) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_107) = &input.key {
        object.key("Key").string(var_107.as_str());
    }
    if let Some(var_108) = &input.value {
        object.key("Value").string(var_108.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_single_master_channel_endpoint_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SingleMasterChannelEndpointConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_109) = &input.protocols {
        let mut array_110 = object.key("Protocols").start_array();
        for item_111 in var_109 {
             {
                array_110.value().string(item_111.as_str());
            }
        }
        array_110.finish();
    }
    if let Some(var_112) = &input.role {
        object.key("Role").string(var_112.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_channel_name_condition(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ChannelNameCondition) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_113) = &input.comparison_operator {
        object.key("ComparisonOperator").string(var_113.as_str());
    }
    if let Some(var_114) = &input.comparison_value {
        object.key("ComparisonValue").string(var_114.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_stream_name_condition(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::StreamNameCondition) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_115) = &input.comparison_operator {
        object.key("ComparisonOperator").string(var_115.as_str());
    }
    if let Some(var_116) = &input.comparison_value {
        object.key("ComparisonValue").string(var_116.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_edge_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::EdgeConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_117) = &input.hub_device_arn {
        object.key("HubDeviceArn").string(var_117.as_str());
    }
    if let Some(var_118) = &input.recorder_config {
        #[allow(unused_mut)]
        let mut object_119 = object.key("RecorderConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_recorder_config(&mut object_119, var_118)?;
        object_119.finish();
    }
    if let Some(var_120) = &input.uploader_config {
        #[allow(unused_mut)]
        let mut object_121 = object.key("UploaderConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_uploader_config(&mut object_121, var_120)?;
        object_121.finish();
    }
    if let Some(var_122) = &input.deletion_config {
        #[allow(unused_mut)]
        let mut object_123 = object.key("DeletionConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_deletion_config(&mut object_123, var_122)?;
        object_123.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_image_generation_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ImageGenerationConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_124) = &input.status {
        object.key("Status").string(var_124.as_str());
    }
    if let Some(var_125) = &input.image_selector_type {
        object.key("ImageSelectorType").string(var_125.as_str());
    }
    if let Some(var_126) = &input.destination_config {
        #[allow(unused_mut)]
        let mut object_127 = object.key("DestinationConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_image_generation_destination_config(&mut object_127, var_126)?;
        object_127.finish();
    }
    if let Some(var_128) = &input.sampling_interval {
        object.key("SamplingInterval").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_128).into()));
    }
    if let Some(var_129) = &input.format {
        object.key("Format").string(var_129.as_str());
    }
    if let Some(var_130) = &input.format_config {
        #[allow(unused_mut)]
        let mut object_131 = object.key("FormatConfig").start_object();
        for (key_132, value_133) in var_130 {
             {
                object_131.key(key_132.as_str()).string(value_133.as_str());
            }
        }
        object_131.finish();
    }
    if let Some(var_134) = &input.width_pixels {
        object.key("WidthPixels").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_134).into()));
    }
    if let Some(var_135) = &input.height_pixels {
        object.key("HeightPixels").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_135).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_model_media_storage_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::MediaStorageConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_136) = &input.stream_arn {
        object.key("StreamARN").string(var_136.as_str());
    }
    if let Some(var_137) = &input.status {
        object.key("Status").string(var_137.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_notification_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::NotificationConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_138) = &input.status {
        object.key("Status").string(var_138.as_str());
    }
    if let Some(var_139) = &input.destination_config {
        #[allow(unused_mut)]
        let mut object_140 = object.key("DestinationConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_notification_destination_config(&mut object_140, var_139)?;
        object_140.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_recorder_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::RecorderConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_141) = &input.media_source_config {
        #[allow(unused_mut)]
        let mut object_142 = object.key("MediaSourceConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_media_source_config(&mut object_142, var_141)?;
        object_142.finish();
    }
    if let Some(var_143) = &input.schedule_config {
        #[allow(unused_mut)]
        let mut object_144 = object.key("ScheduleConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_schedule_config(&mut object_144, var_143)?;
        object_144.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_uploader_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::UploaderConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_145) = &input.schedule_config {
        #[allow(unused_mut)]
        let mut object_146 = object.key("ScheduleConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_schedule_config(&mut object_146, var_145)?;
        object_146.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_deletion_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DeletionConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_147) = &input.edge_retention_in_hours {
        object.key("EdgeRetentionInHours").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_147).into()));
    }
    if let Some(var_148) = &input.local_size_config {
        #[allow(unused_mut)]
        let mut object_149 = object.key("LocalSizeConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_local_size_config(&mut object_149, var_148)?;
        object_149.finish();
    }
    if let Some(var_150) = &input.delete_after_upload {
        object.key("DeleteAfterUpload").boolean(*var_150);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_image_generation_destination_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ImageGenerationDestinationConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_151) = &input.uri {
        object.key("Uri").string(var_151.as_str());
    }
    if let Some(var_152) = &input.destination_region {
        object.key("DestinationRegion").string(var_152.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_notification_destination_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::NotificationDestinationConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_153) = &input.uri {
        object.key("Uri").string(var_153.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_media_source_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::MediaSourceConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_154) = &input.media_uri_secret_arn {
        object.key("MediaUriSecretArn").string(var_154.as_str());
    }
    if let Some(var_155) = &input.media_uri_type {
        object.key("MediaUriType").string(var_155.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_schedule_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ScheduleConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_156) = &input.schedule_expression {
        object.key("ScheduleExpression").string(var_156.as_str());
    }
    if let Some(var_157) = &input.duration_in_seconds {
        object.key("DurationInSeconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_157).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_model_local_size_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::LocalSizeConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_158) = &input.max_local_media_size_in_mb {
        object.key("MaxLocalMediaSizeInMB").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_158).into()));
    }
    if let Some(var_159) = &input.strategy_on_full_size {
        object.key("StrategyOnFullSize").string(var_159.as_str());
    }
    Ok(())
}

