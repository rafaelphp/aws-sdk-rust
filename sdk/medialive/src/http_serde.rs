// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn add_headers_describe_input_device_thumbnail(
                    input: &crate::input::DescribeInputDeviceThumbnailInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.accept {
        let formatted_2 = inner_1.as_str();
                        if !formatted_2.is_empty() {
                            let header_value = formatted_2;
                            let header_value = http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("accept", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("accept", header_value);
                        }
    }
    Ok(builder)
}

pub fn deser_payload_describe_input_device_thumbnail_describe_input_device_thumbnail_output_body(body: &mut aws_smithy_http::body::SdkBody) -> std::result::Result<aws_smithy_http::byte_stream::ByteStream, crate::error::DescribeInputDeviceThumbnailError> {
    // replace the body with an empty body
                let body = std::mem::replace(body, aws_smithy_http::body::SdkBody::taken());
                Ok(aws_smithy_http::byte_stream::ByteStream::new(body))
}

pub(crate) fn deser_header_describe_input_device_thumbnail_describe_input_device_thumbnail_output_content_length(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Content-Length").iter();
    let var_3 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_3.len() > 1 {
                                Err(aws_smithy_http::header::ParseError::new(format!("expected one item but found {}", var_3.len())))
                            } else {
                                let mut var_3 = var_3;
                                Ok(var_3.pop())
                            }
}

pub(crate) fn deser_header_describe_input_device_thumbnail_describe_input_device_thumbnail_output_content_type(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<crate::model::ContentType>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Content-Type").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_describe_input_device_thumbnail_describe_input_device_thumbnail_output_e_tag(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("ETag").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_describe_input_device_thumbnail_describe_input_device_thumbnail_output_last_modified(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<aws_smithy_types::DateTime>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Last-Modified").iter();
    let var_4: Vec<aws_smithy_types::DateTime> = aws_smithy_http::header::many_dates(headers, aws_smithy_types::date_time::Format::HttpDate)?;
    if var_4.len() > 1 {
                                Err(aws_smithy_http::header::ParseError::new(format!("expected one item but found {}", var_4.len())))
                            } else {
                                let mut var_4 = var_4;
                                Ok(var_4.pop())
                            }
}

