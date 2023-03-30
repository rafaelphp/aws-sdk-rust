// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `GetPersonalizedRanking`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_personalized_ranking`](crate::client::Client::get_personalized_ranking).
            ///
            /// See [`crate::client::fluent_builders::GetPersonalizedRanking`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetPersonalizedRanking {
    _private: ()
}
impl GetPersonalizedRanking {
    /// Creates a new builder-style object to manufacture [`GetPersonalizedRankingInput`](crate::input::GetPersonalizedRankingInput).
    pub fn builder() -> crate::input::get_personalized_ranking_input::Builder {
        crate::input::get_personalized_ranking_input::Builder::default()
    }
    /// Creates a new `GetPersonalizedRanking` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetPersonalizedRanking {
                type Output = std::result::Result<crate::output::GetPersonalizedRankingOutput, crate::error::GetPersonalizedRankingError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_personalized_ranking_error(response)
                     } else {
                        crate::operation_deser::parse_get_personalized_ranking_response(response)
                     }
                }
            }

/// Operation shape for `GetRecommendations`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_recommendations`](crate::client::Client::get_recommendations).
            ///
            /// See [`crate::client::fluent_builders::GetRecommendations`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetRecommendations {
    _private: ()
}
impl GetRecommendations {
    /// Creates a new builder-style object to manufacture [`GetRecommendationsInput`](crate::input::GetRecommendationsInput).
    pub fn builder() -> crate::input::get_recommendations_input::Builder {
        crate::input::get_recommendations_input::Builder::default()
    }
    /// Creates a new `GetRecommendations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetRecommendations {
                type Output = std::result::Result<crate::output::GetRecommendationsOutput, crate::error::GetRecommendationsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_recommendations_error(response)
                     } else {
                        crate::operation_deser::parse_get_recommendations_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

