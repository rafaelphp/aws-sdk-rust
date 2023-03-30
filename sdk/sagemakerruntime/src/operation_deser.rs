// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_invoke_endpoint_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::InvokeEndpointOutput, crate::error::InvokeEndpointError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::InvokeEndpointError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::InvokeEndpointError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InternalDependencyException" => crate::error::InvokeEndpointError { meta: generic, kind: crate::error::InvokeEndpointErrorKind::InternalDependencyException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_dependency_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_dependency_exception_json_err(response.body().as_ref(), output).map_err(crate::error::InvokeEndpointError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InternalFailure" => crate::error::InvokeEndpointError { meta: generic, kind: crate::error::InvokeEndpointErrorKind::InternalFailure({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_failure::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_failure_json_err(response.body().as_ref(), output).map_err(crate::error::InvokeEndpointError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ModelError" => crate::error::InvokeEndpointError { meta: generic, kind: crate::error::InvokeEndpointErrorKind::ModelError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::model_error::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_model_error_json_err(response.body().as_ref(), output).map_err(crate::error::InvokeEndpointError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ModelNotReadyException" => crate::error::InvokeEndpointError { meta: generic, kind: crate::error::InvokeEndpointErrorKind::ModelNotReadyException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::model_not_ready_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_model_not_ready_exception_json_err(response.body().as_ref(), output).map_err(crate::error::InvokeEndpointError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ServiceUnavailable" => crate::error::InvokeEndpointError { meta: generic, kind: crate::error::InvokeEndpointErrorKind::ServiceUnavailable({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_unavailable::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_service_unavailable_json_err(response.body().as_ref(), output).map_err(crate::error::InvokeEndpointError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ValidationError" => crate::error::InvokeEndpointError { meta: generic, kind: crate::error::InvokeEndpointErrorKind::ValidationError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_error::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_error_json_err(response.body().as_ref(), output).map_err(crate::error::InvokeEndpointError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::InvokeEndpointError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_invoke_endpoint_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::InvokeEndpointOutput, crate::error::InvokeEndpointError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::invoke_endpoint_output::Builder::default();
        let _ = response;
        output = output.set_body(
            crate::http_serde::deser_payload_invoke_endpoint_invoke_endpoint_output_body(response.body().as_ref())?
        );
        output = output.set_content_type(
            crate::http_serde::deser_header_invoke_endpoint_invoke_endpoint_output_content_type(response.headers())
                                    .map_err(|_|crate::error::InvokeEndpointError::unhandled("Failed to parse ContentType from header `Content-Type"))?
        );
        output = output.set_custom_attributes(
            crate::http_serde::deser_header_invoke_endpoint_invoke_endpoint_output_custom_attributes(response.headers())
                                    .map_err(|_|crate::error::InvokeEndpointError::unhandled("Failed to parse CustomAttributes from header `X-Amzn-SageMaker-Custom-Attributes"))?
        );
        output = output.set_invoked_production_variant(
            crate::http_serde::deser_header_invoke_endpoint_invoke_endpoint_output_invoked_production_variant(response.headers())
                                    .map_err(|_|crate::error::InvokeEndpointError::unhandled("Failed to parse InvokedProductionVariant from header `x-Amzn-Invoked-Production-Variant"))?
        );
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_invoke_endpoint_async_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::InvokeEndpointAsyncOutput, crate::error::InvokeEndpointAsyncError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::InvokeEndpointAsyncError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::InvokeEndpointAsyncError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InternalFailure" => crate::error::InvokeEndpointAsyncError { meta: generic, kind: crate::error::InvokeEndpointAsyncErrorKind::InternalFailure({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_failure::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_failure_json_err(response.body().as_ref(), output).map_err(crate::error::InvokeEndpointAsyncError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ServiceUnavailable" => crate::error::InvokeEndpointAsyncError { meta: generic, kind: crate::error::InvokeEndpointAsyncErrorKind::ServiceUnavailable({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_unavailable::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_service_unavailable_json_err(response.body().as_ref(), output).map_err(crate::error::InvokeEndpointAsyncError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ValidationError" => crate::error::InvokeEndpointAsyncError { meta: generic, kind: crate::error::InvokeEndpointAsyncErrorKind::ValidationError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_error::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_error_json_err(response.body().as_ref(), output).map_err(crate::error::InvokeEndpointAsyncError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::InvokeEndpointAsyncError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_invoke_endpoint_async_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::InvokeEndpointAsyncOutput, crate::error::InvokeEndpointAsyncError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::invoke_endpoint_async_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_invoke_endpoint_async(response.body().as_ref(), output).map_err(crate::error::InvokeEndpointAsyncError::unhandled)?;
        output = output.set_output_location(
            crate::http_serde::deser_header_invoke_endpoint_async_invoke_endpoint_async_output_output_location(response.headers())
                                    .map_err(|_|crate::error::InvokeEndpointAsyncError::unhandled("Failed to parse OutputLocation from header `X-Amzn-SageMaker-OutputLocation"))?
        );
        output.build()
    })
}

