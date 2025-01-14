// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartDetectorModelAnalysis`](crate::operation::start_detector_model_analysis::builders::StartDetectorModelAnalysisFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`detector_model_definition(DetectorModelDefinition)`](crate::operation::start_detector_model_analysis::builders::StartDetectorModelAnalysisFluentBuilder::detector_model_definition) / [`set_detector_model_definition(Option<DetectorModelDefinition>)`](crate::operation::start_detector_model_analysis::builders::StartDetectorModelAnalysisFluentBuilder::set_detector_model_definition): <p>Information that defines how a detector operates.</p>
    /// - On success, responds with [`StartDetectorModelAnalysisOutput`](crate::operation::start_detector_model_analysis::StartDetectorModelAnalysisOutput) with field(s):
    ///   - [`analysis_id(Option<String>)`](crate::operation::start_detector_model_analysis::StartDetectorModelAnalysisOutput::analysis_id): <p>The ID that you can use to retrieve the analysis result.</p>
    /// - On failure, responds with [`SdkError<StartDetectorModelAnalysisError>`](crate::operation::start_detector_model_analysis::StartDetectorModelAnalysisError)
    pub fn start_detector_model_analysis(
        &self,
    ) -> crate::operation::start_detector_model_analysis::builders::StartDetectorModelAnalysisFluentBuilder {
        crate::operation::start_detector_model_analysis::builders::StartDetectorModelAnalysisFluentBuilder::new(self.handle.clone())
    }
}
