// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `BatchPutMetrics`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`batch_put_metrics`](crate::client::Client::batch_put_metrics).
            ///
            /// See [`crate::client::fluent_builders::BatchPutMetrics`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchPutMetrics {
    _private: ()
}
impl BatchPutMetrics {
    /// Creates a new builder-style object to manufacture [`BatchPutMetricsInput`](crate::input::BatchPutMetricsInput).
    pub fn builder() -> crate::input::batch_put_metrics_input::Builder {
        crate::input::batch_put_metrics_input::Builder::default()
    }
    /// Creates a new `BatchPutMetrics` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchPutMetrics {
                type Output = std::result::Result<crate::output::BatchPutMetricsOutput, crate::error::BatchPutMetricsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_batch_put_metrics_error(response)
                     } else {
                        crate::operation_deser::parse_batch_put_metrics_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

