// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_delete_report_definition_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteReportDefinitionOutput, crate::error::DeleteReportDefinitionError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::DeleteReportDefinitionError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DeleteReportDefinitionError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::DeleteReportDefinitionError { meta: generic, kind: crate::error::DeleteReportDefinitionErrorKind::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteReportDefinitionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InternalServerException" => crate::error::DeleteReportDefinitionError { meta: generic, kind: crate::error::DeleteReportDefinitionErrorKind::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteReportDefinitionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ThrottlingException" => crate::error::DeleteReportDefinitionError { meta: generic, kind: crate::error::DeleteReportDefinitionErrorKind::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteReportDefinitionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ValidationException" => crate::error::DeleteReportDefinitionError { meta: generic, kind: crate::error::DeleteReportDefinitionErrorKind::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteReportDefinitionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::DeleteReportDefinitionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_delete_report_definition_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteReportDefinitionOutput, crate::error::DeleteReportDefinitionError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_report_definition_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_delete_report_definition(response.body().as_ref(), output).map_err(crate::error::DeleteReportDefinitionError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_report_definition_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetReportDefinitionOutput, crate::error::GetReportDefinitionError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::GetReportDefinitionError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetReportDefinitionError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::GetReportDefinitionError { meta: generic, kind: crate::error::GetReportDefinitionErrorKind::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetReportDefinitionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InternalServerException" => crate::error::GetReportDefinitionError { meta: generic, kind: crate::error::GetReportDefinitionErrorKind::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetReportDefinitionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ThrottlingException" => crate::error::GetReportDefinitionError { meta: generic, kind: crate::error::GetReportDefinitionErrorKind::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetReportDefinitionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ValidationException" => crate::error::GetReportDefinitionError { meta: generic, kind: crate::error::GetReportDefinitionErrorKind::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetReportDefinitionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::GetReportDefinitionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_report_definition_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetReportDefinitionOutput, crate::error::GetReportDefinitionError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_report_definition_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_get_report_definition(response.body().as_ref(), output).map_err(crate::error::GetReportDefinitionError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_import_application_usage_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ImportApplicationUsageOutput, crate::error::ImportApplicationUsageError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::ImportApplicationUsageError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ImportApplicationUsageError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::ImportApplicationUsageError { meta: generic, kind: crate::error::ImportApplicationUsageErrorKind::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ImportApplicationUsageError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InternalServerException" => crate::error::ImportApplicationUsageError { meta: generic, kind: crate::error::ImportApplicationUsageErrorKind::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ImportApplicationUsageError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ThrottlingException" => crate::error::ImportApplicationUsageError { meta: generic, kind: crate::error::ImportApplicationUsageErrorKind::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ImportApplicationUsageError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ValidationException" => crate::error::ImportApplicationUsageError { meta: generic, kind: crate::error::ImportApplicationUsageErrorKind::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ImportApplicationUsageError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::ImportApplicationUsageError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_import_application_usage_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ImportApplicationUsageOutput, crate::error::ImportApplicationUsageError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::import_application_usage_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_import_application_usage(response.body().as_ref(), output).map_err(crate::error::ImportApplicationUsageError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_report_definitions_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListReportDefinitionsOutput, crate::error::ListReportDefinitionsError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::ListReportDefinitionsError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ListReportDefinitionsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::ListReportDefinitionsError { meta: generic, kind: crate::error::ListReportDefinitionsErrorKind::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListReportDefinitionsError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InternalServerException" => crate::error::ListReportDefinitionsError { meta: generic, kind: crate::error::ListReportDefinitionsErrorKind::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListReportDefinitionsError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ThrottlingException" => crate::error::ListReportDefinitionsError { meta: generic, kind: crate::error::ListReportDefinitionsErrorKind::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListReportDefinitionsError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ValidationException" => crate::error::ListReportDefinitionsError { meta: generic, kind: crate::error::ListReportDefinitionsErrorKind::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListReportDefinitionsError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::ListReportDefinitionsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_report_definitions_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListReportDefinitionsOutput, crate::error::ListReportDefinitionsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_report_definitions_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_list_report_definitions(response.body().as_ref(), output).map_err(crate::error::ListReportDefinitionsError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_put_report_definition_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::PutReportDefinitionOutput, crate::error::PutReportDefinitionError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::PutReportDefinitionError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::PutReportDefinitionError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::PutReportDefinitionError { meta: generic, kind: crate::error::PutReportDefinitionErrorKind::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutReportDefinitionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InternalServerException" => crate::error::PutReportDefinitionError { meta: generic, kind: crate::error::PutReportDefinitionErrorKind::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutReportDefinitionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ServiceQuotaExceededException" => crate::error::PutReportDefinitionError { meta: generic, kind: crate::error::PutReportDefinitionErrorKind::ServiceQuotaExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_quota_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_service_quota_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutReportDefinitionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ThrottlingException" => crate::error::PutReportDefinitionError { meta: generic, kind: crate::error::PutReportDefinitionErrorKind::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutReportDefinitionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ValidationException" => crate::error::PutReportDefinitionError { meta: generic, kind: crate::error::PutReportDefinitionErrorKind::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutReportDefinitionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::PutReportDefinitionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_put_report_definition_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::PutReportDefinitionOutput, crate::error::PutReportDefinitionError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::put_report_definition_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_put_report_definition(response.body().as_ref(), output).map_err(crate::error::PutReportDefinitionError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_update_report_definition_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateReportDefinitionOutput, crate::error::UpdateReportDefinitionError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::UpdateReportDefinitionError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::UpdateReportDefinitionError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::UpdateReportDefinitionError { meta: generic, kind: crate::error::UpdateReportDefinitionErrorKind::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateReportDefinitionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InternalServerException" => crate::error::UpdateReportDefinitionError { meta: generic, kind: crate::error::UpdateReportDefinitionErrorKind::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateReportDefinitionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ThrottlingException" => crate::error::UpdateReportDefinitionError { meta: generic, kind: crate::error::UpdateReportDefinitionErrorKind::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateReportDefinitionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ValidationException" => crate::error::UpdateReportDefinitionError { meta: generic, kind: crate::error::UpdateReportDefinitionErrorKind::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateReportDefinitionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::UpdateReportDefinitionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_update_report_definition_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateReportDefinitionOutput, crate::error::UpdateReportDefinitionError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::update_report_definition_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_update_report_definition(response.body().as_ref(), output).map_err(crate::error::UpdateReportDefinitionError::unhandled)?;
        output.build()
    })
}

