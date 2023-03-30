// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_environment_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateEnvironmentInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.airflow_configuration_options {
        #[allow(unused_mut)]
        let mut object_2 = object.key("AirflowConfigurationOptions").start_object();
        for (key_3, value_4) in var_1 {
             {
                object_2.key(key_3.as_str()).string(value_4.as_str());
            }
        }
        object_2.finish();
    }
    if let Some(var_5) = &input.airflow_version {
        object.key("AirflowVersion").string(var_5.as_str());
    }
    if let Some(var_6) = &input.dag_s3_path {
        object.key("DagS3Path").string(var_6.as_str());
    }
    if let Some(var_7) = &input.environment_class {
        object.key("EnvironmentClass").string(var_7.as_str());
    }
    if let Some(var_8) = &input.execution_role_arn {
        object.key("ExecutionRoleArn").string(var_8.as_str());
    }
    if let Some(var_9) = &input.kms_key {
        object.key("KmsKey").string(var_9.as_str());
    }
    if let Some(var_10) = &input.logging_configuration {
        #[allow(unused_mut)]
        let mut object_11 = object.key("LoggingConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_logging_configuration_input(&mut object_11, var_10)?;
        object_11.finish();
    }
    if let Some(var_12) = &input.max_workers {
        object.key("MaxWorkers").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_12).into()));
    }
    if let Some(var_13) = &input.min_workers {
        object.key("MinWorkers").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_13).into()));
    }
    if let Some(var_14) = &input.network_configuration {
        #[allow(unused_mut)]
        let mut object_15 = object.key("NetworkConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_network_configuration(&mut object_15, var_14)?;
        object_15.finish();
    }
    if let Some(var_16) = &input.plugins_s3_object_version {
        object.key("PluginsS3ObjectVersion").string(var_16.as_str());
    }
    if let Some(var_17) = &input.plugins_s3_path {
        object.key("PluginsS3Path").string(var_17.as_str());
    }
    if let Some(var_18) = &input.requirements_s3_object_version {
        object.key("RequirementsS3ObjectVersion").string(var_18.as_str());
    }
    if let Some(var_19) = &input.requirements_s3_path {
        object.key("RequirementsS3Path").string(var_19.as_str());
    }
    if let Some(var_20) = &input.schedulers {
        object.key("Schedulers").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_20).into()));
    }
    if let Some(var_21) = &input.source_bucket_arn {
        object.key("SourceBucketArn").string(var_21.as_str());
    }
    if let Some(var_22) = &input.tags {
        #[allow(unused_mut)]
        let mut object_23 = object.key("Tags").start_object();
        for (key_24, value_25) in var_22 {
             {
                object_23.key(key_24.as_str()).string(value_25.as_str());
            }
        }
        object_23.finish();
    }
    if let Some(var_26) = &input.webserver_access_mode {
        object.key("WebserverAccessMode").string(var_26.as_str());
    }
    if let Some(var_27) = &input.weekly_maintenance_window_start {
        object.key("WeeklyMaintenanceWindowStart").string(var_27.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_publish_metrics_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PublishMetricsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_28) = &input.metric_data {
        let mut array_29 = object.key("MetricData").start_array();
        for item_30 in var_28 {
             {
                #[allow(unused_mut)]
                let mut object_31 = array_29.value().start_object();
                crate::json_ser::serialize_structure_crate_model_metric_datum(&mut object_31, item_30)?;
                object_31.finish();
            }
        }
        array_29.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_32) = &input.tags {
        #[allow(unused_mut)]
        let mut object_33 = object.key("Tags").start_object();
        for (key_34, value_35) in var_32 {
             {
                object_33.key(key_34.as_str()).string(value_35.as_str());
            }
        }
        object_33.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_environment_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateEnvironmentInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_36) = &input.airflow_configuration_options {
        #[allow(unused_mut)]
        let mut object_37 = object.key("AirflowConfigurationOptions").start_object();
        for (key_38, value_39) in var_36 {
             {
                object_37.key(key_38.as_str()).string(value_39.as_str());
            }
        }
        object_37.finish();
    }
    if let Some(var_40) = &input.airflow_version {
        object.key("AirflowVersion").string(var_40.as_str());
    }
    if let Some(var_41) = &input.dag_s3_path {
        object.key("DagS3Path").string(var_41.as_str());
    }
    if let Some(var_42) = &input.environment_class {
        object.key("EnvironmentClass").string(var_42.as_str());
    }
    if let Some(var_43) = &input.execution_role_arn {
        object.key("ExecutionRoleArn").string(var_43.as_str());
    }
    if let Some(var_44) = &input.logging_configuration {
        #[allow(unused_mut)]
        let mut object_45 = object.key("LoggingConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_logging_configuration_input(&mut object_45, var_44)?;
        object_45.finish();
    }
    if let Some(var_46) = &input.max_workers {
        object.key("MaxWorkers").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_46).into()));
    }
    if let Some(var_47) = &input.min_workers {
        object.key("MinWorkers").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_47).into()));
    }
    if let Some(var_48) = &input.network_configuration {
        #[allow(unused_mut)]
        let mut object_49 = object.key("NetworkConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_update_network_configuration_input(&mut object_49, var_48)?;
        object_49.finish();
    }
    if let Some(var_50) = &input.plugins_s3_object_version {
        object.key("PluginsS3ObjectVersion").string(var_50.as_str());
    }
    if let Some(var_51) = &input.plugins_s3_path {
        object.key("PluginsS3Path").string(var_51.as_str());
    }
    if let Some(var_52) = &input.requirements_s3_object_version {
        object.key("RequirementsS3ObjectVersion").string(var_52.as_str());
    }
    if let Some(var_53) = &input.requirements_s3_path {
        object.key("RequirementsS3Path").string(var_53.as_str());
    }
    if let Some(var_54) = &input.schedulers {
        object.key("Schedulers").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_54).into()));
    }
    if let Some(var_55) = &input.source_bucket_arn {
        object.key("SourceBucketArn").string(var_55.as_str());
    }
    if let Some(var_56) = &input.webserver_access_mode {
        object.key("WebserverAccessMode").string(var_56.as_str());
    }
    if let Some(var_57) = &input.weekly_maintenance_window_start {
        object.key("WeeklyMaintenanceWindowStart").string(var_57.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_logging_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::LoggingConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_58) = &input.dag_processing_logs {
        #[allow(unused_mut)]
        let mut object_59 = object.key("DagProcessingLogs").start_object();
        crate::json_ser::serialize_structure_crate_model_module_logging_configuration_input(&mut object_59, var_58)?;
        object_59.finish();
    }
    if let Some(var_60) = &input.scheduler_logs {
        #[allow(unused_mut)]
        let mut object_61 = object.key("SchedulerLogs").start_object();
        crate::json_ser::serialize_structure_crate_model_module_logging_configuration_input(&mut object_61, var_60)?;
        object_61.finish();
    }
    if let Some(var_62) = &input.webserver_logs {
        #[allow(unused_mut)]
        let mut object_63 = object.key("WebserverLogs").start_object();
        crate::json_ser::serialize_structure_crate_model_module_logging_configuration_input(&mut object_63, var_62)?;
        object_63.finish();
    }
    if let Some(var_64) = &input.worker_logs {
        #[allow(unused_mut)]
        let mut object_65 = object.key("WorkerLogs").start_object();
        crate::json_ser::serialize_structure_crate_model_module_logging_configuration_input(&mut object_65, var_64)?;
        object_65.finish();
    }
    if let Some(var_66) = &input.task_logs {
        #[allow(unused_mut)]
        let mut object_67 = object.key("TaskLogs").start_object();
        crate::json_ser::serialize_structure_crate_model_module_logging_configuration_input(&mut object_67, var_66)?;
        object_67.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_network_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::NetworkConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_68) = &input.subnet_ids {
        let mut array_69 = object.key("SubnetIds").start_array();
        for item_70 in var_68 {
             {
                array_69.value().string(item_70.as_str());
            }
        }
        array_69.finish();
    }
    if let Some(var_71) = &input.security_group_ids {
        let mut array_72 = object.key("SecurityGroupIds").start_array();
        for item_73 in var_71 {
             {
                array_72.value().string(item_73.as_str());
            }
        }
        array_72.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_metric_datum(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::MetricDatum) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_74) = &input.metric_name {
        object.key("MetricName").string(var_74.as_str());
    }
    if let Some(var_75) = &input.timestamp {
        object.key("Timestamp").date_time(var_75, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_76) = &input.dimensions {
        let mut array_77 = object.key("Dimensions").start_array();
        for item_78 in var_76 {
             {
                #[allow(unused_mut)]
                let mut object_79 = array_77.value().start_object();
                crate::json_ser::serialize_structure_crate_model_dimension(&mut object_79, item_78)?;
                object_79.finish();
            }
        }
        array_77.finish();
    }
    if let Some(var_80) = &input.value {
        object.key("Value").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*var_80).into()));
    }
    if let Some(var_81) = &input.unit {
        object.key("Unit").string(var_81.as_str());
    }
    if let Some(var_82) = &input.statistic_values {
        #[allow(unused_mut)]
        let mut object_83 = object.key("StatisticValues").start_object();
        crate::json_ser::serialize_structure_crate_model_statistic_set(&mut object_83, var_82)?;
        object_83.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_update_network_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::UpdateNetworkConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_84) = &input.security_group_ids {
        let mut array_85 = object.key("SecurityGroupIds").start_array();
        for item_86 in var_84 {
             {
                array_85.value().string(item_86.as_str());
            }
        }
        array_85.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_module_logging_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ModuleLoggingConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_87) = &input.enabled {
        object.key("Enabled").boolean(*var_87);
    }
    if let Some(var_88) = &input.log_level {
        object.key("LogLevel").string(var_88.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dimension(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Dimension) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_89) = &input.name {
        object.key("Name").string(var_89.as_str());
    }
    if let Some(var_90) = &input.value {
        object.key("Value").string(var_90.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_statistic_set(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::StatisticSet) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_91) = &input.sample_count {
        object.key("SampleCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_91).into()));
    }
    if let Some(var_92) = &input.sum {
        object.key("Sum").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*var_92).into()));
    }
    if let Some(var_93) = &input.minimum {
        object.key("Minimum").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*var_93).into()));
    }
    if let Some(var_94) = &input.maximum {
        object.key("Maximum").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*var_94).into()));
    }
    Ok(())
}

