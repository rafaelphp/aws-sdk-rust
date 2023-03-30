// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CancelJournalKinesisStream`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`cancel_journal_kinesis_stream`](crate::client::Client::cancel_journal_kinesis_stream).
            ///
            /// See [`crate::client::fluent_builders::CancelJournalKinesisStream`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CancelJournalKinesisStream {
    _private: ()
}
impl CancelJournalKinesisStream {
    /// Creates a new builder-style object to manufacture [`CancelJournalKinesisStreamInput`](crate::input::CancelJournalKinesisStreamInput).
    pub fn builder() -> crate::input::cancel_journal_kinesis_stream_input::Builder {
        crate::input::cancel_journal_kinesis_stream_input::Builder::default()
    }
    /// Creates a new `CancelJournalKinesisStream` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CancelJournalKinesisStream {
                type Output = std::result::Result<crate::output::CancelJournalKinesisStreamOutput, crate::error::CancelJournalKinesisStreamError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_cancel_journal_kinesis_stream_error(response)
                     } else {
                        crate::operation_deser::parse_cancel_journal_kinesis_stream_response(response)
                     }
                }
            }

/// Operation shape for `CreateLedger`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_ledger`](crate::client::Client::create_ledger).
            ///
            /// See [`crate::client::fluent_builders::CreateLedger`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateLedger {
    _private: ()
}
impl CreateLedger {
    /// Creates a new builder-style object to manufacture [`CreateLedgerInput`](crate::input::CreateLedgerInput).
    pub fn builder() -> crate::input::create_ledger_input::Builder {
        crate::input::create_ledger_input::Builder::default()
    }
    /// Creates a new `CreateLedger` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateLedger {
                type Output = std::result::Result<crate::output::CreateLedgerOutput, crate::error::CreateLedgerError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_ledger_error(response)
                     } else {
                        crate::operation_deser::parse_create_ledger_response(response)
                     }
                }
            }

/// Operation shape for `DeleteLedger`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_ledger`](crate::client::Client::delete_ledger).
            ///
            /// See [`crate::client::fluent_builders::DeleteLedger`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteLedger {
    _private: ()
}
impl DeleteLedger {
    /// Creates a new builder-style object to manufacture [`DeleteLedgerInput`](crate::input::DeleteLedgerInput).
    pub fn builder() -> crate::input::delete_ledger_input::Builder {
        crate::input::delete_ledger_input::Builder::default()
    }
    /// Creates a new `DeleteLedger` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteLedger {
                type Output = std::result::Result<crate::output::DeleteLedgerOutput, crate::error::DeleteLedgerError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_ledger_error(response)
                     } else {
                        crate::operation_deser::parse_delete_ledger_response(response)
                     }
                }
            }

/// Operation shape for `DescribeJournalKinesisStream`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_journal_kinesis_stream`](crate::client::Client::describe_journal_kinesis_stream).
            ///
            /// See [`crate::client::fluent_builders::DescribeJournalKinesisStream`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeJournalKinesisStream {
    _private: ()
}
impl DescribeJournalKinesisStream {
    /// Creates a new builder-style object to manufacture [`DescribeJournalKinesisStreamInput`](crate::input::DescribeJournalKinesisStreamInput).
    pub fn builder() -> crate::input::describe_journal_kinesis_stream_input::Builder {
        crate::input::describe_journal_kinesis_stream_input::Builder::default()
    }
    /// Creates a new `DescribeJournalKinesisStream` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeJournalKinesisStream {
                type Output = std::result::Result<crate::output::DescribeJournalKinesisStreamOutput, crate::error::DescribeJournalKinesisStreamError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_journal_kinesis_stream_error(response)
                     } else {
                        crate::operation_deser::parse_describe_journal_kinesis_stream_response(response)
                     }
                }
            }

/// Operation shape for `DescribeJournalS3Export`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_journal_s3_export`](crate::client::Client::describe_journal_s3_export).
            ///
            /// See [`crate::client::fluent_builders::DescribeJournalS3Export`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeJournalS3Export {
    _private: ()
}
impl DescribeJournalS3Export {
    /// Creates a new builder-style object to manufacture [`DescribeJournalS3ExportInput`](crate::input::DescribeJournalS3ExportInput).
    pub fn builder() -> crate::input::describe_journal_s3_export_input::Builder {
        crate::input::describe_journal_s3_export_input::Builder::default()
    }
    /// Creates a new `DescribeJournalS3Export` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeJournalS3Export {
                type Output = std::result::Result<crate::output::DescribeJournalS3ExportOutput, crate::error::DescribeJournalS3ExportError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_journal_s3_export_error(response)
                     } else {
                        crate::operation_deser::parse_describe_journal_s3_export_response(response)
                     }
                }
            }

/// Operation shape for `DescribeLedger`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_ledger`](crate::client::Client::describe_ledger).
            ///
            /// See [`crate::client::fluent_builders::DescribeLedger`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeLedger {
    _private: ()
}
impl DescribeLedger {
    /// Creates a new builder-style object to manufacture [`DescribeLedgerInput`](crate::input::DescribeLedgerInput).
    pub fn builder() -> crate::input::describe_ledger_input::Builder {
        crate::input::describe_ledger_input::Builder::default()
    }
    /// Creates a new `DescribeLedger` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeLedger {
                type Output = std::result::Result<crate::output::DescribeLedgerOutput, crate::error::DescribeLedgerError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_ledger_error(response)
                     } else {
                        crate::operation_deser::parse_describe_ledger_response(response)
                     }
                }
            }

/// Operation shape for `ExportJournalToS3`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`export_journal_to_s3`](crate::client::Client::export_journal_to_s3).
            ///
            /// See [`crate::client::fluent_builders::ExportJournalToS3`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ExportJournalToS3 {
    _private: ()
}
impl ExportJournalToS3 {
    /// Creates a new builder-style object to manufacture [`ExportJournalToS3Input`](crate::input::ExportJournalToS3Input).
    pub fn builder() -> crate::input::export_journal_to_s3_input::Builder {
        crate::input::export_journal_to_s3_input::Builder::default()
    }
    /// Creates a new `ExportJournalToS3` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ExportJournalToS3 {
                type Output = std::result::Result<crate::output::ExportJournalToS3Output, crate::error::ExportJournalToS3Error>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_export_journal_to_s3_error(response)
                     } else {
                        crate::operation_deser::parse_export_journal_to_s3_response(response)
                     }
                }
            }

/// Operation shape for `GetBlock`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_block`](crate::client::Client::get_block).
            ///
            /// See [`crate::client::fluent_builders::GetBlock`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetBlock {
    _private: ()
}
impl GetBlock {
    /// Creates a new builder-style object to manufacture [`GetBlockInput`](crate::input::GetBlockInput).
    pub fn builder() -> crate::input::get_block_input::Builder {
        crate::input::get_block_input::Builder::default()
    }
    /// Creates a new `GetBlock` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetBlock {
                type Output = std::result::Result<crate::output::GetBlockOutput, crate::error::GetBlockError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_block_error(response)
                     } else {
                        crate::operation_deser::parse_get_block_response(response)
                     }
                }
            }

/// Operation shape for `GetDigest`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_digest`](crate::client::Client::get_digest).
            ///
            /// See [`crate::client::fluent_builders::GetDigest`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetDigest {
    _private: ()
}
impl GetDigest {
    /// Creates a new builder-style object to manufacture [`GetDigestInput`](crate::input::GetDigestInput).
    pub fn builder() -> crate::input::get_digest_input::Builder {
        crate::input::get_digest_input::Builder::default()
    }
    /// Creates a new `GetDigest` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetDigest {
                type Output = std::result::Result<crate::output::GetDigestOutput, crate::error::GetDigestError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_digest_error(response)
                     } else {
                        crate::operation_deser::parse_get_digest_response(response)
                     }
                }
            }

/// Operation shape for `GetRevision`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_revision`](crate::client::Client::get_revision).
            ///
            /// See [`crate::client::fluent_builders::GetRevision`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetRevision {
    _private: ()
}
impl GetRevision {
    /// Creates a new builder-style object to manufacture [`GetRevisionInput`](crate::input::GetRevisionInput).
    pub fn builder() -> crate::input::get_revision_input::Builder {
        crate::input::get_revision_input::Builder::default()
    }
    /// Creates a new `GetRevision` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetRevision {
                type Output = std::result::Result<crate::output::GetRevisionOutput, crate::error::GetRevisionError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_revision_error(response)
                     } else {
                        crate::operation_deser::parse_get_revision_response(response)
                     }
                }
            }

/// Operation shape for `ListJournalKinesisStreamsForLedger`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_journal_kinesis_streams_for_ledger`](crate::client::Client::list_journal_kinesis_streams_for_ledger).
            ///
            /// See [`crate::client::fluent_builders::ListJournalKinesisStreamsForLedger`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListJournalKinesisStreamsForLedger {
    _private: ()
}
impl ListJournalKinesisStreamsForLedger {
    /// Creates a new builder-style object to manufacture [`ListJournalKinesisStreamsForLedgerInput`](crate::input::ListJournalKinesisStreamsForLedgerInput).
    pub fn builder() -> crate::input::list_journal_kinesis_streams_for_ledger_input::Builder {
        crate::input::list_journal_kinesis_streams_for_ledger_input::Builder::default()
    }
    /// Creates a new `ListJournalKinesisStreamsForLedger` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListJournalKinesisStreamsForLedger {
                type Output = std::result::Result<crate::output::ListJournalKinesisStreamsForLedgerOutput, crate::error::ListJournalKinesisStreamsForLedgerError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_journal_kinesis_streams_for_ledger_error(response)
                     } else {
                        crate::operation_deser::parse_list_journal_kinesis_streams_for_ledger_response(response)
                     }
                }
            }

/// Operation shape for `ListJournalS3Exports`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_journal_s3_exports`](crate::client::Client::list_journal_s3_exports).
            ///
            /// See [`crate::client::fluent_builders::ListJournalS3Exports`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListJournalS3Exports {
    _private: ()
}
impl ListJournalS3Exports {
    /// Creates a new builder-style object to manufacture [`ListJournalS3ExportsInput`](crate::input::ListJournalS3ExportsInput).
    pub fn builder() -> crate::input::list_journal_s3_exports_input::Builder {
        crate::input::list_journal_s3_exports_input::Builder::default()
    }
    /// Creates a new `ListJournalS3Exports` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListJournalS3Exports {
                type Output = std::result::Result<crate::output::ListJournalS3ExportsOutput, crate::error::ListJournalS3ExportsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_journal_s3_exports_error(response)
                     } else {
                        crate::operation_deser::parse_list_journal_s3_exports_response(response)
                     }
                }
            }

/// Operation shape for `ListJournalS3ExportsForLedger`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_journal_s3_exports_for_ledger`](crate::client::Client::list_journal_s3_exports_for_ledger).
            ///
            /// See [`crate::client::fluent_builders::ListJournalS3ExportsForLedger`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListJournalS3ExportsForLedger {
    _private: ()
}
impl ListJournalS3ExportsForLedger {
    /// Creates a new builder-style object to manufacture [`ListJournalS3ExportsForLedgerInput`](crate::input::ListJournalS3ExportsForLedgerInput).
    pub fn builder() -> crate::input::list_journal_s3_exports_for_ledger_input::Builder {
        crate::input::list_journal_s3_exports_for_ledger_input::Builder::default()
    }
    /// Creates a new `ListJournalS3ExportsForLedger` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListJournalS3ExportsForLedger {
                type Output = std::result::Result<crate::output::ListJournalS3ExportsForLedgerOutput, crate::error::ListJournalS3ExportsForLedgerError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_journal_s3_exports_for_ledger_error(response)
                     } else {
                        crate::operation_deser::parse_list_journal_s3_exports_for_ledger_response(response)
                     }
                }
            }

/// Operation shape for `ListLedgers`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_ledgers`](crate::client::Client::list_ledgers).
            ///
            /// See [`crate::client::fluent_builders::ListLedgers`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListLedgers {
    _private: ()
}
impl ListLedgers {
    /// Creates a new builder-style object to manufacture [`ListLedgersInput`](crate::input::ListLedgersInput).
    pub fn builder() -> crate::input::list_ledgers_input::Builder {
        crate::input::list_ledgers_input::Builder::default()
    }
    /// Creates a new `ListLedgers` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListLedgers {
                type Output = std::result::Result<crate::output::ListLedgersOutput, crate::error::ListLedgersError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_ledgers_error(response)
                     } else {
                        crate::operation_deser::parse_list_ledgers_response(response)
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

/// Operation shape for `StreamJournalToKinesis`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`stream_journal_to_kinesis`](crate::client::Client::stream_journal_to_kinesis).
            ///
            /// See [`crate::client::fluent_builders::StreamJournalToKinesis`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StreamJournalToKinesis {
    _private: ()
}
impl StreamJournalToKinesis {
    /// Creates a new builder-style object to manufacture [`StreamJournalToKinesisInput`](crate::input::StreamJournalToKinesisInput).
    pub fn builder() -> crate::input::stream_journal_to_kinesis_input::Builder {
        crate::input::stream_journal_to_kinesis_input::Builder::default()
    }
    /// Creates a new `StreamJournalToKinesis` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StreamJournalToKinesis {
                type Output = std::result::Result<crate::output::StreamJournalToKinesisOutput, crate::error::StreamJournalToKinesisError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_stream_journal_to_kinesis_error(response)
                     } else {
                        crate::operation_deser::parse_stream_journal_to_kinesis_response(response)
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

/// Operation shape for `UpdateLedger`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_ledger`](crate::client::Client::update_ledger).
            ///
            /// See [`crate::client::fluent_builders::UpdateLedger`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateLedger {
    _private: ()
}
impl UpdateLedger {
    /// Creates a new builder-style object to manufacture [`UpdateLedgerInput`](crate::input::UpdateLedgerInput).
    pub fn builder() -> crate::input::update_ledger_input::Builder {
        crate::input::update_ledger_input::Builder::default()
    }
    /// Creates a new `UpdateLedger` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateLedger {
                type Output = std::result::Result<crate::output::UpdateLedgerOutput, crate::error::UpdateLedgerError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_ledger_error(response)
                     } else {
                        crate::operation_deser::parse_update_ledger_response(response)
                     }
                }
            }

/// Operation shape for `UpdateLedgerPermissionsMode`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_ledger_permissions_mode`](crate::client::Client::update_ledger_permissions_mode).
            ///
            /// See [`crate::client::fluent_builders::UpdateLedgerPermissionsMode`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateLedgerPermissionsMode {
    _private: ()
}
impl UpdateLedgerPermissionsMode {
    /// Creates a new builder-style object to manufacture [`UpdateLedgerPermissionsModeInput`](crate::input::UpdateLedgerPermissionsModeInput).
    pub fn builder() -> crate::input::update_ledger_permissions_mode_input::Builder {
        crate::input::update_ledger_permissions_mode_input::Builder::default()
    }
    /// Creates a new `UpdateLedgerPermissionsMode` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateLedgerPermissionsMode {
                type Output = std::result::Result<crate::output::UpdateLedgerPermissionsModeOutput, crate::error::UpdateLedgerPermissionsModeError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_ledger_permissions_mode_error(response)
                     } else {
                        crate::operation_deser::parse_update_ledger_permissions_mode_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

