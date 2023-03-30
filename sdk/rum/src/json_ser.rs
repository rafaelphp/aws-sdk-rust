// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_batch_create_rum_metric_definitions_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::BatchCreateRumMetricDefinitionsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.destination {
        object.key("Destination").string(var_1.as_str());
    }
    if let Some(var_2) = &input.destination_arn {
        object.key("DestinationArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.metric_definitions {
        let mut array_4 = object.key("MetricDefinitions").start_array();
        for item_5 in var_3 {
             {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::json_ser::serialize_structure_crate_model_metric_definition_request(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_app_monitor_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateAppMonitorInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_7) = &input.app_monitor_configuration {
        #[allow(unused_mut)]
        let mut object_8 = object.key("AppMonitorConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_app_monitor_configuration(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.custom_events {
        #[allow(unused_mut)]
        let mut object_10 = object.key("CustomEvents").start_object();
        crate::json_ser::serialize_structure_crate_model_custom_events(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.cw_log_enabled {
        object.key("CwLogEnabled").boolean(*var_11);
    }
    if let Some(var_12) = &input.domain {
        object.key("Domain").string(var_12.as_str());
    }
    if let Some(var_13) = &input.name {
        object.key("Name").string(var_13.as_str());
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

pub fn serialize_structure_crate_input_get_app_monitor_data_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetAppMonitorDataInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_18) = &input.filters {
        let mut array_19 = object.key("Filters").start_array();
        for item_20 in var_18 {
             {
                #[allow(unused_mut)]
                let mut object_21 = array_19.value().start_object();
                crate::json_ser::serialize_structure_crate_model_query_filter(&mut object_21, item_20)?;
                object_21.finish();
            }
        }
        array_19.finish();
    }
    if input.max_results != 0 {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_results).into()));
    }
    if let Some(var_22) = &input.next_token {
        object.key("NextToken").string(var_22.as_str());
    }
    if let Some(var_23) = &input.time_range {
        #[allow(unused_mut)]
        let mut object_24 = object.key("TimeRange").start_object();
        crate::json_ser::serialize_structure_crate_model_time_range(&mut object_24, var_23)?;
        object_24.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_rum_events_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutRumEventsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_25) = &input.app_monitor_details {
        #[allow(unused_mut)]
        let mut object_26 = object.key("AppMonitorDetails").start_object();
        crate::json_ser::serialize_structure_crate_model_app_monitor_details(&mut object_26, var_25)?;
        object_26.finish();
    }
    if let Some(var_27) = &input.batch_id {
        object.key("BatchId").string(var_27.as_str());
    }
    if let Some(var_28) = &input.rum_events {
        let mut array_29 = object.key("RumEvents").start_array();
        for item_30 in var_28 {
             {
                #[allow(unused_mut)]
                let mut object_31 = array_29.value().start_object();
                crate::json_ser::serialize_structure_crate_model_rum_event(&mut object_31, item_30)?;
                object_31.finish();
            }
        }
        array_29.finish();
    }
    if let Some(var_32) = &input.user_details {
        #[allow(unused_mut)]
        let mut object_33 = object.key("UserDetails").start_object();
        crate::json_ser::serialize_structure_crate_model_user_details(&mut object_33, var_32)?;
        object_33.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_rum_metrics_destination_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutRumMetricsDestinationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_34) = &input.destination {
        object.key("Destination").string(var_34.as_str());
    }
    if let Some(var_35) = &input.destination_arn {
        object.key("DestinationArn").string(var_35.as_str());
    }
    if let Some(var_36) = &input.iam_role_arn {
        object.key("IamRoleArn").string(var_36.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_37) = &input.tags {
        #[allow(unused_mut)]
        let mut object_38 = object.key("Tags").start_object();
        for (key_39, value_40) in var_37 {
             {
                object_38.key(key_39.as_str()).string(value_40.as_str());
            }
        }
        object_38.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_app_monitor_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateAppMonitorInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_41) = &input.app_monitor_configuration {
        #[allow(unused_mut)]
        let mut object_42 = object.key("AppMonitorConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_app_monitor_configuration(&mut object_42, var_41)?;
        object_42.finish();
    }
    if let Some(var_43) = &input.custom_events {
        #[allow(unused_mut)]
        let mut object_44 = object.key("CustomEvents").start_object();
        crate::json_ser::serialize_structure_crate_model_custom_events(&mut object_44, var_43)?;
        object_44.finish();
    }
    if let Some(var_45) = &input.cw_log_enabled {
        object.key("CwLogEnabled").boolean(*var_45);
    }
    if let Some(var_46) = &input.domain {
        object.key("Domain").string(var_46.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_rum_metric_definition_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateRumMetricDefinitionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_47) = &input.destination {
        object.key("Destination").string(var_47.as_str());
    }
    if let Some(var_48) = &input.destination_arn {
        object.key("DestinationArn").string(var_48.as_str());
    }
    if let Some(var_49) = &input.metric_definition {
        #[allow(unused_mut)]
        let mut object_50 = object.key("MetricDefinition").start_object();
        crate::json_ser::serialize_structure_crate_model_metric_definition_request(&mut object_50, var_49)?;
        object_50.finish();
    }
    if let Some(var_51) = &input.metric_definition_id {
        object.key("MetricDefinitionId").string(var_51.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_metric_definition_request(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::MetricDefinitionRequest) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_52) = &input.name {
        object.key("Name").string(var_52.as_str());
    }
    if let Some(var_53) = &input.value_key {
        object.key("ValueKey").string(var_53.as_str());
    }
    if let Some(var_54) = &input.unit_label {
        object.key("UnitLabel").string(var_54.as_str());
    }
    if let Some(var_55) = &input.dimension_keys {
        #[allow(unused_mut)]
        let mut object_56 = object.key("DimensionKeys").start_object();
        for (key_57, value_58) in var_55 {
             {
                object_56.key(key_57.as_str()).string(value_58.as_str());
            }
        }
        object_56.finish();
    }
    if let Some(var_59) = &input.event_pattern {
        object.key("EventPattern").string(var_59.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_app_monitor_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AppMonitorConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_60) = &input.identity_pool_id {
        object.key("IdentityPoolId").string(var_60.as_str());
    }
    if let Some(var_61) = &input.excluded_pages {
        let mut array_62 = object.key("ExcludedPages").start_array();
        for item_63 in var_61 {
             {
                array_62.value().string(item_63.as_str());
            }
        }
        array_62.finish();
    }
    if let Some(var_64) = &input.included_pages {
        let mut array_65 = object.key("IncludedPages").start_array();
        for item_66 in var_64 {
             {
                array_65.value().string(item_66.as_str());
            }
        }
        array_65.finish();
    }
    if let Some(var_67) = &input.favorite_pages {
        let mut array_68 = object.key("FavoritePages").start_array();
        for item_69 in var_67 {
             {
                array_68.value().string(item_69.as_str());
            }
        }
        array_68.finish();
    }
    if input.session_sample_rate != 0.0 {
        object.key("SessionSampleRate").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((input.session_sample_rate).into()));
    }
    if let Some(var_70) = &input.guest_role_arn {
        object.key("GuestRoleArn").string(var_70.as_str());
    }
    if let Some(var_71) = &input.allow_cookies {
        object.key("AllowCookies").boolean(*var_71);
    }
    if let Some(var_72) = &input.telemetries {
        let mut array_73 = object.key("Telemetries").start_array();
        for item_74 in var_72 {
             {
                array_73.value().string(item_74.as_str());
            }
        }
        array_73.finish();
    }
    if let Some(var_75) = &input.enable_x_ray {
        object.key("EnableXRay").boolean(*var_75);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_custom_events(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::CustomEvents) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_76) = &input.status {
        object.key("Status").string(var_76.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_query_filter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::QueryFilter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_77) = &input.name {
        object.key("Name").string(var_77.as_str());
    }
    if let Some(var_78) = &input.values {
        let mut array_79 = object.key("Values").start_array();
        for item_80 in var_78 {
             {
                array_79.value().string(item_80.as_str());
            }
        }
        array_79.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_time_range(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::TimeRange) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
     {
        object.key("After").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.after).into()));
    }
    if input.before != 0 {
        object.key("Before").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.before).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_model_app_monitor_details(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AppMonitorDetails) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_81) = &input.name {
        object.key("name").string(var_81.as_str());
    }
    if let Some(var_82) = &input.id {
        object.key("id").string(var_82.as_str());
    }
    if let Some(var_83) = &input.version {
        object.key("version").string(var_83.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_rum_event(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::RumEvent) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_84) = &input.id {
        object.key("id").string(var_84.as_str());
    }
    if let Some(var_85) = &input.timestamp {
        object.key("timestamp").date_time(var_85, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_86) = &input.r#type {
        object.key("type").string(var_86.as_str());
    }
    if let Some(var_87) = &input.metadata {
        object.key("metadata").string(var_87.as_str());
    }
    if let Some(var_88) = &input.details {
        object.key("details").string(var_88.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_user_details(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::UserDetails) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_89) = &input.user_id {
        object.key("userId").string(var_89.as_str());
    }
    if let Some(var_90) = &input.session_id {
        object.key("sessionId").string(var_90.as_str());
    }
    Ok(())
}

