// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_job_manifest_location(
    input: &crate::types::JobManifestLocation,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.object_arn {
        let mut inner_writer = scope.start_el("ObjectArn").finish();
        inner_writer.data(var_1.as_str());
    }
    if let Some(var_2) = &input.object_version_id {
        let mut inner_writer = scope.start_el("ObjectVersionId").finish();
        inner_writer.data(var_2.as_str());
    }
    if let Some(var_3) = &input.e_tag {
        let mut inner_writer = scope.start_el("ETag").finish();
        inner_writer.data(var_3.as_str());
    }
    scope.finish();
    Ok(())
}

pub fn de_job_manifest_location(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::JobManifestLocation, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::JobManifestLocation::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ObjectArn") /* ObjectArn com.amazonaws.s3control#JobManifestLocation$ObjectArn */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_object_arn(var_4);
            }
            ,
            s if s.matches("ObjectVersionId") /* ObjectVersionId com.amazonaws.s3control#JobManifestLocation$ObjectVersionId */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_object_version_id(var_5);
            }
            ,
            s if s.matches("ETag") /* ETag com.amazonaws.s3control#JobManifestLocation$ETag */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_e_tag(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
