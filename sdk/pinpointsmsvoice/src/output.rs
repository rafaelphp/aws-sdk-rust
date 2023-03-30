// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// An empty object that indicates that the event destination was updated successfully.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UpdateConfigurationSetEventDestinationOutput  {
}
/// See [`UpdateConfigurationSetEventDestinationOutput`](crate::output::UpdateConfigurationSetEventDestinationOutput).
pub mod update_configuration_set_event_destination_output {
    
    /// A builder for [`UpdateConfigurationSetEventDestinationOutput`](crate::output::UpdateConfigurationSetEventDestinationOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`UpdateConfigurationSetEventDestinationOutput`](crate::output::UpdateConfigurationSetEventDestinationOutput).
        pub fn build(self) -> crate::output::UpdateConfigurationSetEventDestinationOutput {
            crate::output::UpdateConfigurationSetEventDestinationOutput {
            }
        }
    }
    
    
}
impl UpdateConfigurationSetEventDestinationOutput {
    /// Creates a new builder-style object to manufacture [`UpdateConfigurationSetEventDestinationOutput`](crate::output::UpdateConfigurationSetEventDestinationOutput).
    pub fn builder() -> crate::output::update_configuration_set_event_destination_output::Builder {
        crate::output::update_configuration_set_event_destination_output::Builder::default()
    }
}

/// An object that that contains the Message ID of a Voice message that was sent successfully.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct SendVoiceMessageOutput  {
    /// A unique identifier for the voice message.
    #[doc(hidden)]
    pub message_id: std::option::Option<std::string::String>,
}
impl SendVoiceMessageOutput {
    /// A unique identifier for the voice message.
    pub fn message_id(&self) -> std::option::Option<& str> {
        self.message_id.as_deref()
    }
}
/// See [`SendVoiceMessageOutput`](crate::output::SendVoiceMessageOutput).
pub mod send_voice_message_output {
    
    /// A builder for [`SendVoiceMessageOutput`](crate::output::SendVoiceMessageOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// A unique identifier for the voice message.
        pub fn message_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.message_id = Some(input.into());
            self
        }
        /// A unique identifier for the voice message.
        pub fn set_message_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message_id = input; self
        }
        /// Consumes the builder and constructs a [`SendVoiceMessageOutput`](crate::output::SendVoiceMessageOutput).
        pub fn build(self) -> crate::output::SendVoiceMessageOutput {
            crate::output::SendVoiceMessageOutput {
                message_id: self.message_id
                ,
            }
        }
    }
    
    
}
impl SendVoiceMessageOutput {
    /// Creates a new builder-style object to manufacture [`SendVoiceMessageOutput`](crate::output::SendVoiceMessageOutput).
    pub fn builder() -> crate::output::send_voice_message_output::Builder {
        crate::output::send_voice_message_output::Builder::default()
    }
}

/// An object that contains information about the configuration sets for your account in the current region.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListConfigurationSetsOutput  {
    /// An object that contains a list of configuration sets for your account in the current region.
    #[doc(hidden)]
    pub configuration_sets: std::option::Option<std::vec::Vec<std::string::String>>,
    /// A token returned from a previous call to ListConfigurationSets to indicate the position in the list of configuration sets.
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl ListConfigurationSetsOutput {
    /// An object that contains a list of configuration sets for your account in the current region.
    pub fn configuration_sets(&self) -> std::option::Option<& [std::string::String]> {
        self.configuration_sets.as_deref()
    }
    /// A token returned from a previous call to ListConfigurationSets to indicate the position in the list of configuration sets.
    pub fn next_token(&self) -> std::option::Option<& str> {
        self.next_token.as_deref()
    }
}
/// See [`ListConfigurationSetsOutput`](crate::output::ListConfigurationSetsOutput).
pub mod list_configuration_sets_output {
    
    /// A builder for [`ListConfigurationSetsOutput`](crate::output::ListConfigurationSetsOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) configuration_sets: std::option::Option<std::vec::Vec<std::string::String>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `configuration_sets`.
        ///
        /// To override the contents of this collection use [`set_configuration_sets`](Self::set_configuration_sets).
        ///
        /// An object that contains a list of configuration sets for your account in the current region.
        pub fn configuration_sets(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.configuration_sets.unwrap_or_default();
                            v.push(input.into());
                            self.configuration_sets = Some(v);
                            self
        }
        /// An object that contains a list of configuration sets for your account in the current region.
        pub fn set_configuration_sets(mut self, input: std::option::Option<std::vec::Vec<std::string::String>>) -> Self {
            self.configuration_sets = input; self
        }
        /// A token returned from a previous call to ListConfigurationSets to indicate the position in the list of configuration sets.
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// A token returned from a previous call to ListConfigurationSets to indicate the position in the list of configuration sets.
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input; self
        }
        /// Consumes the builder and constructs a [`ListConfigurationSetsOutput`](crate::output::ListConfigurationSetsOutput).
        pub fn build(self) -> crate::output::ListConfigurationSetsOutput {
            crate::output::ListConfigurationSetsOutput {
                configuration_sets: self.configuration_sets
                ,
                next_token: self.next_token
                ,
            }
        }
    }
    
    
}
impl ListConfigurationSetsOutput {
    /// Creates a new builder-style object to manufacture [`ListConfigurationSetsOutput`](crate::output::ListConfigurationSetsOutput).
    pub fn builder() -> crate::output::list_configuration_sets_output::Builder {
        crate::output::list_configuration_sets_output::Builder::default()
    }
}

/// An object that contains information about an event destination.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetConfigurationSetEventDestinationsOutput  {
    /// An array of EventDestination objects. Each EventDestination object includes ARNs and other information that define an event destination.
    #[doc(hidden)]
    pub event_destinations: std::option::Option<std::vec::Vec<crate::model::EventDestination>>,
}
impl GetConfigurationSetEventDestinationsOutput {
    /// An array of EventDestination objects. Each EventDestination object includes ARNs and other information that define an event destination.
    pub fn event_destinations(&self) -> std::option::Option<& [crate::model::EventDestination]> {
        self.event_destinations.as_deref()
    }
}
/// See [`GetConfigurationSetEventDestinationsOutput`](crate::output::GetConfigurationSetEventDestinationsOutput).
pub mod get_configuration_set_event_destinations_output {
    
    /// A builder for [`GetConfigurationSetEventDestinationsOutput`](crate::output::GetConfigurationSetEventDestinationsOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) event_destinations: std::option::Option<std::vec::Vec<crate::model::EventDestination>>,
    }
    impl Builder {
        /// Appends an item to `event_destinations`.
        ///
        /// To override the contents of this collection use [`set_event_destinations`](Self::set_event_destinations).
        ///
        /// An array of EventDestination objects. Each EventDestination object includes ARNs and other information that define an event destination.
        pub fn event_destinations(mut self, input: crate::model::EventDestination) -> Self {
            let mut v = self.event_destinations.unwrap_or_default();
                            v.push(input);
                            self.event_destinations = Some(v);
                            self
        }
        /// An array of EventDestination objects. Each EventDestination object includes ARNs and other information that define an event destination.
        pub fn set_event_destinations(mut self, input: std::option::Option<std::vec::Vec<crate::model::EventDestination>>) -> Self {
            self.event_destinations = input; self
        }
        /// Consumes the builder and constructs a [`GetConfigurationSetEventDestinationsOutput`](crate::output::GetConfigurationSetEventDestinationsOutput).
        pub fn build(self) -> crate::output::GetConfigurationSetEventDestinationsOutput {
            crate::output::GetConfigurationSetEventDestinationsOutput {
                event_destinations: self.event_destinations
                ,
            }
        }
    }
    
    
}
impl GetConfigurationSetEventDestinationsOutput {
    /// Creates a new builder-style object to manufacture [`GetConfigurationSetEventDestinationsOutput`](crate::output::GetConfigurationSetEventDestinationsOutput).
    pub fn builder() -> crate::output::get_configuration_set_event_destinations_output::Builder {
        crate::output::get_configuration_set_event_destinations_output::Builder::default()
    }
}

/// An empty object that indicates that the event destination was deleted successfully.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteConfigurationSetEventDestinationOutput  {
}
/// See [`DeleteConfigurationSetEventDestinationOutput`](crate::output::DeleteConfigurationSetEventDestinationOutput).
pub mod delete_configuration_set_event_destination_output {
    
    /// A builder for [`DeleteConfigurationSetEventDestinationOutput`](crate::output::DeleteConfigurationSetEventDestinationOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteConfigurationSetEventDestinationOutput`](crate::output::DeleteConfigurationSetEventDestinationOutput).
        pub fn build(self) -> crate::output::DeleteConfigurationSetEventDestinationOutput {
            crate::output::DeleteConfigurationSetEventDestinationOutput {
            }
        }
    }
    
    
}
impl DeleteConfigurationSetEventDestinationOutput {
    /// Creates a new builder-style object to manufacture [`DeleteConfigurationSetEventDestinationOutput`](crate::output::DeleteConfigurationSetEventDestinationOutput).
    pub fn builder() -> crate::output::delete_configuration_set_event_destination_output::Builder {
        crate::output::delete_configuration_set_event_destination_output::Builder::default()
    }
}

/// An empty object that indicates that the configuration set was deleted successfully.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteConfigurationSetOutput  {
}
/// See [`DeleteConfigurationSetOutput`](crate::output::DeleteConfigurationSetOutput).
pub mod delete_configuration_set_output {
    
    /// A builder for [`DeleteConfigurationSetOutput`](crate::output::DeleteConfigurationSetOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteConfigurationSetOutput`](crate::output::DeleteConfigurationSetOutput).
        pub fn build(self) -> crate::output::DeleteConfigurationSetOutput {
            crate::output::DeleteConfigurationSetOutput {
            }
        }
    }
    
    
}
impl DeleteConfigurationSetOutput {
    /// Creates a new builder-style object to manufacture [`DeleteConfigurationSetOutput`](crate::output::DeleteConfigurationSetOutput).
    pub fn builder() -> crate::output::delete_configuration_set_output::Builder {
        crate::output::delete_configuration_set_output::Builder::default()
    }
}

/// An empty object that indicates that the event destination was created successfully.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateConfigurationSetEventDestinationOutput  {
}
/// See [`CreateConfigurationSetEventDestinationOutput`](crate::output::CreateConfigurationSetEventDestinationOutput).
pub mod create_configuration_set_event_destination_output {
    
    /// A builder for [`CreateConfigurationSetEventDestinationOutput`](crate::output::CreateConfigurationSetEventDestinationOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`CreateConfigurationSetEventDestinationOutput`](crate::output::CreateConfigurationSetEventDestinationOutput).
        pub fn build(self) -> crate::output::CreateConfigurationSetEventDestinationOutput {
            crate::output::CreateConfigurationSetEventDestinationOutput {
            }
        }
    }
    
    
}
impl CreateConfigurationSetEventDestinationOutput {
    /// Creates a new builder-style object to manufacture [`CreateConfigurationSetEventDestinationOutput`](crate::output::CreateConfigurationSetEventDestinationOutput).
    pub fn builder() -> crate::output::create_configuration_set_event_destination_output::Builder {
        crate::output::create_configuration_set_event_destination_output::Builder::default()
    }
}

/// An empty object that indicates that the configuration set was successfully created.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateConfigurationSetOutput  {
}
/// See [`CreateConfigurationSetOutput`](crate::output::CreateConfigurationSetOutput).
pub mod create_configuration_set_output {
    
    /// A builder for [`CreateConfigurationSetOutput`](crate::output::CreateConfigurationSetOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`CreateConfigurationSetOutput`](crate::output::CreateConfigurationSetOutput).
        pub fn build(self) -> crate::output::CreateConfigurationSetOutput {
            crate::output::CreateConfigurationSetOutput {
            }
        }
    }
    
    
}
impl CreateConfigurationSetOutput {
    /// Creates a new builder-style object to manufacture [`CreateConfigurationSetOutput`](crate::output::CreateConfigurationSetOutput).
    pub fn builder() -> crate::output::create_configuration_set_output::Builder {
        crate::output::create_configuration_set_output::Builder::default()
    }
}

