// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_start_call_analytics_stream_transcription(op_response: &mut aws_smithy_http::operation::Response) -> std::result::Result<crate::output::StartCallAnalyticsStreamTranscriptionOutput, crate::error::StartCallAnalyticsStreamTranscriptionError> {
    #[allow(unused_variables)]
    let (response, properties) = op_response.parts_mut();
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::start_call_analytics_stream_transcription_output::Builder::default();
        let _ = response;
        output = output.set_call_analytics_transcript_result_stream(
            Some(crate::http_serde::deser_payload_start_call_analytics_stream_transcription_start_call_analytics_stream_transcription_output_call_analytics_transcript_result_stream(response.body_mut())?)
        );
        output = output.set_content_identification_type(
            crate::http_serde::deser_header_start_call_analytics_stream_transcription_start_call_analytics_stream_transcription_output_content_identification_type(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse ContentIdentificationType from header `x-amzn-transcribe-content-identification-type"))?
        );
        output = output.set_content_redaction_type(
            crate::http_serde::deser_header_start_call_analytics_stream_transcription_start_call_analytics_stream_transcription_output_content_redaction_type(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse ContentRedactionType from header `x-amzn-transcribe-content-redaction-type"))?
        );
        output = output.set_enable_partial_results_stabilization(
            crate::http_serde::deser_header_start_call_analytics_stream_transcription_start_call_analytics_stream_transcription_output_enable_partial_results_stabilization(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse EnablePartialResultsStabilization from header `x-amzn-transcribe-enable-partial-results-stabilization"))?
        );
        output = output.set_language_code(
            crate::http_serde::deser_header_start_call_analytics_stream_transcription_start_call_analytics_stream_transcription_output_language_code(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse LanguageCode from header `x-amzn-transcribe-language-code"))?
        );
        output = output.set_language_model_name(
            crate::http_serde::deser_header_start_call_analytics_stream_transcription_start_call_analytics_stream_transcription_output_language_model_name(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse LanguageModelName from header `x-amzn-transcribe-language-model-name"))?
        );
        output = output.set_media_encoding(
            crate::http_serde::deser_header_start_call_analytics_stream_transcription_start_call_analytics_stream_transcription_output_media_encoding(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse MediaEncoding from header `x-amzn-transcribe-media-encoding"))?
        );
        output = output.set_media_sample_rate_hertz(
            crate::http_serde::deser_header_start_call_analytics_stream_transcription_start_call_analytics_stream_transcription_output_media_sample_rate_hertz(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse MediaSampleRateHertz from header `x-amzn-transcribe-sample-rate"))?
        );
        output = output.set_partial_results_stability(
            crate::http_serde::deser_header_start_call_analytics_stream_transcription_start_call_analytics_stream_transcription_output_partial_results_stability(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse PartialResultsStability from header `x-amzn-transcribe-partial-results-stability"))?
        );
        output = output.set_pii_entity_types(
            crate::http_serde::deser_header_start_call_analytics_stream_transcription_start_call_analytics_stream_transcription_output_pii_entity_types(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse PiiEntityTypes from header `x-amzn-transcribe-pii-entity-types"))?
        );
        output = output.set_request_id(
            crate::http_serde::deser_header_start_call_analytics_stream_transcription_start_call_analytics_stream_transcription_output_request_id(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse RequestId from header `x-amzn-request-id"))?
        );
        output = output.set_session_id(
            crate::http_serde::deser_header_start_call_analytics_stream_transcription_start_call_analytics_stream_transcription_output_session_id(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse SessionId from header `x-amzn-transcribe-session-id"))?
        );
        output = output.set_vocabulary_filter_method(
            crate::http_serde::deser_header_start_call_analytics_stream_transcription_start_call_analytics_stream_transcription_output_vocabulary_filter_method(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse VocabularyFilterMethod from header `x-amzn-transcribe-vocabulary-filter-method"))?
        );
        output = output.set_vocabulary_filter_name(
            crate::http_serde::deser_header_start_call_analytics_stream_transcription_start_call_analytics_stream_transcription_output_vocabulary_filter_name(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse VocabularyFilterName from header `x-amzn-transcribe-vocabulary-filter-name"))?
        );
        output = output.set_vocabulary_name(
            crate::http_serde::deser_header_start_call_analytics_stream_transcription_start_call_analytics_stream_transcription_output_vocabulary_name(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse VocabularyName from header `x-amzn-transcribe-vocabulary-name"))?
        );
        output.build().map_err(crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled)?
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_start_call_analytics_stream_transcription_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::StartCallAnalyticsStreamTranscriptionOutput, crate::error::StartCallAnalyticsStreamTranscriptionError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ServiceUnavailableException" => crate::error::StartCallAnalyticsStreamTranscriptionError { meta: generic, kind: crate::error::StartCallAnalyticsStreamTranscriptionErrorKind::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_service_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "BadRequestException" => crate::error::StartCallAnalyticsStreamTranscriptionError { meta: generic, kind: crate::error::StartCallAnalyticsStreamTranscriptionErrorKind::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_bad_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InternalFailureException" => crate::error::StartCallAnalyticsStreamTranscriptionError { meta: generic, kind: crate::error::StartCallAnalyticsStreamTranscriptionErrorKind::InternalFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_failure_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_failure_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ConflictException" => crate::error::StartCallAnalyticsStreamTranscriptionError { meta: generic, kind: crate::error::StartCallAnalyticsStreamTranscriptionErrorKind::ConflictException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::conflict_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "LimitExceededException" => crate::error::StartCallAnalyticsStreamTranscriptionError { meta: generic, kind: crate::error::StartCallAnalyticsStreamTranscriptionErrorKind::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::StartCallAnalyticsStreamTranscriptionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_start_medical_stream_transcription(op_response: &mut aws_smithy_http::operation::Response) -> std::result::Result<crate::output::StartMedicalStreamTranscriptionOutput, crate::error::StartMedicalStreamTranscriptionError> {
    #[allow(unused_variables)]
    let (response, properties) = op_response.parts_mut();
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::start_medical_stream_transcription_output::Builder::default();
        let _ = response;
        output = output.set_content_identification_type(
            crate::http_serde::deser_header_start_medical_stream_transcription_start_medical_stream_transcription_output_content_identification_type(response.headers())
                                    .map_err(|_|crate::error::StartMedicalStreamTranscriptionError::unhandled("Failed to parse ContentIdentificationType from header `x-amzn-transcribe-content-identification-type"))?
        );
        output = output.set_enable_channel_identification(
            crate::http_serde::deser_header_start_medical_stream_transcription_start_medical_stream_transcription_output_enable_channel_identification(response.headers())
                                    .map_err(|_|crate::error::StartMedicalStreamTranscriptionError::unhandled("Failed to parse EnableChannelIdentification from header `x-amzn-transcribe-enable-channel-identification"))?
        );
        output = output.set_language_code(
            crate::http_serde::deser_header_start_medical_stream_transcription_start_medical_stream_transcription_output_language_code(response.headers())
                                    .map_err(|_|crate::error::StartMedicalStreamTranscriptionError::unhandled("Failed to parse LanguageCode from header `x-amzn-transcribe-language-code"))?
        );
        output = output.set_media_encoding(
            crate::http_serde::deser_header_start_medical_stream_transcription_start_medical_stream_transcription_output_media_encoding(response.headers())
                                    .map_err(|_|crate::error::StartMedicalStreamTranscriptionError::unhandled("Failed to parse MediaEncoding from header `x-amzn-transcribe-media-encoding"))?
        );
        output = output.set_media_sample_rate_hertz(
            crate::http_serde::deser_header_start_medical_stream_transcription_start_medical_stream_transcription_output_media_sample_rate_hertz(response.headers())
                                    .map_err(|_|crate::error::StartMedicalStreamTranscriptionError::unhandled("Failed to parse MediaSampleRateHertz from header `x-amzn-transcribe-sample-rate"))?
        );
        output = output.set_number_of_channels(
            crate::http_serde::deser_header_start_medical_stream_transcription_start_medical_stream_transcription_output_number_of_channels(response.headers())
                                    .map_err(|_|crate::error::StartMedicalStreamTranscriptionError::unhandled("Failed to parse NumberOfChannels from header `x-amzn-transcribe-number-of-channels"))?
        );
        output = output.set_request_id(
            crate::http_serde::deser_header_start_medical_stream_transcription_start_medical_stream_transcription_output_request_id(response.headers())
                                    .map_err(|_|crate::error::StartMedicalStreamTranscriptionError::unhandled("Failed to parse RequestId from header `x-amzn-request-id"))?
        );
        output = output.set_session_id(
            crate::http_serde::deser_header_start_medical_stream_transcription_start_medical_stream_transcription_output_session_id(response.headers())
                                    .map_err(|_|crate::error::StartMedicalStreamTranscriptionError::unhandled("Failed to parse SessionId from header `x-amzn-transcribe-session-id"))?
        );
        output = output.set_show_speaker_label(
            crate::http_serde::deser_header_start_medical_stream_transcription_start_medical_stream_transcription_output_show_speaker_label(response.headers())
                                    .map_err(|_|crate::error::StartMedicalStreamTranscriptionError::unhandled("Failed to parse ShowSpeakerLabel from header `x-amzn-transcribe-show-speaker-label"))?
        );
        output = output.set_specialty(
            crate::http_serde::deser_header_start_medical_stream_transcription_start_medical_stream_transcription_output_specialty(response.headers())
                                    .map_err(|_|crate::error::StartMedicalStreamTranscriptionError::unhandled("Failed to parse Specialty from header `x-amzn-transcribe-specialty"))?
        );
        output = output.set_transcript_result_stream(
            Some(crate::http_serde::deser_payload_start_medical_stream_transcription_start_medical_stream_transcription_output_transcript_result_stream(response.body_mut())?)
        );
        output = output.set_type(
            crate::http_serde::deser_header_start_medical_stream_transcription_start_medical_stream_transcription_output_type(response.headers())
                                    .map_err(|_|crate::error::StartMedicalStreamTranscriptionError::unhandled("Failed to parse Type from header `x-amzn-transcribe-type"))?
        );
        output = output.set_vocabulary_name(
            crate::http_serde::deser_header_start_medical_stream_transcription_start_medical_stream_transcription_output_vocabulary_name(response.headers())
                                    .map_err(|_|crate::error::StartMedicalStreamTranscriptionError::unhandled("Failed to parse VocabularyName from header `x-amzn-transcribe-vocabulary-name"))?
        );
        output.build().map_err(crate::error::StartMedicalStreamTranscriptionError::unhandled)?
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_start_medical_stream_transcription_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::StartMedicalStreamTranscriptionOutput, crate::error::StartMedicalStreamTranscriptionError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::StartMedicalStreamTranscriptionError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::StartMedicalStreamTranscriptionError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ServiceUnavailableException" => crate::error::StartMedicalStreamTranscriptionError { meta: generic, kind: crate::error::StartMedicalStreamTranscriptionErrorKind::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_service_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartMedicalStreamTranscriptionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "BadRequestException" => crate::error::StartMedicalStreamTranscriptionError { meta: generic, kind: crate::error::StartMedicalStreamTranscriptionErrorKind::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_bad_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartMedicalStreamTranscriptionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InternalFailureException" => crate::error::StartMedicalStreamTranscriptionError { meta: generic, kind: crate::error::StartMedicalStreamTranscriptionErrorKind::InternalFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_failure_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_failure_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartMedicalStreamTranscriptionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ConflictException" => crate::error::StartMedicalStreamTranscriptionError { meta: generic, kind: crate::error::StartMedicalStreamTranscriptionErrorKind::ConflictException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::conflict_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartMedicalStreamTranscriptionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "LimitExceededException" => crate::error::StartMedicalStreamTranscriptionError { meta: generic, kind: crate::error::StartMedicalStreamTranscriptionErrorKind::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartMedicalStreamTranscriptionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::StartMedicalStreamTranscriptionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_start_stream_transcription(op_response: &mut aws_smithy_http::operation::Response) -> std::result::Result<crate::output::StartStreamTranscriptionOutput, crate::error::StartStreamTranscriptionError> {
    #[allow(unused_variables)]
    let (response, properties) = op_response.parts_mut();
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::start_stream_transcription_output::Builder::default();
        let _ = response;
        output = output.set_content_identification_type(
            crate::http_serde::deser_header_start_stream_transcription_start_stream_transcription_output_content_identification_type(response.headers())
                                    .map_err(|_|crate::error::StartStreamTranscriptionError::unhandled("Failed to parse ContentIdentificationType from header `x-amzn-transcribe-content-identification-type"))?
        );
        output = output.set_content_redaction_type(
            crate::http_serde::deser_header_start_stream_transcription_start_stream_transcription_output_content_redaction_type(response.headers())
                                    .map_err(|_|crate::error::StartStreamTranscriptionError::unhandled("Failed to parse ContentRedactionType from header `x-amzn-transcribe-content-redaction-type"))?
        );
        output = output.set_enable_channel_identification(
            crate::http_serde::deser_header_start_stream_transcription_start_stream_transcription_output_enable_channel_identification(response.headers())
                                    .map_err(|_|crate::error::StartStreamTranscriptionError::unhandled("Failed to parse EnableChannelIdentification from header `x-amzn-transcribe-enable-channel-identification"))?
        );
        output = output.set_enable_partial_results_stabilization(
            crate::http_serde::deser_header_start_stream_transcription_start_stream_transcription_output_enable_partial_results_stabilization(response.headers())
                                    .map_err(|_|crate::error::StartStreamTranscriptionError::unhandled("Failed to parse EnablePartialResultsStabilization from header `x-amzn-transcribe-enable-partial-results-stabilization"))?
        );
        output = output.set_identify_language(
            crate::http_serde::deser_header_start_stream_transcription_start_stream_transcription_output_identify_language(response.headers())
                                    .map_err(|_|crate::error::StartStreamTranscriptionError::unhandled("Failed to parse IdentifyLanguage from header `x-amzn-transcribe-identify-language"))?
        );
        output = output.set_language_code(
            crate::http_serde::deser_header_start_stream_transcription_start_stream_transcription_output_language_code(response.headers())
                                    .map_err(|_|crate::error::StartStreamTranscriptionError::unhandled("Failed to parse LanguageCode from header `x-amzn-transcribe-language-code"))?
        );
        output = output.set_language_model_name(
            crate::http_serde::deser_header_start_stream_transcription_start_stream_transcription_output_language_model_name(response.headers())
                                    .map_err(|_|crate::error::StartStreamTranscriptionError::unhandled("Failed to parse LanguageModelName from header `x-amzn-transcribe-language-model-name"))?
        );
        output = output.set_language_options(
            crate::http_serde::deser_header_start_stream_transcription_start_stream_transcription_output_language_options(response.headers())
                                    .map_err(|_|crate::error::StartStreamTranscriptionError::unhandled("Failed to parse LanguageOptions from header `x-amzn-transcribe-language-options"))?
        );
        output = output.set_media_encoding(
            crate::http_serde::deser_header_start_stream_transcription_start_stream_transcription_output_media_encoding(response.headers())
                                    .map_err(|_|crate::error::StartStreamTranscriptionError::unhandled("Failed to parse MediaEncoding from header `x-amzn-transcribe-media-encoding"))?
        );
        output = output.set_media_sample_rate_hertz(
            crate::http_serde::deser_header_start_stream_transcription_start_stream_transcription_output_media_sample_rate_hertz(response.headers())
                                    .map_err(|_|crate::error::StartStreamTranscriptionError::unhandled("Failed to parse MediaSampleRateHertz from header `x-amzn-transcribe-sample-rate"))?
        );
        output = output.set_number_of_channels(
            crate::http_serde::deser_header_start_stream_transcription_start_stream_transcription_output_number_of_channels(response.headers())
                                    .map_err(|_|crate::error::StartStreamTranscriptionError::unhandled("Failed to parse NumberOfChannels from header `x-amzn-transcribe-number-of-channels"))?
        );
        output = output.set_partial_results_stability(
            crate::http_serde::deser_header_start_stream_transcription_start_stream_transcription_output_partial_results_stability(response.headers())
                                    .map_err(|_|crate::error::StartStreamTranscriptionError::unhandled("Failed to parse PartialResultsStability from header `x-amzn-transcribe-partial-results-stability"))?
        );
        output = output.set_pii_entity_types(
            crate::http_serde::deser_header_start_stream_transcription_start_stream_transcription_output_pii_entity_types(response.headers())
                                    .map_err(|_|crate::error::StartStreamTranscriptionError::unhandled("Failed to parse PiiEntityTypes from header `x-amzn-transcribe-pii-entity-types"))?
        );
        output = output.set_preferred_language(
            crate::http_serde::deser_header_start_stream_transcription_start_stream_transcription_output_preferred_language(response.headers())
                                    .map_err(|_|crate::error::StartStreamTranscriptionError::unhandled("Failed to parse PreferredLanguage from header `x-amzn-transcribe-preferred-language"))?
        );
        output = output.set_request_id(
            crate::http_serde::deser_header_start_stream_transcription_start_stream_transcription_output_request_id(response.headers())
                                    .map_err(|_|crate::error::StartStreamTranscriptionError::unhandled("Failed to parse RequestId from header `x-amzn-request-id"))?
        );
        output = output.set_session_id(
            crate::http_serde::deser_header_start_stream_transcription_start_stream_transcription_output_session_id(response.headers())
                                    .map_err(|_|crate::error::StartStreamTranscriptionError::unhandled("Failed to parse SessionId from header `x-amzn-transcribe-session-id"))?
        );
        output = output.set_show_speaker_label(
            crate::http_serde::deser_header_start_stream_transcription_start_stream_transcription_output_show_speaker_label(response.headers())
                                    .map_err(|_|crate::error::StartStreamTranscriptionError::unhandled("Failed to parse ShowSpeakerLabel from header `x-amzn-transcribe-show-speaker-label"))?
        );
        output = output.set_transcript_result_stream(
            Some(crate::http_serde::deser_payload_start_stream_transcription_start_stream_transcription_output_transcript_result_stream(response.body_mut())?)
        );
        output = output.set_vocabulary_filter_method(
            crate::http_serde::deser_header_start_stream_transcription_start_stream_transcription_output_vocabulary_filter_method(response.headers())
                                    .map_err(|_|crate::error::StartStreamTranscriptionError::unhandled("Failed to parse VocabularyFilterMethod from header `x-amzn-transcribe-vocabulary-filter-method"))?
        );
        output = output.set_vocabulary_filter_name(
            crate::http_serde::deser_header_start_stream_transcription_start_stream_transcription_output_vocabulary_filter_name(response.headers())
                                    .map_err(|_|crate::error::StartStreamTranscriptionError::unhandled("Failed to parse VocabularyFilterName from header `x-amzn-transcribe-vocabulary-filter-name"))?
        );
        output = output.set_vocabulary_filter_names(
            crate::http_serde::deser_header_start_stream_transcription_start_stream_transcription_output_vocabulary_filter_names(response.headers())
                                    .map_err(|_|crate::error::StartStreamTranscriptionError::unhandled("Failed to parse VocabularyFilterNames from header `x-amzn-transcribe-vocabulary-filter-names"))?
        );
        output = output.set_vocabulary_name(
            crate::http_serde::deser_header_start_stream_transcription_start_stream_transcription_output_vocabulary_name(response.headers())
                                    .map_err(|_|crate::error::StartStreamTranscriptionError::unhandled("Failed to parse VocabularyName from header `x-amzn-transcribe-vocabulary-name"))?
        );
        output = output.set_vocabulary_names(
            crate::http_serde::deser_header_start_stream_transcription_start_stream_transcription_output_vocabulary_names(response.headers())
                                    .map_err(|_|crate::error::StartStreamTranscriptionError::unhandled("Failed to parse VocabularyNames from header `x-amzn-transcribe-vocabulary-names"))?
        );
        output.build().map_err(crate::error::StartStreamTranscriptionError::unhandled)?
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_start_stream_transcription_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::StartStreamTranscriptionOutput, crate::error::StartStreamTranscriptionError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::StartStreamTranscriptionError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::StartStreamTranscriptionError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ServiceUnavailableException" => crate::error::StartStreamTranscriptionError { meta: generic, kind: crate::error::StartStreamTranscriptionErrorKind::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_service_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartStreamTranscriptionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "BadRequestException" => crate::error::StartStreamTranscriptionError { meta: generic, kind: crate::error::StartStreamTranscriptionErrorKind::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_bad_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartStreamTranscriptionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InternalFailureException" => crate::error::StartStreamTranscriptionError { meta: generic, kind: crate::error::StartStreamTranscriptionErrorKind::InternalFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_failure_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_failure_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartStreamTranscriptionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ConflictException" => crate::error::StartStreamTranscriptionError { meta: generic, kind: crate::error::StartStreamTranscriptionErrorKind::ConflictException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::conflict_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartStreamTranscriptionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "LimitExceededException" => crate::error::StartStreamTranscriptionError { meta: generic, kind: crate::error::StartStreamTranscriptionErrorKind::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartStreamTranscriptionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::StartStreamTranscriptionError::generic(generic)
    })
}

