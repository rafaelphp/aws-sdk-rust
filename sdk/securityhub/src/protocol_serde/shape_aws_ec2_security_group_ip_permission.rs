// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_ec2_security_group_ip_permission(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AwsEc2SecurityGroupIpPermission,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.ip_protocol {
        object.key("IpProtocol").string(var_1.as_str());
    }
    if input.from_port != 0 {
        object.key("FromPort").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.from_port).into()),
        );
    }
    if input.to_port != 0 {
        object.key("ToPort").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.to_port).into()),
        );
    }
    if let Some(var_2) = &input.user_id_group_pairs {
        let mut array_3 = object.key("UserIdGroupPairs").start_array();
        for item_4 in var_2 {
            {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_aws_ec2_security_group_user_id_group_pair::ser_aws_ec2_security_group_user_id_group_pair(
                    &mut object_5,
                    item_4,
                )?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.ip_ranges {
        let mut array_7 = object.key("IpRanges").start_array();
        for item_8 in var_6 {
            {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_aws_ec2_security_group_ip_range::ser_aws_ec2_security_group_ip_range(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if let Some(var_10) = &input.ipv6_ranges {
        let mut array_11 = object.key("Ipv6Ranges").start_array();
        for item_12 in var_10 {
            {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_aws_ec2_security_group_ipv6_range::ser_aws_ec2_security_group_ipv6_range(&mut object_13, item_12)?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    if let Some(var_14) = &input.prefix_list_ids {
        let mut array_15 = object.key("PrefixListIds").start_array();
        for item_16 in var_14 {
            {
                #[allow(unused_mut)]
                let mut object_17 = array_15.value().start_object();
                crate::protocol_serde::shape_aws_ec2_security_group_prefix_list_id::ser_aws_ec2_security_group_prefix_list_id(
                    &mut object_17,
                    item_16,
                )?;
                object_17.finish();
            }
        }
        array_15.finish();
    }
    Ok(())
}

pub(crate) fn de_aws_ec2_security_group_ip_permission<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::AwsEc2SecurityGroupIpPermission>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AwsEc2SecurityGroupIpPermissionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "IpProtocol" => {
                            builder = builder.set_ip_protocol(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "FromPort" => {
                            builder = builder.set_from_port(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "ToPort" => {
                            builder = builder.set_to_port(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "UserIdGroupPairs" => {
                            builder = builder.set_user_id_group_pairs(
                                    crate::protocol_serde::shape_aws_ec2_security_group_user_id_group_pair_list::de_aws_ec2_security_group_user_id_group_pair_list(tokens)?
                                );
                        }
                        "IpRanges" => {
                            builder = builder.set_ip_ranges(
                                crate::protocol_serde::shape_aws_ec2_security_group_ip_range_list::de_aws_ec2_security_group_ip_range_list(tokens)?,
                            );
                        }
                        "Ipv6Ranges" => {
                            builder = builder.set_ipv6_ranges(
                                crate::protocol_serde::shape_aws_ec2_security_group_ipv6_range_list::de_aws_ec2_security_group_ipv6_range_list(
                                    tokens,
                                )?,
                            );
                        }
                        "PrefixListIds" => {
                            builder = builder.set_prefix_list_ids(
                                    crate::protocol_serde::shape_aws_ec2_security_group_prefix_list_id_list::de_aws_ec2_security_group_prefix_list_id_list(tokens)?
                                );
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
