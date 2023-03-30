// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_delete_human_loop_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteHumanLoopOutput, crate::error::DeleteHumanLoopError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::DeleteHumanLoopError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DeleteHumanLoopError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InternalServerException" => crate::error::DeleteHumanLoopError { meta: generic, kind: crate::error::DeleteHumanLoopErrorKind::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteHumanLoopError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ResourceNotFoundException" => crate::error::DeleteHumanLoopError { meta: generic, kind: crate::error::DeleteHumanLoopErrorKind::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteHumanLoopError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ThrottlingException" => crate::error::DeleteHumanLoopError { meta: generic, kind: crate::error::DeleteHumanLoopErrorKind::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteHumanLoopError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ValidationException" => crate::error::DeleteHumanLoopError { meta: generic, kind: crate::error::DeleteHumanLoopErrorKind::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteHumanLoopError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::DeleteHumanLoopError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_delete_human_loop_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteHumanLoopOutput, crate::error::DeleteHumanLoopError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_human_loop_output::Builder::default();
        let _ = response;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_human_loop_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeHumanLoopOutput, crate::error::DescribeHumanLoopError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::DescribeHumanLoopError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribeHumanLoopError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InternalServerException" => crate::error::DescribeHumanLoopError { meta: generic, kind: crate::error::DescribeHumanLoopErrorKind::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeHumanLoopError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ResourceNotFoundException" => crate::error::DescribeHumanLoopError { meta: generic, kind: crate::error::DescribeHumanLoopErrorKind::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeHumanLoopError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ThrottlingException" => crate::error::DescribeHumanLoopError { meta: generic, kind: crate::error::DescribeHumanLoopErrorKind::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeHumanLoopError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ValidationException" => crate::error::DescribeHumanLoopError { meta: generic, kind: crate::error::DescribeHumanLoopErrorKind::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeHumanLoopError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::DescribeHumanLoopError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_human_loop_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeHumanLoopOutput, crate::error::DescribeHumanLoopError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_human_loop_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_describe_human_loop(response.body().as_ref(), output).map_err(crate::error::DescribeHumanLoopError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_human_loops_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListHumanLoopsOutput, crate::error::ListHumanLoopsError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::ListHumanLoopsError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ListHumanLoopsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InternalServerException" => crate::error::ListHumanLoopsError { meta: generic, kind: crate::error::ListHumanLoopsErrorKind::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListHumanLoopsError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ResourceNotFoundException" => crate::error::ListHumanLoopsError { meta: generic, kind: crate::error::ListHumanLoopsErrorKind::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListHumanLoopsError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ThrottlingException" => crate::error::ListHumanLoopsError { meta: generic, kind: crate::error::ListHumanLoopsErrorKind::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListHumanLoopsError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ValidationException" => crate::error::ListHumanLoopsError { meta: generic, kind: crate::error::ListHumanLoopsErrorKind::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListHumanLoopsError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::ListHumanLoopsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_human_loops_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListHumanLoopsOutput, crate::error::ListHumanLoopsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_human_loops_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_list_human_loops(response.body().as_ref(), output).map_err(crate::error::ListHumanLoopsError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_start_human_loop_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::StartHumanLoopOutput, crate::error::StartHumanLoopError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::StartHumanLoopError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::StartHumanLoopError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ConflictException" => crate::error::StartHumanLoopError { meta: generic, kind: crate::error::StartHumanLoopErrorKind::ConflictException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::conflict_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartHumanLoopError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InternalServerException" => crate::error::StartHumanLoopError { meta: generic, kind: crate::error::StartHumanLoopErrorKind::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartHumanLoopError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ServiceQuotaExceededException" => crate::error::StartHumanLoopError { meta: generic, kind: crate::error::StartHumanLoopErrorKind::ServiceQuotaExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_quota_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_service_quota_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartHumanLoopError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ThrottlingException" => crate::error::StartHumanLoopError { meta: generic, kind: crate::error::StartHumanLoopErrorKind::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartHumanLoopError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ValidationException" => crate::error::StartHumanLoopError { meta: generic, kind: crate::error::StartHumanLoopErrorKind::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartHumanLoopError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::StartHumanLoopError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_start_human_loop_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::StartHumanLoopOutput, crate::error::StartHumanLoopError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::start_human_loop_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_start_human_loop(response.body().as_ref(), output).map_err(crate::error::StartHumanLoopError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_stop_human_loop_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::StopHumanLoopOutput, crate::error::StopHumanLoopError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::StopHumanLoopError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::StopHumanLoopError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InternalServerException" => crate::error::StopHumanLoopError { meta: generic, kind: crate::error::StopHumanLoopErrorKind::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StopHumanLoopError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ResourceNotFoundException" => crate::error::StopHumanLoopError { meta: generic, kind: crate::error::StopHumanLoopErrorKind::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StopHumanLoopError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ThrottlingException" => crate::error::StopHumanLoopError { meta: generic, kind: crate::error::StopHumanLoopErrorKind::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StopHumanLoopError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ValidationException" => crate::error::StopHumanLoopError { meta: generic, kind: crate::error::StopHumanLoopErrorKind::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StopHumanLoopError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::StopHumanLoopError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_stop_human_loop_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::StopHumanLoopOutput, crate::error::StopHumanLoopError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::stop_human_loop_output::Builder::default();
        let _ = response;
        output.build()
    })
}

