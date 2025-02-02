// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_multi_polygon_geometry_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::MultiPolygonGeometryInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.coordinates {
        let mut array_2 = object.key("Coordinates").start_array();
        for item_3 in var_1 {
            {
                let mut array_4 = array_2.value().start_array();
                for item_5 in item_3 {
                    {
                        let mut array_6 = array_4.value().start_array();
                        for item_7 in item_5 {
                            {
                                let mut array_8 = array_6.value().start_array();
                                for item_9 in item_7 {
                                    {
                                        array_8.value().number(
                                            #[allow(clippy::useless_conversion)]
                                            ::aws_smithy_types::Number::Float((*item_9).into()),
                                        );
                                    }
                                }
                                array_8.finish();
                            }
                        }
                        array_6.finish();
                    }
                }
                array_4.finish();
            }
        }
        array_2.finish();
    }
    Ok(())
}

pub(crate) fn de_multi_polygon_geometry_input<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::MultiPolygonGeometryInput>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::MultiPolygonGeometryInputBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Coordinates" => {
                            builder = builder.set_coordinates(crate::protocol_serde::shape_linear_rings_list::de_linear_rings_list(tokens)?);
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
