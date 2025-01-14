// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetSendStatisticsInput {}
impl GetSendStatisticsInput {
    /// Creates a new builder-style object to manufacture [`GetSendStatisticsInput`](crate::operation::get_send_statistics::GetSendStatisticsInput).
    pub fn builder() -> crate::operation::get_send_statistics::builders::GetSendStatisticsInputBuilder {
        crate::operation::get_send_statistics::builders::GetSendStatisticsInputBuilder::default()
    }
}

/// A builder for [`GetSendStatisticsInput`](crate::operation::get_send_statistics::GetSendStatisticsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetSendStatisticsInputBuilder {}
impl GetSendStatisticsInputBuilder {
    /// Consumes the builder and constructs a [`GetSendStatisticsInput`](crate::operation::get_send_statistics::GetSendStatisticsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::get_send_statistics::GetSendStatisticsInput, ::aws_smithy_http::operation::error::BuildError> {
        ::std::result::Result::Ok(crate::operation::get_send_statistics::GetSendStatisticsInput {})
    }
}
