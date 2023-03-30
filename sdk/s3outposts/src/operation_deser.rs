// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_create_endpoint_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateEndpointOutput, crate::error::CreateEndpointError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::CreateEndpointError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CreateEndpointError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::CreateEndpointError { meta: generic, kind: crate::error::CreateEndpointErrorKind::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateEndpointError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ConflictException" => crate::error::CreateEndpointError { meta: generic, kind: crate::error::CreateEndpointErrorKind::ConflictException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::conflict_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateEndpointError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InternalServerException" => crate::error::CreateEndpointError { meta: generic, kind: crate::error::CreateEndpointErrorKind::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateEndpointError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ResourceNotFoundException" => crate::error::CreateEndpointError { meta: generic, kind: crate::error::CreateEndpointErrorKind::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateEndpointError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ValidationException" => crate::error::CreateEndpointError { meta: generic, kind: crate::error::CreateEndpointErrorKind::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateEndpointError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::CreateEndpointError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_create_endpoint_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateEndpointOutput, crate::error::CreateEndpointError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_endpoint_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_create_endpoint(response.body().as_ref(), output).map_err(crate::error::CreateEndpointError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_delete_endpoint_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteEndpointOutput, crate::error::DeleteEndpointError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::DeleteEndpointError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DeleteEndpointError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::DeleteEndpointError { meta: generic, kind: crate::error::DeleteEndpointErrorKind::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteEndpointError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InternalServerException" => crate::error::DeleteEndpointError { meta: generic, kind: crate::error::DeleteEndpointErrorKind::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteEndpointError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ResourceNotFoundException" => crate::error::DeleteEndpointError { meta: generic, kind: crate::error::DeleteEndpointErrorKind::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteEndpointError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ValidationException" => crate::error::DeleteEndpointError { meta: generic, kind: crate::error::DeleteEndpointErrorKind::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteEndpointError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::DeleteEndpointError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_delete_endpoint_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteEndpointOutput, crate::error::DeleteEndpointError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_endpoint_output::Builder::default();
        let _ = response;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_endpoints_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListEndpointsOutput, crate::error::ListEndpointsError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::ListEndpointsError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ListEndpointsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::ListEndpointsError { meta: generic, kind: crate::error::ListEndpointsErrorKind::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListEndpointsError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InternalServerException" => crate::error::ListEndpointsError { meta: generic, kind: crate::error::ListEndpointsErrorKind::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListEndpointsError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ResourceNotFoundException" => crate::error::ListEndpointsError { meta: generic, kind: crate::error::ListEndpointsErrorKind::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListEndpointsError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ValidationException" => crate::error::ListEndpointsError { meta: generic, kind: crate::error::ListEndpointsErrorKind::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListEndpointsError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::ListEndpointsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_endpoints_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListEndpointsOutput, crate::error::ListEndpointsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_endpoints_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_list_endpoints(response.body().as_ref(), output).map_err(crate::error::ListEndpointsError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_shared_endpoints_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListSharedEndpointsOutput, crate::error::ListSharedEndpointsError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::ListSharedEndpointsError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ListSharedEndpointsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::ListSharedEndpointsError { meta: generic, kind: crate::error::ListSharedEndpointsErrorKind::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListSharedEndpointsError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InternalServerException" => crate::error::ListSharedEndpointsError { meta: generic, kind: crate::error::ListSharedEndpointsErrorKind::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListSharedEndpointsError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ResourceNotFoundException" => crate::error::ListSharedEndpointsError { meta: generic, kind: crate::error::ListSharedEndpointsErrorKind::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListSharedEndpointsError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ValidationException" => crate::error::ListSharedEndpointsError { meta: generic, kind: crate::error::ListSharedEndpointsErrorKind::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListSharedEndpointsError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::ListSharedEndpointsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_shared_endpoints_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListSharedEndpointsOutput, crate::error::ListSharedEndpointsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_shared_endpoints_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_list_shared_endpoints(response.body().as_ref(), output).map_err(crate::error::ListSharedEndpointsError::unhandled)?;
        output.build()
    })
}

