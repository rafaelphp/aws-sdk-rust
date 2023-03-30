// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Paginator for [`DescribeLoadBalancers`](crate::operation::DescribeLoadBalancers)
            pub struct DescribeLoadBalancersPaginator {
                handle: std::sync::Arc<crate::client::Handle>,
                builder: crate::input::describe_load_balancers_input::Builder,
                stop_on_duplicate_token: bool,
            }

            impl DescribeLoadBalancersPaginator  {
                /// Create a new paginator-wrapper
                pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>, builder: crate::input::describe_load_balancers_input::Builder) -> Self {
                    Self {
                        handle,
                        builder,
                        stop_on_duplicate_token: true,
                    }
                }

                

                /// Create a flattened paginator
                    ///
                    /// This paginator automatically flattens results using `load_balancer_descriptions`. Queries to the underlying service
                    /// are dispatched lazily.
                    pub fn items(self) -> crate::paginator::DescribeLoadBalancersPaginatorItems {
                        crate::paginator::DescribeLoadBalancersPaginatorItems(self)
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
                pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::output::DescribeLoadBalancersOutput, aws_smithy_http::result::SdkError<crate::error::DescribeLoadBalancersError>>> + Unpin
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
                                    let new_token = crate::lens::reflens_structure_crate_output_describe_load_balancers_output_next_marker(resp);
                                    let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                                    if !is_empty && new_token == input.marker.as_ref() && self.stop_on_duplicate_token {
                                        true
                                    } else {
                                        input.marker = new_token.cloned();
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

/// Flattened paginator for `DescribeLoadBalancersPaginator`
                ///
                /// This is created with [`.items()`](DescribeLoadBalancersPaginator::items)
                pub struct DescribeLoadBalancersPaginatorItems(DescribeLoadBalancersPaginator);

                impl  DescribeLoadBalancersPaginatorItems  {
                    /// Create the pagination stream
                    ///
                    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
                    ///
                    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
                    pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::model::LoadBalancerDescription, aws_smithy_http::result::SdkError<crate::error::DescribeLoadBalancersError>>> + Unpin
                     {
                        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_structure_crate_output_describe_load_balancers_output_load_balancer_descriptions(page).unwrap_or_default().into_iter())
                    }
                }

