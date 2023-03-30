// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_add_tags_to_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AddTagsToStreamInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.stream_name {
        object.key("StreamName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.tags {
        #[allow(unused_mut)]
        let mut object_3 = object.key("Tags").start_object();
        for (key_4, value_5) in var_2 {
             {
                object_3.key(key_4.as_str()).string(value_5.as_str());
            }
        }
        object_3.finish();
    }
    if let Some(var_6) = &input.stream_arn {
        object.key("StreamARN").string(var_6.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateStreamInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_7) = &input.stream_name {
        object.key("StreamName").string(var_7.as_str());
    }
    if let Some(var_8) = &input.shard_count {
        object.key("ShardCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_8).into()));
    }
    if let Some(var_9) = &input.stream_mode_details {
        #[allow(unused_mut)]
        let mut object_10 = object.key("StreamModeDetails").start_object();
        crate::json_ser::serialize_structure_crate_model_stream_mode_details(&mut object_10, var_9)?;
        object_10.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_decrease_stream_retention_period_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DecreaseStreamRetentionPeriodInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_11) = &input.stream_name {
        object.key("StreamName").string(var_11.as_str());
    }
    if let Some(var_12) = &input.retention_period_hours {
        object.key("RetentionPeriodHours").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_12).into()));
    }
    if let Some(var_13) = &input.stream_arn {
        object.key("StreamARN").string(var_13.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteStreamInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_14) = &input.stream_name {
        object.key("StreamName").string(var_14.as_str());
    }
    if let Some(var_15) = &input.enforce_consumer_deletion {
        object.key("EnforceConsumerDeletion").boolean(*var_15);
    }
    if let Some(var_16) = &input.stream_arn {
        object.key("StreamARN").string(var_16.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_deregister_stream_consumer_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeregisterStreamConsumerInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_17) = &input.stream_arn {
        object.key("StreamARN").string(var_17.as_str());
    }
    if let Some(var_18) = &input.consumer_name {
        object.key("ConsumerName").string(var_18.as_str());
    }
    if let Some(var_19) = &input.consumer_arn {
        object.key("ConsumerARN").string(var_19.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeStreamInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_20) = &input.stream_name {
        object.key("StreamName").string(var_20.as_str());
    }
    if let Some(var_21) = &input.limit {
        object.key("Limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_21).into()));
    }
    if let Some(var_22) = &input.exclusive_start_shard_id {
        object.key("ExclusiveStartShardId").string(var_22.as_str());
    }
    if let Some(var_23) = &input.stream_arn {
        object.key("StreamARN").string(var_23.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_stream_consumer_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeStreamConsumerInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_24) = &input.stream_arn {
        object.key("StreamARN").string(var_24.as_str());
    }
    if let Some(var_25) = &input.consumer_name {
        object.key("ConsumerName").string(var_25.as_str());
    }
    if let Some(var_26) = &input.consumer_arn {
        object.key("ConsumerARN").string(var_26.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_stream_summary_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeStreamSummaryInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_27) = &input.stream_name {
        object.key("StreamName").string(var_27.as_str());
    }
    if let Some(var_28) = &input.stream_arn {
        object.key("StreamARN").string(var_28.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_disable_enhanced_monitoring_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DisableEnhancedMonitoringInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_29) = &input.stream_name {
        object.key("StreamName").string(var_29.as_str());
    }
    if let Some(var_30) = &input.shard_level_metrics {
        let mut array_31 = object.key("ShardLevelMetrics").start_array();
        for item_32 in var_30 {
             {
                array_31.value().string(item_32.as_str());
            }
        }
        array_31.finish();
    }
    if let Some(var_33) = &input.stream_arn {
        object.key("StreamARN").string(var_33.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_enable_enhanced_monitoring_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::EnableEnhancedMonitoringInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_34) = &input.stream_name {
        object.key("StreamName").string(var_34.as_str());
    }
    if let Some(var_35) = &input.shard_level_metrics {
        let mut array_36 = object.key("ShardLevelMetrics").start_array();
        for item_37 in var_35 {
             {
                array_36.value().string(item_37.as_str());
            }
        }
        array_36.finish();
    }
    if let Some(var_38) = &input.stream_arn {
        object.key("StreamARN").string(var_38.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_records_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetRecordsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_39) = &input.shard_iterator {
        object.key("ShardIterator").string(var_39.as_str());
    }
    if let Some(var_40) = &input.limit {
        object.key("Limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_40).into()));
    }
    if let Some(var_41) = &input.stream_arn {
        object.key("StreamARN").string(var_41.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_shard_iterator_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetShardIteratorInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_42) = &input.stream_name {
        object.key("StreamName").string(var_42.as_str());
    }
    if let Some(var_43) = &input.shard_id {
        object.key("ShardId").string(var_43.as_str());
    }
    if let Some(var_44) = &input.shard_iterator_type {
        object.key("ShardIteratorType").string(var_44.as_str());
    }
    if let Some(var_45) = &input.starting_sequence_number {
        object.key("StartingSequenceNumber").string(var_45.as_str());
    }
    if let Some(var_46) = &input.timestamp {
        object.key("Timestamp").date_time(var_46, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_47) = &input.stream_arn {
        object.key("StreamARN").string(var_47.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_increase_stream_retention_period_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::IncreaseStreamRetentionPeriodInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_48) = &input.stream_name {
        object.key("StreamName").string(var_48.as_str());
    }
    if let Some(var_49) = &input.retention_period_hours {
        object.key("RetentionPeriodHours").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_49).into()));
    }
    if let Some(var_50) = &input.stream_arn {
        object.key("StreamARN").string(var_50.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_shards_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListShardsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_51) = &input.stream_name {
        object.key("StreamName").string(var_51.as_str());
    }
    if let Some(var_52) = &input.next_token {
        object.key("NextToken").string(var_52.as_str());
    }
    if let Some(var_53) = &input.exclusive_start_shard_id {
        object.key("ExclusiveStartShardId").string(var_53.as_str());
    }
    if let Some(var_54) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_54).into()));
    }
    if let Some(var_55) = &input.stream_creation_timestamp {
        object.key("StreamCreationTimestamp").date_time(var_55, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_56) = &input.shard_filter {
        #[allow(unused_mut)]
        let mut object_57 = object.key("ShardFilter").start_object();
        crate::json_ser::serialize_structure_crate_model_shard_filter(&mut object_57, var_56)?;
        object_57.finish();
    }
    if let Some(var_58) = &input.stream_arn {
        object.key("StreamARN").string(var_58.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_stream_consumers_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListStreamConsumersInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_59) = &input.stream_arn {
        object.key("StreamARN").string(var_59.as_str());
    }
    if let Some(var_60) = &input.next_token {
        object.key("NextToken").string(var_60.as_str());
    }
    if let Some(var_61) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_61).into()));
    }
    if let Some(var_62) = &input.stream_creation_timestamp {
        object.key("StreamCreationTimestamp").date_time(var_62, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_streams_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListStreamsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_63) = &input.limit {
        object.key("Limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_63).into()));
    }
    if let Some(var_64) = &input.exclusive_start_stream_name {
        object.key("ExclusiveStartStreamName").string(var_64.as_str());
    }
    if let Some(var_65) = &input.next_token {
        object.key("NextToken").string(var_65.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListTagsForStreamInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_66) = &input.stream_name {
        object.key("StreamName").string(var_66.as_str());
    }
    if let Some(var_67) = &input.exclusive_start_tag_key {
        object.key("ExclusiveStartTagKey").string(var_67.as_str());
    }
    if let Some(var_68) = &input.limit {
        object.key("Limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_68).into()));
    }
    if let Some(var_69) = &input.stream_arn {
        object.key("StreamARN").string(var_69.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_merge_shards_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::MergeShardsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_70) = &input.stream_name {
        object.key("StreamName").string(var_70.as_str());
    }
    if let Some(var_71) = &input.shard_to_merge {
        object.key("ShardToMerge").string(var_71.as_str());
    }
    if let Some(var_72) = &input.adjacent_shard_to_merge {
        object.key("AdjacentShardToMerge").string(var_72.as_str());
    }
    if let Some(var_73) = &input.stream_arn {
        object.key("StreamARN").string(var_73.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_record_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutRecordInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_74) = &input.stream_name {
        object.key("StreamName").string(var_74.as_str());
    }
    if let Some(var_75) = &input.data {
        object.key("Data").string_unchecked(&aws_smithy_types::base64::encode(var_75));
    }
    if let Some(var_76) = &input.partition_key {
        object.key("PartitionKey").string(var_76.as_str());
    }
    if let Some(var_77) = &input.explicit_hash_key {
        object.key("ExplicitHashKey").string(var_77.as_str());
    }
    if let Some(var_78) = &input.sequence_number_for_ordering {
        object.key("SequenceNumberForOrdering").string(var_78.as_str());
    }
    if let Some(var_79) = &input.stream_arn {
        object.key("StreamARN").string(var_79.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_records_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutRecordsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_80) = &input.records {
        let mut array_81 = object.key("Records").start_array();
        for item_82 in var_80 {
             {
                #[allow(unused_mut)]
                let mut object_83 = array_81.value().start_object();
                crate::json_ser::serialize_structure_crate_model_put_records_request_entry(&mut object_83, item_82)?;
                object_83.finish();
            }
        }
        array_81.finish();
    }
    if let Some(var_84) = &input.stream_name {
        object.key("StreamName").string(var_84.as_str());
    }
    if let Some(var_85) = &input.stream_arn {
        object.key("StreamARN").string(var_85.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_register_stream_consumer_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::RegisterStreamConsumerInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_86) = &input.stream_arn {
        object.key("StreamARN").string(var_86.as_str());
    }
    if let Some(var_87) = &input.consumer_name {
        object.key("ConsumerName").string(var_87.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_remove_tags_from_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::RemoveTagsFromStreamInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_88) = &input.stream_name {
        object.key("StreamName").string(var_88.as_str());
    }
    if let Some(var_89) = &input.tag_keys {
        let mut array_90 = object.key("TagKeys").start_array();
        for item_91 in var_89 {
             {
                array_90.value().string(item_91.as_str());
            }
        }
        array_90.finish();
    }
    if let Some(var_92) = &input.stream_arn {
        object.key("StreamARN").string(var_92.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_split_shard_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::SplitShardInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_93) = &input.stream_name {
        object.key("StreamName").string(var_93.as_str());
    }
    if let Some(var_94) = &input.shard_to_split {
        object.key("ShardToSplit").string(var_94.as_str());
    }
    if let Some(var_95) = &input.new_starting_hash_key {
        object.key("NewStartingHashKey").string(var_95.as_str());
    }
    if let Some(var_96) = &input.stream_arn {
        object.key("StreamARN").string(var_96.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_stream_encryption_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartStreamEncryptionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_97) = &input.stream_name {
        object.key("StreamName").string(var_97.as_str());
    }
    if let Some(var_98) = &input.encryption_type {
        object.key("EncryptionType").string(var_98.as_str());
    }
    if let Some(var_99) = &input.key_id {
        object.key("KeyId").string(var_99.as_str());
    }
    if let Some(var_100) = &input.stream_arn {
        object.key("StreamARN").string(var_100.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_stop_stream_encryption_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StopStreamEncryptionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_101) = &input.stream_name {
        object.key("StreamName").string(var_101.as_str());
    }
    if let Some(var_102) = &input.encryption_type {
        object.key("EncryptionType").string(var_102.as_str());
    }
    if let Some(var_103) = &input.key_id {
        object.key("KeyId").string(var_103.as_str());
    }
    if let Some(var_104) = &input.stream_arn {
        object.key("StreamARN").string(var_104.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_shard_count_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateShardCountInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_105) = &input.stream_name {
        object.key("StreamName").string(var_105.as_str());
    }
    if let Some(var_106) = &input.target_shard_count {
        object.key("TargetShardCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_106).into()));
    }
    if let Some(var_107) = &input.scaling_type {
        object.key("ScalingType").string(var_107.as_str());
    }
    if let Some(var_108) = &input.stream_arn {
        object.key("StreamARN").string(var_108.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_stream_mode_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateStreamModeInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_109) = &input.stream_arn {
        object.key("StreamARN").string(var_109.as_str());
    }
    if let Some(var_110) = &input.stream_mode_details {
        #[allow(unused_mut)]
        let mut object_111 = object.key("StreamModeDetails").start_object();
        crate::json_ser::serialize_structure_crate_model_stream_mode_details(&mut object_111, var_110)?;
        object_111.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_stream_mode_details(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::StreamModeDetails) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_112) = &input.stream_mode {
        object.key("StreamMode").string(var_112.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_shard_filter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ShardFilter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_113) = &input.r#type {
        object.key("Type").string(var_113.as_str());
    }
    if let Some(var_114) = &input.shard_id {
        object.key("ShardId").string(var_114.as_str());
    }
    if let Some(var_115) = &input.timestamp {
        object.key("Timestamp").date_time(var_115, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

pub fn serialize_structure_crate_model_put_records_request_entry(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PutRecordsRequestEntry) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_116) = &input.data {
        object.key("Data").string_unchecked(&aws_smithy_types::base64::encode(var_116));
    }
    if let Some(var_117) = &input.explicit_hash_key {
        object.key("ExplicitHashKey").string(var_117.as_str());
    }
    if let Some(var_118) = &input.partition_key {
        object.key("PartitionKey").string(var_118.as_str());
    }
    Ok(())
}

