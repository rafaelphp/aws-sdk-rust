// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `BatchExecuteStatement`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`batch_execute_statement`](crate::client::Client::batch_execute_statement).
            ///
            /// See [`crate::client::fluent_builders::BatchExecuteStatement`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchExecuteStatement {
    _private: ()
}
impl BatchExecuteStatement {
    /// Creates a new builder-style object to manufacture [`BatchExecuteStatementInput`](crate::input::BatchExecuteStatementInput).
    pub fn builder() -> crate::input::batch_execute_statement_input::Builder {
        crate::input::batch_execute_statement_input::Builder::default()
    }
    /// Creates a new `BatchExecuteStatement` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchExecuteStatement {
                type Output = std::result::Result<crate::output::BatchExecuteStatementOutput, crate::error::BatchExecuteStatementError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_batch_execute_statement_error(response)
                     } else {
                        crate::operation_deser::parse_batch_execute_statement_response(response)
                     }
                }
            }

/// Operation shape for `CancelStatement`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`cancel_statement`](crate::client::Client::cancel_statement).
            ///
            /// See [`crate::client::fluent_builders::CancelStatement`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CancelStatement {
    _private: ()
}
impl CancelStatement {
    /// Creates a new builder-style object to manufacture [`CancelStatementInput`](crate::input::CancelStatementInput).
    pub fn builder() -> crate::input::cancel_statement_input::Builder {
        crate::input::cancel_statement_input::Builder::default()
    }
    /// Creates a new `CancelStatement` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CancelStatement {
                type Output = std::result::Result<crate::output::CancelStatementOutput, crate::error::CancelStatementError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_cancel_statement_error(response)
                     } else {
                        crate::operation_deser::parse_cancel_statement_response(response)
                     }
                }
            }

/// Operation shape for `DescribeStatement`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_statement`](crate::client::Client::describe_statement).
            ///
            /// See [`crate::client::fluent_builders::DescribeStatement`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeStatement {
    _private: ()
}
impl DescribeStatement {
    /// Creates a new builder-style object to manufacture [`DescribeStatementInput`](crate::input::DescribeStatementInput).
    pub fn builder() -> crate::input::describe_statement_input::Builder {
        crate::input::describe_statement_input::Builder::default()
    }
    /// Creates a new `DescribeStatement` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeStatement {
                type Output = std::result::Result<crate::output::DescribeStatementOutput, crate::error::DescribeStatementError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_statement_error(response)
                     } else {
                        crate::operation_deser::parse_describe_statement_response(response)
                     }
                }
            }

/// Operation shape for `DescribeTable`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_table`](crate::client::Client::describe_table).
            ///
            /// See [`crate::client::fluent_builders::DescribeTable`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeTable {
    _private: ()
}
impl DescribeTable {
    /// Creates a new builder-style object to manufacture [`DescribeTableInput`](crate::input::DescribeTableInput).
    pub fn builder() -> crate::input::describe_table_input::Builder {
        crate::input::describe_table_input::Builder::default()
    }
    /// Creates a new `DescribeTable` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeTable {
                type Output = std::result::Result<crate::output::DescribeTableOutput, crate::error::DescribeTableError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_table_error(response)
                     } else {
                        crate::operation_deser::parse_describe_table_response(response)
                     }
                }
            }

/// Operation shape for `ExecuteStatement`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`execute_statement`](crate::client::Client::execute_statement).
            ///
            /// See [`crate::client::fluent_builders::ExecuteStatement`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ExecuteStatement {
    _private: ()
}
impl ExecuteStatement {
    /// Creates a new builder-style object to manufacture [`ExecuteStatementInput`](crate::input::ExecuteStatementInput).
    pub fn builder() -> crate::input::execute_statement_input::Builder {
        crate::input::execute_statement_input::Builder::default()
    }
    /// Creates a new `ExecuteStatement` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ExecuteStatement {
                type Output = std::result::Result<crate::output::ExecuteStatementOutput, crate::error::ExecuteStatementError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_execute_statement_error(response)
                     } else {
                        crate::operation_deser::parse_execute_statement_response(response)
                     }
                }
            }

/// Operation shape for `GetStatementResult`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_statement_result`](crate::client::Client::get_statement_result).
            ///
            /// See [`crate::client::fluent_builders::GetStatementResult`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetStatementResult {
    _private: ()
}
impl GetStatementResult {
    /// Creates a new builder-style object to manufacture [`GetStatementResultInput`](crate::input::GetStatementResultInput).
    pub fn builder() -> crate::input::get_statement_result_input::Builder {
        crate::input::get_statement_result_input::Builder::default()
    }
    /// Creates a new `GetStatementResult` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetStatementResult {
                type Output = std::result::Result<crate::output::GetStatementResultOutput, crate::error::GetStatementResultError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_statement_result_error(response)
                     } else {
                        crate::operation_deser::parse_get_statement_result_response(response)
                     }
                }
            }

/// Operation shape for `ListDatabases`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_databases`](crate::client::Client::list_databases).
            ///
            /// See [`crate::client::fluent_builders::ListDatabases`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListDatabases {
    _private: ()
}
impl ListDatabases {
    /// Creates a new builder-style object to manufacture [`ListDatabasesInput`](crate::input::ListDatabasesInput).
    pub fn builder() -> crate::input::list_databases_input::Builder {
        crate::input::list_databases_input::Builder::default()
    }
    /// Creates a new `ListDatabases` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListDatabases {
                type Output = std::result::Result<crate::output::ListDatabasesOutput, crate::error::ListDatabasesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_databases_error(response)
                     } else {
                        crate::operation_deser::parse_list_databases_response(response)
                     }
                }
            }

/// Operation shape for `ListSchemas`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_schemas`](crate::client::Client::list_schemas).
            ///
            /// See [`crate::client::fluent_builders::ListSchemas`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListSchemas {
    _private: ()
}
impl ListSchemas {
    /// Creates a new builder-style object to manufacture [`ListSchemasInput`](crate::input::ListSchemasInput).
    pub fn builder() -> crate::input::list_schemas_input::Builder {
        crate::input::list_schemas_input::Builder::default()
    }
    /// Creates a new `ListSchemas` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListSchemas {
                type Output = std::result::Result<crate::output::ListSchemasOutput, crate::error::ListSchemasError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_schemas_error(response)
                     } else {
                        crate::operation_deser::parse_list_schemas_response(response)
                     }
                }
            }

/// Operation shape for `ListStatements`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_statements`](crate::client::Client::list_statements).
            ///
            /// See [`crate::client::fluent_builders::ListStatements`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListStatements {
    _private: ()
}
impl ListStatements {
    /// Creates a new builder-style object to manufacture [`ListStatementsInput`](crate::input::ListStatementsInput).
    pub fn builder() -> crate::input::list_statements_input::Builder {
        crate::input::list_statements_input::Builder::default()
    }
    /// Creates a new `ListStatements` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListStatements {
                type Output = std::result::Result<crate::output::ListStatementsOutput, crate::error::ListStatementsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_statements_error(response)
                     } else {
                        crate::operation_deser::parse_list_statements_response(response)
                     }
                }
            }

/// Operation shape for `ListTables`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_tables`](crate::client::Client::list_tables).
            ///
            /// See [`crate::client::fluent_builders::ListTables`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListTables {
    _private: ()
}
impl ListTables {
    /// Creates a new builder-style object to manufacture [`ListTablesInput`](crate::input::ListTablesInput).
    pub fn builder() -> crate::input::list_tables_input::Builder {
        crate::input::list_tables_input::Builder::default()
    }
    /// Creates a new `ListTables` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTables {
                type Output = std::result::Result<crate::output::ListTablesOutput, crate::error::ListTablesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_tables_error(response)
                     } else {
                        crate::operation_deser::parse_list_tables_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

