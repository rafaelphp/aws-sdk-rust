// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The output from the UpdateThingShadow operation.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UpdateThingShadowOutput  {
    /// <p>The state information, in JSON format.</p>
    #[doc(hidden)]
    pub payload: std::option::Option<aws_smithy_types::Blob>,
}
impl UpdateThingShadowOutput {
    /// <p>The state information, in JSON format.</p>
    pub fn payload(&self) -> std::option::Option<& aws_smithy_types::Blob> {
        self.payload.as_ref()
    }
}
/// See [`UpdateThingShadowOutput`](crate::output::UpdateThingShadowOutput).
pub mod update_thing_shadow_output {
    
    /// A builder for [`UpdateThingShadowOutput`](crate::output::UpdateThingShadowOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) payload: std::option::Option<aws_smithy_types::Blob>,
    }
    impl Builder {
        /// <p>The state information, in JSON format.</p>
        pub fn payload(mut self, input: aws_smithy_types::Blob) -> Self {
            self.payload = Some(input);
            self
        }
        /// <p>The state information, in JSON format.</p>
        pub fn set_payload(mut self, input: std::option::Option<aws_smithy_types::Blob>) -> Self {
            self.payload = input; self
        }
        /// Consumes the builder and constructs a [`UpdateThingShadowOutput`](crate::output::UpdateThingShadowOutput).
        pub fn build(self) -> crate::output::UpdateThingShadowOutput {
            crate::output::UpdateThingShadowOutput {
                payload: self.payload
                ,
            }
        }
    }
    
    
}
impl UpdateThingShadowOutput {
    /// Creates a new builder-style object to manufacture [`UpdateThingShadowOutput`](crate::output::UpdateThingShadowOutput).
    pub fn builder() -> crate::output::update_thing_shadow_output::Builder {
        crate::output::update_thing_shadow_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct PublishOutput  {
}
/// See [`PublishOutput`](crate::output::PublishOutput).
pub mod publish_output {
    
    /// A builder for [`PublishOutput`](crate::output::PublishOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`PublishOutput`](crate::output::PublishOutput).
        pub fn build(self) -> crate::output::PublishOutput {
            crate::output::PublishOutput {
            }
        }
    }
    
    
}
impl PublishOutput {
    /// Creates a new builder-style object to manufacture [`PublishOutput`](crate::output::PublishOutput).
    pub fn builder() -> crate::output::publish_output::Builder {
        crate::output::publish_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListRetainedMessagesOutput  {
    /// <p>A summary list the account's retained messages. The information returned doesn't include the message payloads of the retained messages.</p>
    #[doc(hidden)]
    pub retained_topics: std::option::Option<std::vec::Vec<crate::model::RetainedMessageSummary>>,
    /// <p>The token for the next set of results, or null if there are no additional results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl ListRetainedMessagesOutput {
    /// <p>A summary list the account's retained messages. The information returned doesn't include the message payloads of the retained messages.</p>
    pub fn retained_topics(&self) -> std::option::Option<& [crate::model::RetainedMessageSummary]> {
        self.retained_topics.as_deref()
    }
    /// <p>The token for the next set of results, or null if there are no additional results.</p>
    pub fn next_token(&self) -> std::option::Option<& str> {
        self.next_token.as_deref()
    }
}
/// See [`ListRetainedMessagesOutput`](crate::output::ListRetainedMessagesOutput).
pub mod list_retained_messages_output {
    
    /// A builder for [`ListRetainedMessagesOutput`](crate::output::ListRetainedMessagesOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) retained_topics: std::option::Option<std::vec::Vec<crate::model::RetainedMessageSummary>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `retained_topics`.
        ///
        /// To override the contents of this collection use [`set_retained_topics`](Self::set_retained_topics).
        ///
        /// <p>A summary list the account's retained messages. The information returned doesn't include the message payloads of the retained messages.</p>
        pub fn retained_topics(mut self, input: crate::model::RetainedMessageSummary) -> Self {
            let mut v = self.retained_topics.unwrap_or_default();
                            v.push(input);
                            self.retained_topics = Some(v);
                            self
        }
        /// <p>A summary list the account's retained messages. The information returned doesn't include the message payloads of the retained messages.</p>
        pub fn set_retained_topics(mut self, input: std::option::Option<std::vec::Vec<crate::model::RetainedMessageSummary>>) -> Self {
            self.retained_topics = input; self
        }
        /// <p>The token for the next set of results, or null if there are no additional results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The token for the next set of results, or null if there are no additional results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input; self
        }
        /// Consumes the builder and constructs a [`ListRetainedMessagesOutput`](crate::output::ListRetainedMessagesOutput).
        pub fn build(self) -> crate::output::ListRetainedMessagesOutput {
            crate::output::ListRetainedMessagesOutput {
                retained_topics: self.retained_topics
                ,
                next_token: self.next_token
                ,
            }
        }
    }
    
    
}
impl ListRetainedMessagesOutput {
    /// Creates a new builder-style object to manufacture [`ListRetainedMessagesOutput`](crate::output::ListRetainedMessagesOutput).
    pub fn builder() -> crate::output::list_retained_messages_output::Builder {
        crate::output::list_retained_messages_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListNamedShadowsForThingOutput  {
    /// <p>The list of shadows for the specified thing.</p>
    #[doc(hidden)]
    pub results: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The token to use to get the next set of results, or <b>null</b> if there are no additional results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The Epoch date and time the response was generated by IoT.</p>
    #[doc(hidden)]
    pub timestamp: i64,
}
impl ListNamedShadowsForThingOutput {
    /// <p>The list of shadows for the specified thing.</p>
    pub fn results(&self) -> std::option::Option<& [std::string::String]> {
        self.results.as_deref()
    }
    /// <p>The token to use to get the next set of results, or <b>null</b> if there are no additional results.</p>
    pub fn next_token(&self) -> std::option::Option<& str> {
        self.next_token.as_deref()
    }
    /// <p>The Epoch date and time the response was generated by IoT.</p>
    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }
}
/// See [`ListNamedShadowsForThingOutput`](crate::output::ListNamedShadowsForThingOutput).
pub mod list_named_shadows_for_thing_output {
    
    /// A builder for [`ListNamedShadowsForThingOutput`](crate::output::ListNamedShadowsForThingOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) results: std::option::Option<std::vec::Vec<std::string::String>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
        pub(crate) timestamp: std::option::Option<i64>,
    }
    impl Builder {
        /// Appends an item to `results`.
        ///
        /// To override the contents of this collection use [`set_results`](Self::set_results).
        ///
        /// <p>The list of shadows for the specified thing.</p>
        pub fn results(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.results.unwrap_or_default();
                            v.push(input.into());
                            self.results = Some(v);
                            self
        }
        /// <p>The list of shadows for the specified thing.</p>
        pub fn set_results(mut self, input: std::option::Option<std::vec::Vec<std::string::String>>) -> Self {
            self.results = input; self
        }
        /// <p>The token to use to get the next set of results, or <b>null</b> if there are no additional results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The token to use to get the next set of results, or <b>null</b> if there are no additional results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input; self
        }
        /// <p>The Epoch date and time the response was generated by IoT.</p>
        pub fn timestamp(mut self, input: i64) -> Self {
            self.timestamp = Some(input);
            self
        }
        /// <p>The Epoch date and time the response was generated by IoT.</p>
        pub fn set_timestamp(mut self, input: std::option::Option<i64>) -> Self {
            self.timestamp = input; self
        }
        /// Consumes the builder and constructs a [`ListNamedShadowsForThingOutput`](crate::output::ListNamedShadowsForThingOutput).
        pub fn build(self) -> crate::output::ListNamedShadowsForThingOutput {
            crate::output::ListNamedShadowsForThingOutput {
                results: self.results
                ,
                next_token: self.next_token
                ,
                timestamp: self.timestamp
                    .unwrap_or_default()
                ,
            }
        }
    }
    
    
}
impl ListNamedShadowsForThingOutput {
    /// Creates a new builder-style object to manufacture [`ListNamedShadowsForThingOutput`](crate::output::ListNamedShadowsForThingOutput).
    pub fn builder() -> crate::output::list_named_shadows_for_thing_output::Builder {
        crate::output::list_named_shadows_for_thing_output::Builder::default()
    }
}

/// <p>The output from the GetThingShadow operation.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetThingShadowOutput  {
    /// <p>The state information, in JSON format.</p>
    #[doc(hidden)]
    pub payload: std::option::Option<aws_smithy_types::Blob>,
}
impl GetThingShadowOutput {
    /// <p>The state information, in JSON format.</p>
    pub fn payload(&self) -> std::option::Option<& aws_smithy_types::Blob> {
        self.payload.as_ref()
    }
}
/// See [`GetThingShadowOutput`](crate::output::GetThingShadowOutput).
pub mod get_thing_shadow_output {
    
    /// A builder for [`GetThingShadowOutput`](crate::output::GetThingShadowOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) payload: std::option::Option<aws_smithy_types::Blob>,
    }
    impl Builder {
        /// <p>The state information, in JSON format.</p>
        pub fn payload(mut self, input: aws_smithy_types::Blob) -> Self {
            self.payload = Some(input);
            self
        }
        /// <p>The state information, in JSON format.</p>
        pub fn set_payload(mut self, input: std::option::Option<aws_smithy_types::Blob>) -> Self {
            self.payload = input; self
        }
        /// Consumes the builder and constructs a [`GetThingShadowOutput`](crate::output::GetThingShadowOutput).
        pub fn build(self) -> crate::output::GetThingShadowOutput {
            crate::output::GetThingShadowOutput {
                payload: self.payload
                ,
            }
        }
    }
    
    
}
impl GetThingShadowOutput {
    /// Creates a new builder-style object to manufacture [`GetThingShadowOutput`](crate::output::GetThingShadowOutput).
    pub fn builder() -> crate::output::get_thing_shadow_output::Builder {
        crate::output::get_thing_shadow_output::Builder::default()
    }
}

/// <p>The output from the GetRetainedMessage operation.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetRetainedMessageOutput  {
    /// <p>The topic name to which the retained message was published.</p>
    #[doc(hidden)]
    pub topic: std::option::Option<std::string::String>,
    /// <p>The Base64-encoded message payload of the retained message body.</p>
    #[doc(hidden)]
    pub payload: std::option::Option<aws_smithy_types::Blob>,
    /// <p>The quality of service (QoS) level used to publish the retained message.</p>
    #[doc(hidden)]
    pub qos: i32,
    /// <p>The Epoch date and time, in milliseconds, when the retained message was stored by IoT.</p>
    #[doc(hidden)]
    pub last_modified_time: i64,
}
impl GetRetainedMessageOutput {
    /// <p>The topic name to which the retained message was published.</p>
    pub fn topic(&self) -> std::option::Option<& str> {
        self.topic.as_deref()
    }
    /// <p>The Base64-encoded message payload of the retained message body.</p>
    pub fn payload(&self) -> std::option::Option<& aws_smithy_types::Blob> {
        self.payload.as_ref()
    }
    /// <p>The quality of service (QoS) level used to publish the retained message.</p>
    pub fn qos(&self) -> i32 {
        self.qos
    }
    /// <p>The Epoch date and time, in milliseconds, when the retained message was stored by IoT.</p>
    pub fn last_modified_time(&self) -> i64 {
        self.last_modified_time
    }
}
/// See [`GetRetainedMessageOutput`](crate::output::GetRetainedMessageOutput).
pub mod get_retained_message_output {
    
    /// A builder for [`GetRetainedMessageOutput`](crate::output::GetRetainedMessageOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) topic: std::option::Option<std::string::String>,
        pub(crate) payload: std::option::Option<aws_smithy_types::Blob>,
        pub(crate) qos: std::option::Option<i32>,
        pub(crate) last_modified_time: std::option::Option<i64>,
    }
    impl Builder {
        /// <p>The topic name to which the retained message was published.</p>
        pub fn topic(mut self, input: impl Into<std::string::String>) -> Self {
            self.topic = Some(input.into());
            self
        }
        /// <p>The topic name to which the retained message was published.</p>
        pub fn set_topic(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.topic = input; self
        }
        /// <p>The Base64-encoded message payload of the retained message body.</p>
        pub fn payload(mut self, input: aws_smithy_types::Blob) -> Self {
            self.payload = Some(input);
            self
        }
        /// <p>The Base64-encoded message payload of the retained message body.</p>
        pub fn set_payload(mut self, input: std::option::Option<aws_smithy_types::Blob>) -> Self {
            self.payload = input; self
        }
        /// <p>The quality of service (QoS) level used to publish the retained message.</p>
        pub fn qos(mut self, input: i32) -> Self {
            self.qos = Some(input);
            self
        }
        /// <p>The quality of service (QoS) level used to publish the retained message.</p>
        pub fn set_qos(mut self, input: std::option::Option<i32>) -> Self {
            self.qos = input; self
        }
        /// <p>The Epoch date and time, in milliseconds, when the retained message was stored by IoT.</p>
        pub fn last_modified_time(mut self, input: i64) -> Self {
            self.last_modified_time = Some(input);
            self
        }
        /// <p>The Epoch date and time, in milliseconds, when the retained message was stored by IoT.</p>
        pub fn set_last_modified_time(mut self, input: std::option::Option<i64>) -> Self {
            self.last_modified_time = input; self
        }
        /// Consumes the builder and constructs a [`GetRetainedMessageOutput`](crate::output::GetRetainedMessageOutput).
        pub fn build(self) -> crate::output::GetRetainedMessageOutput {
            crate::output::GetRetainedMessageOutput {
                topic: self.topic
                ,
                payload: self.payload
                ,
                qos: self.qos
                    .unwrap_or_default()
                ,
                last_modified_time: self.last_modified_time
                    .unwrap_or_default()
                ,
            }
        }
    }
    
    
}
impl GetRetainedMessageOutput {
    /// Creates a new builder-style object to manufacture [`GetRetainedMessageOutput`](crate::output::GetRetainedMessageOutput).
    pub fn builder() -> crate::output::get_retained_message_output::Builder {
        crate::output::get_retained_message_output::Builder::default()
    }
}

/// <p>The output from the DeleteThingShadow operation.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteThingShadowOutput  {
    /// <p>The state information, in JSON format.</p>
    #[doc(hidden)]
    pub payload: std::option::Option<aws_smithy_types::Blob>,
}
impl DeleteThingShadowOutput {
    /// <p>The state information, in JSON format.</p>
    pub fn payload(&self) -> std::option::Option<& aws_smithy_types::Blob> {
        self.payload.as_ref()
    }
}
/// See [`DeleteThingShadowOutput`](crate::output::DeleteThingShadowOutput).
pub mod delete_thing_shadow_output {
    
    /// A builder for [`DeleteThingShadowOutput`](crate::output::DeleteThingShadowOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) payload: std::option::Option<aws_smithy_types::Blob>,
    }
    impl Builder {
        /// <p>The state information, in JSON format.</p>
        pub fn payload(mut self, input: aws_smithy_types::Blob) -> Self {
            self.payload = Some(input);
            self
        }
        /// <p>The state information, in JSON format.</p>
        pub fn set_payload(mut self, input: std::option::Option<aws_smithy_types::Blob>) -> Self {
            self.payload = input; self
        }
        /// Consumes the builder and constructs a [`DeleteThingShadowOutput`](crate::output::DeleteThingShadowOutput).
        pub fn build(self) -> crate::output::DeleteThingShadowOutput {
            crate::output::DeleteThingShadowOutput {
                payload: self.payload
                ,
            }
        }
    }
    
    
}
impl DeleteThingShadowOutput {
    /// Creates a new builder-style object to manufacture [`DeleteThingShadowOutput`](crate::output::DeleteThingShadowOutput).
    pub fn builder() -> crate::output::delete_thing_shadow_output::Builder {
        crate::output::delete_thing_shadow_output::Builder::default()
    }
}

