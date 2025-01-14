// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_continuous_deployment_single_header_config(
    input: &crate::types::ContinuousDeploymentSingleHeaderConfig,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.header {
        let mut inner_writer = scope.start_el("Header").finish();
        inner_writer.data(var_1.as_str());
    }
    if let Some(var_2) = &input.value {
        let mut inner_writer = scope.start_el("Value").finish();
        inner_writer.data(var_2.as_str());
    }
    scope.finish();
    Ok(())
}

pub fn de_continuous_deployment_single_header_config(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::ContinuousDeploymentSingleHeaderConfig, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ContinuousDeploymentSingleHeaderConfig::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Header") /* Header com.amazonaws.cloudfront#ContinuousDeploymentSingleHeaderConfig$Header */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_header(var_3);
            }
            ,
            s if s.matches("Value") /* Value com.amazonaws.cloudfront#ContinuousDeploymentSingleHeaderConfig$Value */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_value(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
