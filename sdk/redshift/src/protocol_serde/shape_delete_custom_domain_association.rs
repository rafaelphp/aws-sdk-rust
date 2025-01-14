// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_custom_domain_association_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_custom_domain_association::DeleteCustomDomainAssociationOutput,
    crate::operation::delete_custom_domain_association::DeleteCustomDomainAssociationError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::delete_custom_domain_association::DeleteCustomDomainAssociationError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::delete_custom_domain_association::DeleteCustomDomainAssociationError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ClusterNotFound" => crate::operation::delete_custom_domain_association::DeleteCustomDomainAssociationError::ClusterNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ClusterNotFoundFaultBuilder::default();
                output = crate::protocol_serde::shape_cluster_not_found_fault::de_cluster_not_found_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::delete_custom_domain_association::DeleteCustomDomainAssociationError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "CustomCnameAssociationFault" => {
            crate::operation::delete_custom_domain_association::DeleteCustomDomainAssociationError::CustomCnameAssociationFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::CustomCnameAssociationFaultBuilder::default();
                    output = crate::protocol_serde::shape_custom_cname_association_fault::de_custom_cname_association_fault_xml_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::delete_custom_domain_association::DeleteCustomDomainAssociationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "UnsupportedOperation" => {
            crate::operation::delete_custom_domain_association::DeleteCustomDomainAssociationError::UnsupportedOperationFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnsupportedOperationFaultBuilder::default();
                    output = crate::protocol_serde::shape_unsupported_operation_fault::de_unsupported_operation_fault_xml_err(_response_body, output)
                        .map_err(crate::operation::delete_custom_domain_association::DeleteCustomDomainAssociationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::delete_custom_domain_association::DeleteCustomDomainAssociationError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_custom_domain_association_http_response(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_custom_domain_association::DeleteCustomDomainAssociationOutput,
    crate::operation::delete_custom_domain_association::DeleteCustomDomainAssociationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_custom_domain_association::builders::DeleteCustomDomainAssociationOutputBuilder::default();
        output._set_request_id(::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}
