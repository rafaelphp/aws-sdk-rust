// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_scaling_process_types_input_input(
    input: &crate::operation::describe_scaling_process_types::DescribeScalingProcessTypesInput,
) -> Result<::aws_smithy_http::body::SdkBody, ::aws_smithy_http::operation::error::SerializationError> {
    let _ = input;
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DescribeScalingProcessTypes", "2011-01-01");
    writer.finish();
    Ok(::aws_smithy_http::body::SdkBody::from(out))
}
