// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_query_forecast_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::QueryForecastOutput, crate::error::QueryForecastError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::QueryForecastError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::QueryForecastError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidInputException" => crate::error::QueryForecastError { meta: generic, kind: crate::error::QueryForecastErrorKind::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_input_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_invalid_input_exception_json_err(response.body().as_ref(), output).map_err(crate::error::QueryForecastError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InvalidNextTokenException" => crate::error::QueryForecastError { meta: generic, kind: crate::error::QueryForecastErrorKind::InvalidNextTokenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_next_token_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_invalid_next_token_exception_json_err(response.body().as_ref(), output).map_err(crate::error::QueryForecastError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "LimitExceededException" => crate::error::QueryForecastError { meta: generic, kind: crate::error::QueryForecastErrorKind::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::QueryForecastError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ResourceInUseException" => crate::error::QueryForecastError { meta: generic, kind: crate::error::QueryForecastErrorKind::ResourceInUseException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_in_use_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_in_use_exception_json_err(response.body().as_ref(), output).map_err(crate::error::QueryForecastError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ResourceNotFoundException" => crate::error::QueryForecastError { meta: generic, kind: crate::error::QueryForecastErrorKind::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::QueryForecastError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::QueryForecastError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_query_forecast_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::QueryForecastOutput, crate::error::QueryForecastError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::query_forecast_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_query_forecast(response.body().as_ref(), output).map_err(crate::error::QueryForecastError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_query_what_if_forecast_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::QueryWhatIfForecastOutput, crate::error::QueryWhatIfForecastError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::QueryWhatIfForecastError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::QueryWhatIfForecastError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidInputException" => crate::error::QueryWhatIfForecastError { meta: generic, kind: crate::error::QueryWhatIfForecastErrorKind::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_input_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_invalid_input_exception_json_err(response.body().as_ref(), output).map_err(crate::error::QueryWhatIfForecastError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InvalidNextTokenException" => crate::error::QueryWhatIfForecastError { meta: generic, kind: crate::error::QueryWhatIfForecastErrorKind::InvalidNextTokenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_next_token_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_invalid_next_token_exception_json_err(response.body().as_ref(), output).map_err(crate::error::QueryWhatIfForecastError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "LimitExceededException" => crate::error::QueryWhatIfForecastError { meta: generic, kind: crate::error::QueryWhatIfForecastErrorKind::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::QueryWhatIfForecastError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ResourceInUseException" => crate::error::QueryWhatIfForecastError { meta: generic, kind: crate::error::QueryWhatIfForecastErrorKind::ResourceInUseException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_in_use_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_in_use_exception_json_err(response.body().as_ref(), output).map_err(crate::error::QueryWhatIfForecastError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ResourceNotFoundException" => crate::error::QueryWhatIfForecastError { meta: generic, kind: crate::error::QueryWhatIfForecastErrorKind::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::QueryWhatIfForecastError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::QueryWhatIfForecastError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_query_what_if_forecast_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::QueryWhatIfForecastOutput, crate::error::QueryWhatIfForecastError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::query_what_if_forecast_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_query_what_if_forecast(response.body().as_ref(), output).map_err(crate::error::QueryWhatIfForecastError::unhandled)?;
        output.build()
    })
}

