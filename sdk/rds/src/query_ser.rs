// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn serialize_structure_crate_model_tag(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::Tag) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Key");
    if let Some(var_2) = &input.key {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Value");
    if let Some(var_4) = &input.value {
        scope_3.string(var_4);
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_scaling_configuration(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::ScalingConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("MinCapacity");
    if let Some(var_6) = &input.min_capacity {
        scope_5.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_6).into()));
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("MaxCapacity");
    if let Some(var_8) = &input.max_capacity {
        scope_7.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_8).into()));
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("AutoPause");
    if let Some(var_10) = &input.auto_pause {
        scope_9.boolean(*var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("SecondsUntilAutoPause");
    if let Some(var_12) = &input.seconds_until_auto_pause {
        scope_11.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_12).into()));
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("TimeoutAction");
    if let Some(var_14) = &input.timeout_action {
        scope_13.string(var_14);
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("SecondsBeforeTimeout");
    if let Some(var_16) = &input.seconds_before_timeout {
        scope_15.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_16).into()));
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_serverless_v2_scaling_configuration(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::ServerlessV2ScalingConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("MinCapacity");
    if let Some(var_18) = &input.min_capacity {
        scope_17.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*var_18).into()));
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("MaxCapacity");
    if let Some(var_20) = &input.max_capacity {
        scope_19.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*var_20).into()));
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_processor_feature(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::ProcessorFeature) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("Name");
    if let Some(var_22) = &input.name {
        scope_21.string(var_22);
    }
    #[allow(unused_mut)]
    let mut scope_23 = writer.prefix("Value");
    if let Some(var_24) = &input.value {
        scope_23.string(var_24);
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_user_auth_config(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::UserAuthConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_25 = writer.prefix("Description");
    if let Some(var_26) = &input.description {
        scope_25.string(var_26);
    }
    #[allow(unused_mut)]
    let mut scope_27 = writer.prefix("UserName");
    if let Some(var_28) = &input.user_name {
        scope_27.string(var_28);
    }
    #[allow(unused_mut)]
    let mut scope_29 = writer.prefix("AuthScheme");
    if let Some(var_30) = &input.auth_scheme {
        scope_29.string(var_30.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_31 = writer.prefix("SecretArn");
    if let Some(var_32) = &input.secret_arn {
        scope_31.string(var_32);
    }
    #[allow(unused_mut)]
    let mut scope_33 = writer.prefix("IAMAuth");
    if let Some(var_34) = &input.iam_auth {
        scope_33.string(var_34.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_35 = writer.prefix("ClientPasswordAuthType");
    if let Some(var_36) = &input.client_password_auth_type {
        scope_35.string(var_36.as_str());
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_filter(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::Filter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_37 = writer.prefix("Name");
    if let Some(var_38) = &input.name {
        scope_37.string(var_38);
    }
    #[allow(unused_mut)]
    let mut scope_39 = writer.prefix("Values");
    if let Some(var_40) = &input.values {
        let mut list_42 = scope_39.start_list(false, Some("Value"));
        for item_41 in var_40 {
            #[allow(unused_mut)]
            let mut entry_43 = list_42.entry();
            entry_43.string(item_41);
        }
        list_42.finish();
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_cloudwatch_logs_export_configuration(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::CloudwatchLogsExportConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_44 = writer.prefix("EnableLogTypes");
    if let Some(var_45) = &input.enable_log_types {
        let mut list_47 = scope_44.start_list(false, None);
        for item_46 in var_45 {
            #[allow(unused_mut)]
            let mut entry_48 = list_47.entry();
            entry_48.string(item_46);
        }
        list_47.finish();
    }
    #[allow(unused_mut)]
    let mut scope_49 = writer.prefix("DisableLogTypes");
    if let Some(var_50) = &input.disable_log_types {
        let mut list_52 = scope_49.start_list(false, None);
        for item_51 in var_50 {
            #[allow(unused_mut)]
            let mut entry_53 = list_52.entry();
            entry_53.string(item_51);
        }
        list_52.finish();
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_parameter(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::Parameter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_54 = writer.prefix("ParameterName");
    if let Some(var_55) = &input.parameter_name {
        scope_54.string(var_55);
    }
    #[allow(unused_mut)]
    let mut scope_56 = writer.prefix("ParameterValue");
    if let Some(var_57) = &input.parameter_value {
        scope_56.string(var_57);
    }
    #[allow(unused_mut)]
    let mut scope_58 = writer.prefix("Description");
    if let Some(var_59) = &input.description {
        scope_58.string(var_59);
    }
    #[allow(unused_mut)]
    let mut scope_60 = writer.prefix("Source");
    if let Some(var_61) = &input.source {
        scope_60.string(var_61);
    }
    #[allow(unused_mut)]
    let mut scope_62 = writer.prefix("ApplyType");
    if let Some(var_63) = &input.apply_type {
        scope_62.string(var_63);
    }
    #[allow(unused_mut)]
    let mut scope_64 = writer.prefix("DataType");
    if let Some(var_65) = &input.data_type {
        scope_64.string(var_65);
    }
    #[allow(unused_mut)]
    let mut scope_66 = writer.prefix("AllowedValues");
    if let Some(var_67) = &input.allowed_values {
        scope_66.string(var_67);
    }
    #[allow(unused_mut)]
    let mut scope_68 = writer.prefix("IsModifiable");
    if input.is_modifiable {
        scope_68.boolean(input.is_modifiable);
    }
    #[allow(unused_mut)]
    let mut scope_69 = writer.prefix("MinimumEngineVersion");
    if let Some(var_70) = &input.minimum_engine_version {
        scope_69.string(var_70);
    }
    #[allow(unused_mut)]
    let mut scope_71 = writer.prefix("ApplyMethod");
    if let Some(var_72) = &input.apply_method {
        scope_71.string(var_72.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_73 = writer.prefix("SupportedEngineModes");
    if let Some(var_74) = &input.supported_engine_modes {
        let mut list_76 = scope_73.start_list(false, None);
        for item_75 in var_74 {
            #[allow(unused_mut)]
            let mut entry_77 = list_76.entry();
            entry_77.string(item_75);
        }
        list_76.finish();
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_connection_pool_configuration(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::ConnectionPoolConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_78 = writer.prefix("MaxConnectionsPercent");
    if let Some(var_79) = &input.max_connections_percent {
        scope_78.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_79).into()));
    }
    #[allow(unused_mut)]
    let mut scope_80 = writer.prefix("MaxIdleConnectionsPercent");
    if let Some(var_81) = &input.max_idle_connections_percent {
        scope_80.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_81).into()));
    }
    #[allow(unused_mut)]
    let mut scope_82 = writer.prefix("ConnectionBorrowTimeout");
    if let Some(var_83) = &input.connection_borrow_timeout {
        scope_82.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_83).into()));
    }
    #[allow(unused_mut)]
    let mut scope_84 = writer.prefix("SessionPinningFilters");
    if let Some(var_85) = &input.session_pinning_filters {
        let mut list_87 = scope_84.start_list(false, None);
        for item_86 in var_85 {
            #[allow(unused_mut)]
            let mut entry_88 = list_87.entry();
            entry_88.string(item_86);
        }
        list_87.finish();
    }
    #[allow(unused_mut)]
    let mut scope_89 = writer.prefix("InitQuery");
    if let Some(var_90) = &input.init_query {
        scope_89.string(var_90);
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_option_configuration(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::OptionConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_91 = writer.prefix("OptionName");
    if let Some(var_92) = &input.option_name {
        scope_91.string(var_92);
    }
    #[allow(unused_mut)]
    let mut scope_93 = writer.prefix("Port");
    if let Some(var_94) = &input.port {
        scope_93.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_94).into()));
    }
    #[allow(unused_mut)]
    let mut scope_95 = writer.prefix("OptionVersion");
    if let Some(var_96) = &input.option_version {
        scope_95.string(var_96);
    }
    #[allow(unused_mut)]
    let mut scope_97 = writer.prefix("DBSecurityGroupMemberships");
    if let Some(var_98) = &input.db_security_group_memberships {
        let mut list_100 = scope_97.start_list(false, Some("DBSecurityGroupName"));
        for item_99 in var_98 {
            #[allow(unused_mut)]
            let mut entry_101 = list_100.entry();
            entry_101.string(item_99);
        }
        list_100.finish();
    }
    #[allow(unused_mut)]
    let mut scope_102 = writer.prefix("VpcSecurityGroupMemberships");
    if let Some(var_103) = &input.vpc_security_group_memberships {
        let mut list_105 = scope_102.start_list(false, Some("VpcSecurityGroupId"));
        for item_104 in var_103 {
            #[allow(unused_mut)]
            let mut entry_106 = list_105.entry();
            entry_106.string(item_104);
        }
        list_105.finish();
    }
    #[allow(unused_mut)]
    let mut scope_107 = writer.prefix("OptionSettings");
    if let Some(var_108) = &input.option_settings {
        let mut list_110 = scope_107.start_list(false, Some("OptionSetting"));
        for item_109 in var_108 {
            #[allow(unused_mut)]
            let mut entry_111 = list_110.entry();
            crate::query_ser::serialize_structure_crate_model_option_setting(entry_111, item_109)?;
        }
        list_110.finish();
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_option_setting(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::OptionSetting) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_112 = writer.prefix("Name");
    if let Some(var_113) = &input.name {
        scope_112.string(var_113);
    }
    #[allow(unused_mut)]
    let mut scope_114 = writer.prefix("Value");
    if let Some(var_115) = &input.value {
        scope_114.string(var_115);
    }
    #[allow(unused_mut)]
    let mut scope_116 = writer.prefix("DefaultValue");
    if let Some(var_117) = &input.default_value {
        scope_116.string(var_117);
    }
    #[allow(unused_mut)]
    let mut scope_118 = writer.prefix("Description");
    if let Some(var_119) = &input.description {
        scope_118.string(var_119);
    }
    #[allow(unused_mut)]
    let mut scope_120 = writer.prefix("ApplyType");
    if let Some(var_121) = &input.apply_type {
        scope_120.string(var_121);
    }
    #[allow(unused_mut)]
    let mut scope_122 = writer.prefix("DataType");
    if let Some(var_123) = &input.data_type {
        scope_122.string(var_123);
    }
    #[allow(unused_mut)]
    let mut scope_124 = writer.prefix("AllowedValues");
    if let Some(var_125) = &input.allowed_values {
        scope_124.string(var_125);
    }
    #[allow(unused_mut)]
    let mut scope_126 = writer.prefix("IsModifiable");
    if input.is_modifiable {
        scope_126.boolean(input.is_modifiable);
    }
    #[allow(unused_mut)]
    let mut scope_127 = writer.prefix("IsCollection");
    if input.is_collection {
        scope_127.boolean(input.is_collection);
    }
    Ok(())
}

