// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Paginator for [`GetInstancesHealthStatus`](crate::operation::GetInstancesHealthStatus)
            pub struct GetInstancesHealthStatusPaginator {
                handle: std::sync::Arc<crate::client::Handle>,
                builder: crate::input::get_instances_health_status_input::Builder,
                stop_on_duplicate_token: bool,
            }

            impl GetInstancesHealthStatusPaginator  {
                /// Create a new paginator-wrapper
                pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>, builder: crate::input::get_instances_health_status_input::Builder) -> Self {
                    Self {
                        handle,
                        builder,
                        stop_on_duplicate_token: true,
                    }
                }

                /// Set the page size
                ///
                /// _Note: this method will override any previously set value for `max_results`_
                pub fn page_size(mut self, limit: i32) -> Self {
                    self.builder.max_results = Some(limit);
                    self
                }

                

                /// Stop paginating when the service returns the same pagination token twice in a row.
                ///
                /// Defaults to true.
                ///
                /// For certain operations, it may be useful to continue on duplicate token. For example,
                /// if an operation is for tailing a log file in real-time, then continuing may be desired.
                /// This option can be set to `false` to accommodate these use cases.
                pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {
                    self.stop_on_duplicate_token = stop_on_duplicate_token;
                    self
                }

                /// Create the pagination stream
                ///
                /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
                pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::output::GetInstancesHealthStatusOutput, aws_smithy_http::result::SdkError<crate::error::GetInstancesHealthStatusError>>> + Unpin
                 {
                    // Move individual fields out of self for the borrow checker
                    let builder = self.builder;
                    let handle = self.handle;
                    aws_smithy_async::future::fn_stream::FnStream::new(move |tx| Box::pin(async move {
                        // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                        let mut input = match builder.build().map_err(aws_smithy_http::result::SdkError::construction_failure) {
                            Ok(input) => input,
                            Err(e) => { let _ = tx.send(Err(e)).await; return; }
                        };
                        loop {
                            let op = match input.make_operation(&handle.conf)
                                .await
                                .map_err(aws_smithy_http::result::SdkError::construction_failure) {
                                Ok(op) => op,
                                Err(e) => {
                                    let _ = tx.send(Err(e)).await;
                                    return;
                                }
                            };
                            let resp = handle.client.call(op).await;
                            // If the input member is None or it was an error
                            let done = match resp {
                                Ok(ref resp) => {
                                    let new_token = crate::lens::reflens_structure_crate_output_get_instances_health_status_output_next_token(resp);
                                    let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                                    if !is_empty && new_token == input.next_token.as_ref() && self.stop_on_duplicate_token {
                                        true
                                    } else {
                                        input.next_token = new_token.cloned();
                                        is_empty
                                    }
                                },
                                Err(_) => true,
                            };
                            if tx.send(resp).await.is_err() {
                                // receiving end was dropped
                                return
                            }
                            if done {
                                return
                            }
                        }
                    }))
                }
            }

/// Paginator for [`ListInstances`](crate::operation::ListInstances)
            pub struct ListInstancesPaginator {
                handle: std::sync::Arc<crate::client::Handle>,
                builder: crate::input::list_instances_input::Builder,
                stop_on_duplicate_token: bool,
            }

            impl ListInstancesPaginator  {
                /// Create a new paginator-wrapper
                pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>, builder: crate::input::list_instances_input::Builder) -> Self {
                    Self {
                        handle,
                        builder,
                        stop_on_duplicate_token: true,
                    }
                }

                /// Set the page size
                ///
                /// _Note: this method will override any previously set value for `max_results`_
                pub fn page_size(mut self, limit: i32) -> Self {
                    self.builder.max_results = Some(limit);
                    self
                }

                

                /// Stop paginating when the service returns the same pagination token twice in a row.
                ///
                /// Defaults to true.
                ///
                /// For certain operations, it may be useful to continue on duplicate token. For example,
                /// if an operation is for tailing a log file in real-time, then continuing may be desired.
                /// This option can be set to `false` to accommodate these use cases.
                pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {
                    self.stop_on_duplicate_token = stop_on_duplicate_token;
                    self
                }

                /// Create the pagination stream
                ///
                /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
                pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::output::ListInstancesOutput, aws_smithy_http::result::SdkError<crate::error::ListInstancesError>>> + Unpin
                 {
                    // Move individual fields out of self for the borrow checker
                    let builder = self.builder;
                    let handle = self.handle;
                    aws_smithy_async::future::fn_stream::FnStream::new(move |tx| Box::pin(async move {
                        // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                        let mut input = match builder.build().map_err(aws_smithy_http::result::SdkError::construction_failure) {
                            Ok(input) => input,
                            Err(e) => { let _ = tx.send(Err(e)).await; return; }
                        };
                        loop {
                            let op = match input.make_operation(&handle.conf)
                                .await
                                .map_err(aws_smithy_http::result::SdkError::construction_failure) {
                                Ok(op) => op,
                                Err(e) => {
                                    let _ = tx.send(Err(e)).await;
                                    return;
                                }
                            };
                            let resp = handle.client.call(op).await;
                            // If the input member is None or it was an error
                            let done = match resp {
                                Ok(ref resp) => {
                                    let new_token = crate::lens::reflens_structure_crate_output_list_instances_output_next_token(resp);
                                    let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                                    if !is_empty && new_token == input.next_token.as_ref() && self.stop_on_duplicate_token {
                                        true
                                    } else {
                                        input.next_token = new_token.cloned();
                                        is_empty
                                    }
                                },
                                Err(_) => true,
                            };
                            if tx.send(resp).await.is_err() {
                                // receiving end was dropped
                                return
                            }
                            if done {
                                return
                            }
                        }
                    }))
                }
            }

/// Paginator for [`ListNamespaces`](crate::operation::ListNamespaces)
            pub struct ListNamespacesPaginator {
                handle: std::sync::Arc<crate::client::Handle>,
                builder: crate::input::list_namespaces_input::Builder,
                stop_on_duplicate_token: bool,
            }

            impl ListNamespacesPaginator  {
                /// Create a new paginator-wrapper
                pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>, builder: crate::input::list_namespaces_input::Builder) -> Self {
                    Self {
                        handle,
                        builder,
                        stop_on_duplicate_token: true,
                    }
                }

                /// Set the page size
                ///
                /// _Note: this method will override any previously set value for `max_results`_
                pub fn page_size(mut self, limit: i32) -> Self {
                    self.builder.max_results = Some(limit);
                    self
                }

                

                /// Stop paginating when the service returns the same pagination token twice in a row.
                ///
                /// Defaults to true.
                ///
                /// For certain operations, it may be useful to continue on duplicate token. For example,
                /// if an operation is for tailing a log file in real-time, then continuing may be desired.
                /// This option can be set to `false` to accommodate these use cases.
                pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {
                    self.stop_on_duplicate_token = stop_on_duplicate_token;
                    self
                }

                /// Create the pagination stream
                ///
                /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
                pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::output::ListNamespacesOutput, aws_smithy_http::result::SdkError<crate::error::ListNamespacesError>>> + Unpin
                 {
                    // Move individual fields out of self for the borrow checker
                    let builder = self.builder;
                    let handle = self.handle;
                    aws_smithy_async::future::fn_stream::FnStream::new(move |tx| Box::pin(async move {
                        // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                        let mut input = match builder.build().map_err(aws_smithy_http::result::SdkError::construction_failure) {
                            Ok(input) => input,
                            Err(e) => { let _ = tx.send(Err(e)).await; return; }
                        };
                        loop {
                            let op = match input.make_operation(&handle.conf)
                                .await
                                .map_err(aws_smithy_http::result::SdkError::construction_failure) {
                                Ok(op) => op,
                                Err(e) => {
                                    let _ = tx.send(Err(e)).await;
                                    return;
                                }
                            };
                            let resp = handle.client.call(op).await;
                            // If the input member is None or it was an error
                            let done = match resp {
                                Ok(ref resp) => {
                                    let new_token = crate::lens::reflens_structure_crate_output_list_namespaces_output_next_token(resp);
                                    let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                                    if !is_empty && new_token == input.next_token.as_ref() && self.stop_on_duplicate_token {
                                        true
                                    } else {
                                        input.next_token = new_token.cloned();
                                        is_empty
                                    }
                                },
                                Err(_) => true,
                            };
                            if tx.send(resp).await.is_err() {
                                // receiving end was dropped
                                return
                            }
                            if done {
                                return
                            }
                        }
                    }))
                }
            }

/// Paginator for [`ListOperations`](crate::operation::ListOperations)
            pub struct ListOperationsPaginator {
                handle: std::sync::Arc<crate::client::Handle>,
                builder: crate::input::list_operations_input::Builder,
                stop_on_duplicate_token: bool,
            }

            impl ListOperationsPaginator  {
                /// Create a new paginator-wrapper
                pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>, builder: crate::input::list_operations_input::Builder) -> Self {
                    Self {
                        handle,
                        builder,
                        stop_on_duplicate_token: true,
                    }
                }

                /// Set the page size
                ///
                /// _Note: this method will override any previously set value for `max_results`_
                pub fn page_size(mut self, limit: i32) -> Self {
                    self.builder.max_results = Some(limit);
                    self
                }

                

                /// Stop paginating when the service returns the same pagination token twice in a row.
                ///
                /// Defaults to true.
                ///
                /// For certain operations, it may be useful to continue on duplicate token. For example,
                /// if an operation is for tailing a log file in real-time, then continuing may be desired.
                /// This option can be set to `false` to accommodate these use cases.
                pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {
                    self.stop_on_duplicate_token = stop_on_duplicate_token;
                    self
                }

                /// Create the pagination stream
                ///
                /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
                pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::output::ListOperationsOutput, aws_smithy_http::result::SdkError<crate::error::ListOperationsError>>> + Unpin
                 {
                    // Move individual fields out of self for the borrow checker
                    let builder = self.builder;
                    let handle = self.handle;
                    aws_smithy_async::future::fn_stream::FnStream::new(move |tx| Box::pin(async move {
                        // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                        let mut input = match builder.build().map_err(aws_smithy_http::result::SdkError::construction_failure) {
                            Ok(input) => input,
                            Err(e) => { let _ = tx.send(Err(e)).await; return; }
                        };
                        loop {
                            let op = match input.make_operation(&handle.conf)
                                .await
                                .map_err(aws_smithy_http::result::SdkError::construction_failure) {
                                Ok(op) => op,
                                Err(e) => {
                                    let _ = tx.send(Err(e)).await;
                                    return;
                                }
                            };
                            let resp = handle.client.call(op).await;
                            // If the input member is None or it was an error
                            let done = match resp {
                                Ok(ref resp) => {
                                    let new_token = crate::lens::reflens_structure_crate_output_list_operations_output_next_token(resp);
                                    let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                                    if !is_empty && new_token == input.next_token.as_ref() && self.stop_on_duplicate_token {
                                        true
                                    } else {
                                        input.next_token = new_token.cloned();
                                        is_empty
                                    }
                                },
                                Err(_) => true,
                            };
                            if tx.send(resp).await.is_err() {
                                // receiving end was dropped
                                return
                            }
                            if done {
                                return
                            }
                        }
                    }))
                }
            }

/// Paginator for [`ListServices`](crate::operation::ListServices)
            pub struct ListServicesPaginator {
                handle: std::sync::Arc<crate::client::Handle>,
                builder: crate::input::list_services_input::Builder,
                stop_on_duplicate_token: bool,
            }

            impl ListServicesPaginator  {
                /// Create a new paginator-wrapper
                pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>, builder: crate::input::list_services_input::Builder) -> Self {
                    Self {
                        handle,
                        builder,
                        stop_on_duplicate_token: true,
                    }
                }

                /// Set the page size
                ///
                /// _Note: this method will override any previously set value for `max_results`_
                pub fn page_size(mut self, limit: i32) -> Self {
                    self.builder.max_results = Some(limit);
                    self
                }

                

                /// Stop paginating when the service returns the same pagination token twice in a row.
                ///
                /// Defaults to true.
                ///
                /// For certain operations, it may be useful to continue on duplicate token. For example,
                /// if an operation is for tailing a log file in real-time, then continuing may be desired.
                /// This option can be set to `false` to accommodate these use cases.
                pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {
                    self.stop_on_duplicate_token = stop_on_duplicate_token;
                    self
                }

                /// Create the pagination stream
                ///
                /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
                pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::output::ListServicesOutput, aws_smithy_http::result::SdkError<crate::error::ListServicesError>>> + Unpin
                 {
                    // Move individual fields out of self for the borrow checker
                    let builder = self.builder;
                    let handle = self.handle;
                    aws_smithy_async::future::fn_stream::FnStream::new(move |tx| Box::pin(async move {
                        // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                        let mut input = match builder.build().map_err(aws_smithy_http::result::SdkError::construction_failure) {
                            Ok(input) => input,
                            Err(e) => { let _ = tx.send(Err(e)).await; return; }
                        };
                        loop {
                            let op = match input.make_operation(&handle.conf)
                                .await
                                .map_err(aws_smithy_http::result::SdkError::construction_failure) {
                                Ok(op) => op,
                                Err(e) => {
                                    let _ = tx.send(Err(e)).await;
                                    return;
                                }
                            };
                            let resp = handle.client.call(op).await;
                            // If the input member is None or it was an error
                            let done = match resp {
                                Ok(ref resp) => {
                                    let new_token = crate::lens::reflens_structure_crate_output_list_services_output_next_token(resp);
                                    let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                                    if !is_empty && new_token == input.next_token.as_ref() && self.stop_on_duplicate_token {
                                        true
                                    } else {
                                        input.next_token = new_token.cloned();
                                        is_empty
                                    }
                                },
                                Err(_) => true,
                            };
                            if tx.send(resp).await.is_err() {
                                // receiving end was dropped
                                return
                            }
                            if done {
                                return
                            }
                        }
                    }))
                }
            }

