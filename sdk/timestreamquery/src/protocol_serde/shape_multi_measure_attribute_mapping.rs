// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_multi_measure_attribute_mapping(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::MultiMeasureAttributeMapping,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.source_column {
        object.key("SourceColumn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.target_multi_measure_attribute_name {
        object.key("TargetMultiMeasureAttributeName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.measure_value_type {
        object.key("MeasureValueType").string(var_3.as_str());
    }
    Ok(())
}

pub(crate) fn de_multi_measure_attribute_mapping<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::MultiMeasureAttributeMapping>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::MultiMeasureAttributeMappingBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "SourceColumn" => {
                            builder = builder.set_source_column(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "TargetMultiMeasureAttributeName" => {
                            builder = builder.set_target_multi_measure_attribute_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "MeasureValueType" => {
                            builder = builder.set_measure_value_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ScalarMeasureValueType::from(u.as_ref())))
                                    .transpose()?,
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
