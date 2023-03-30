// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `AssociateRepository`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`associate_repository`](crate::client::Client::associate_repository).
            ///
            /// See [`crate::client::fluent_builders::AssociateRepository`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct AssociateRepository {
    _private: ()
}
impl AssociateRepository {
    /// Creates a new builder-style object to manufacture [`AssociateRepositoryInput`](crate::input::AssociateRepositoryInput).
    pub fn builder() -> crate::input::associate_repository_input::Builder {
        crate::input::associate_repository_input::Builder::default()
    }
    /// Creates a new `AssociateRepository` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AssociateRepository {
                type Output = std::result::Result<crate::output::AssociateRepositoryOutput, crate::error::AssociateRepositoryError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_associate_repository_error(response)
                     } else {
                        crate::operation_deser::parse_associate_repository_response(response)
                     }
                }
            }

/// Operation shape for `CreateCodeReview`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_code_review`](crate::client::Client::create_code_review).
            ///
            /// See [`crate::client::fluent_builders::CreateCodeReview`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateCodeReview {
    _private: ()
}
impl CreateCodeReview {
    /// Creates a new builder-style object to manufacture [`CreateCodeReviewInput`](crate::input::CreateCodeReviewInput).
    pub fn builder() -> crate::input::create_code_review_input::Builder {
        crate::input::create_code_review_input::Builder::default()
    }
    /// Creates a new `CreateCodeReview` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateCodeReview {
                type Output = std::result::Result<crate::output::CreateCodeReviewOutput, crate::error::CreateCodeReviewError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_code_review_error(response)
                     } else {
                        crate::operation_deser::parse_create_code_review_response(response)
                     }
                }
            }

/// Operation shape for `DescribeCodeReview`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_code_review`](crate::client::Client::describe_code_review).
            ///
            /// See [`crate::client::fluent_builders::DescribeCodeReview`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeCodeReview {
    _private: ()
}
impl DescribeCodeReview {
    /// Creates a new builder-style object to manufacture [`DescribeCodeReviewInput`](crate::input::DescribeCodeReviewInput).
    pub fn builder() -> crate::input::describe_code_review_input::Builder {
        crate::input::describe_code_review_input::Builder::default()
    }
    /// Creates a new `DescribeCodeReview` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeCodeReview {
                type Output = std::result::Result<crate::output::DescribeCodeReviewOutput, crate::error::DescribeCodeReviewError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_code_review_error(response)
                     } else {
                        crate::operation_deser::parse_describe_code_review_response(response)
                     }
                }
            }

/// Operation shape for `DescribeRecommendationFeedback`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_recommendation_feedback`](crate::client::Client::describe_recommendation_feedback).
            ///
            /// See [`crate::client::fluent_builders::DescribeRecommendationFeedback`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeRecommendationFeedback {
    _private: ()
}
impl DescribeRecommendationFeedback {
    /// Creates a new builder-style object to manufacture [`DescribeRecommendationFeedbackInput`](crate::input::DescribeRecommendationFeedbackInput).
    pub fn builder() -> crate::input::describe_recommendation_feedback_input::Builder {
        crate::input::describe_recommendation_feedback_input::Builder::default()
    }
    /// Creates a new `DescribeRecommendationFeedback` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeRecommendationFeedback {
                type Output = std::result::Result<crate::output::DescribeRecommendationFeedbackOutput, crate::error::DescribeRecommendationFeedbackError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_recommendation_feedback_error(response)
                     } else {
                        crate::operation_deser::parse_describe_recommendation_feedback_response(response)
                     }
                }
            }

/// Operation shape for `DescribeRepositoryAssociation`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_repository_association`](crate::client::Client::describe_repository_association).
            ///
            /// See [`crate::client::fluent_builders::DescribeRepositoryAssociation`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeRepositoryAssociation {
    _private: ()
}
impl DescribeRepositoryAssociation {
    /// Creates a new builder-style object to manufacture [`DescribeRepositoryAssociationInput`](crate::input::DescribeRepositoryAssociationInput).
    pub fn builder() -> crate::input::describe_repository_association_input::Builder {
        crate::input::describe_repository_association_input::Builder::default()
    }
    /// Creates a new `DescribeRepositoryAssociation` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeRepositoryAssociation {
                type Output = std::result::Result<crate::output::DescribeRepositoryAssociationOutput, crate::error::DescribeRepositoryAssociationError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_repository_association_error(response)
                     } else {
                        crate::operation_deser::parse_describe_repository_association_response(response)
                     }
                }
            }

/// Operation shape for `DisassociateRepository`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`disassociate_repository`](crate::client::Client::disassociate_repository).
            ///
            /// See [`crate::client::fluent_builders::DisassociateRepository`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DisassociateRepository {
    _private: ()
}
impl DisassociateRepository {
    /// Creates a new builder-style object to manufacture [`DisassociateRepositoryInput`](crate::input::DisassociateRepositoryInput).
    pub fn builder() -> crate::input::disassociate_repository_input::Builder {
        crate::input::disassociate_repository_input::Builder::default()
    }
    /// Creates a new `DisassociateRepository` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DisassociateRepository {
                type Output = std::result::Result<crate::output::DisassociateRepositoryOutput, crate::error::DisassociateRepositoryError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_disassociate_repository_error(response)
                     } else {
                        crate::operation_deser::parse_disassociate_repository_response(response)
                     }
                }
            }

/// Operation shape for `ListCodeReviews`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_code_reviews`](crate::client::Client::list_code_reviews).
            ///
            /// See [`crate::client::fluent_builders::ListCodeReviews`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListCodeReviews {
    _private: ()
}
impl ListCodeReviews {
    /// Creates a new builder-style object to manufacture [`ListCodeReviewsInput`](crate::input::ListCodeReviewsInput).
    pub fn builder() -> crate::input::list_code_reviews_input::Builder {
        crate::input::list_code_reviews_input::Builder::default()
    }
    /// Creates a new `ListCodeReviews` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListCodeReviews {
                type Output = std::result::Result<crate::output::ListCodeReviewsOutput, crate::error::ListCodeReviewsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_code_reviews_error(response)
                     } else {
                        crate::operation_deser::parse_list_code_reviews_response(response)
                     }
                }
            }

/// Operation shape for `ListRecommendationFeedback`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_recommendation_feedback`](crate::client::Client::list_recommendation_feedback).
            ///
            /// See [`crate::client::fluent_builders::ListRecommendationFeedback`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListRecommendationFeedback {
    _private: ()
}
impl ListRecommendationFeedback {
    /// Creates a new builder-style object to manufacture [`ListRecommendationFeedbackInput`](crate::input::ListRecommendationFeedbackInput).
    pub fn builder() -> crate::input::list_recommendation_feedback_input::Builder {
        crate::input::list_recommendation_feedback_input::Builder::default()
    }
    /// Creates a new `ListRecommendationFeedback` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListRecommendationFeedback {
                type Output = std::result::Result<crate::output::ListRecommendationFeedbackOutput, crate::error::ListRecommendationFeedbackError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_recommendation_feedback_error(response)
                     } else {
                        crate::operation_deser::parse_list_recommendation_feedback_response(response)
                     }
                }
            }

/// Operation shape for `ListRecommendations`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_recommendations`](crate::client::Client::list_recommendations).
            ///
            /// See [`crate::client::fluent_builders::ListRecommendations`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListRecommendations {
    _private: ()
}
impl ListRecommendations {
    /// Creates a new builder-style object to manufacture [`ListRecommendationsInput`](crate::input::ListRecommendationsInput).
    pub fn builder() -> crate::input::list_recommendations_input::Builder {
        crate::input::list_recommendations_input::Builder::default()
    }
    /// Creates a new `ListRecommendations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListRecommendations {
                type Output = std::result::Result<crate::output::ListRecommendationsOutput, crate::error::ListRecommendationsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_recommendations_error(response)
                     } else {
                        crate::operation_deser::parse_list_recommendations_response(response)
                     }
                }
            }

/// Operation shape for `ListRepositoryAssociations`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_repository_associations`](crate::client::Client::list_repository_associations).
            ///
            /// See [`crate::client::fluent_builders::ListRepositoryAssociations`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListRepositoryAssociations {
    _private: ()
}
impl ListRepositoryAssociations {
    /// Creates a new builder-style object to manufacture [`ListRepositoryAssociationsInput`](crate::input::ListRepositoryAssociationsInput).
    pub fn builder() -> crate::input::list_repository_associations_input::Builder {
        crate::input::list_repository_associations_input::Builder::default()
    }
    /// Creates a new `ListRepositoryAssociations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListRepositoryAssociations {
                type Output = std::result::Result<crate::output::ListRepositoryAssociationsOutput, crate::error::ListRepositoryAssociationsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_repository_associations_error(response)
                     } else {
                        crate::operation_deser::parse_list_repository_associations_response(response)
                     }
                }
            }

/// Operation shape for `ListTagsForResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_tags_for_resource`](crate::client::Client::list_tags_for_resource).
            ///
            /// See [`crate::client::fluent_builders::ListTagsForResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: ()
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput).
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    /// Creates a new `ListTagsForResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForResource {
                type Output = std::result::Result<crate::output::ListTagsForResourceOutput, crate::error::ListTagsForResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_tags_for_resource_error(response)
                     } else {
                        crate::operation_deser::parse_list_tags_for_resource_response(response)
                     }
                }
            }

/// Operation shape for `PutRecommendationFeedback`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`put_recommendation_feedback`](crate::client::Client::put_recommendation_feedback).
            ///
            /// See [`crate::client::fluent_builders::PutRecommendationFeedback`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutRecommendationFeedback {
    _private: ()
}
impl PutRecommendationFeedback {
    /// Creates a new builder-style object to manufacture [`PutRecommendationFeedbackInput`](crate::input::PutRecommendationFeedbackInput).
    pub fn builder() -> crate::input::put_recommendation_feedback_input::Builder {
        crate::input::put_recommendation_feedback_input::Builder::default()
    }
    /// Creates a new `PutRecommendationFeedback` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutRecommendationFeedback {
                type Output = std::result::Result<crate::output::PutRecommendationFeedbackOutput, crate::error::PutRecommendationFeedbackError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_put_recommendation_feedback_error(response)
                     } else {
                        crate::operation_deser::parse_put_recommendation_feedback_response(response)
                     }
                }
            }

/// Operation shape for `TagResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`tag_resource`](crate::client::Client::tag_resource).
            ///
            /// See [`crate::client::fluent_builders::TagResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct TagResource {
    _private: ()
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput).
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    /// Creates a new `TagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResource {
                type Output = std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_tag_resource_error(response)
                     } else {
                        crate::operation_deser::parse_tag_resource_response(response)
                     }
                }
            }

/// Operation shape for `UntagResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`untag_resource`](crate::client::Client::untag_resource).
            ///
            /// See [`crate::client::fluent_builders::UntagResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UntagResource {
    _private: ()
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput).
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    /// Creates a new `UntagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResource {
                type Output = std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_untag_resource_error(response)
                     } else {
                        crate::operation_deser::parse_untag_resource_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

