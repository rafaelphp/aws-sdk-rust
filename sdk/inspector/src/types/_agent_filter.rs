// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about an Amazon Inspector agent. This data type is used as a request parameter in the <code>ListAssessmentRunAgents</code> action.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AgentFilter {
    /// <p>The current health state of the agent. Values can be set to <b>HEALTHY</b> or <b>UNHEALTHY</b>.</p>
    pub agent_healths: ::std::option::Option<::std::vec::Vec<crate::types::AgentHealth>>,
    /// <p>The detailed health state of the agent. Values can be set to <b>IDLE</b>, <b>RUNNING</b>, <b>SHUTDOWN</b>, <b>UNHEALTHY</b>, <b>THROTTLED</b>, and <b>UNKNOWN</b>. </p>
    pub agent_health_codes: ::std::option::Option<::std::vec::Vec<crate::types::AgentHealthCode>>,
}
impl AgentFilter {
    /// <p>The current health state of the agent. Values can be set to <b>HEALTHY</b> or <b>UNHEALTHY</b>.</p>
    pub fn agent_healths(&self) -> ::std::option::Option<&[crate::types::AgentHealth]> {
        self.agent_healths.as_deref()
    }
    /// <p>The detailed health state of the agent. Values can be set to <b>IDLE</b>, <b>RUNNING</b>, <b>SHUTDOWN</b>, <b>UNHEALTHY</b>, <b>THROTTLED</b>, and <b>UNKNOWN</b>. </p>
    pub fn agent_health_codes(&self) -> ::std::option::Option<&[crate::types::AgentHealthCode]> {
        self.agent_health_codes.as_deref()
    }
}
impl AgentFilter {
    /// Creates a new builder-style object to manufacture [`AgentFilter`](crate::types::AgentFilter).
    pub fn builder() -> crate::types::builders::AgentFilterBuilder {
        crate::types::builders::AgentFilterBuilder::default()
    }
}

/// A builder for [`AgentFilter`](crate::types::AgentFilter).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AgentFilterBuilder {
    pub(crate) agent_healths: ::std::option::Option<::std::vec::Vec<crate::types::AgentHealth>>,
    pub(crate) agent_health_codes: ::std::option::Option<::std::vec::Vec<crate::types::AgentHealthCode>>,
}
impl AgentFilterBuilder {
    /// Appends an item to `agent_healths`.
    ///
    /// To override the contents of this collection use [`set_agent_healths`](Self::set_agent_healths).
    ///
    /// <p>The current health state of the agent. Values can be set to <b>HEALTHY</b> or <b>UNHEALTHY</b>.</p>
    pub fn agent_healths(mut self, input: crate::types::AgentHealth) -> Self {
        let mut v = self.agent_healths.unwrap_or_default();
        v.push(input);
        self.agent_healths = ::std::option::Option::Some(v);
        self
    }
    /// <p>The current health state of the agent. Values can be set to <b>HEALTHY</b> or <b>UNHEALTHY</b>.</p>
    pub fn set_agent_healths(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AgentHealth>>) -> Self {
        self.agent_healths = input;
        self
    }
    /// <p>The current health state of the agent. Values can be set to <b>HEALTHY</b> or <b>UNHEALTHY</b>.</p>
    pub fn get_agent_healths(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AgentHealth>> {
        &self.agent_healths
    }
    /// Appends an item to `agent_health_codes`.
    ///
    /// To override the contents of this collection use [`set_agent_health_codes`](Self::set_agent_health_codes).
    ///
    /// <p>The detailed health state of the agent. Values can be set to <b>IDLE</b>, <b>RUNNING</b>, <b>SHUTDOWN</b>, <b>UNHEALTHY</b>, <b>THROTTLED</b>, and <b>UNKNOWN</b>. </p>
    pub fn agent_health_codes(mut self, input: crate::types::AgentHealthCode) -> Self {
        let mut v = self.agent_health_codes.unwrap_or_default();
        v.push(input);
        self.agent_health_codes = ::std::option::Option::Some(v);
        self
    }
    /// <p>The detailed health state of the agent. Values can be set to <b>IDLE</b>, <b>RUNNING</b>, <b>SHUTDOWN</b>, <b>UNHEALTHY</b>, <b>THROTTLED</b>, and <b>UNKNOWN</b>. </p>
    pub fn set_agent_health_codes(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AgentHealthCode>>) -> Self {
        self.agent_health_codes = input;
        self
    }
    /// <p>The detailed health state of the agent. Values can be set to <b>IDLE</b>, <b>RUNNING</b>, <b>SHUTDOWN</b>, <b>UNHEALTHY</b>, <b>THROTTLED</b>, and <b>UNKNOWN</b>. </p>
    pub fn get_agent_health_codes(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AgentHealthCode>> {
        &self.agent_health_codes
    }
    /// Consumes the builder and constructs a [`AgentFilter`](crate::types::AgentFilter).
    pub fn build(self) -> crate::types::AgentFilter {
        crate::types::AgentFilter {
            agent_healths: self.agent_healths,
            agent_health_codes: self.agent_health_codes,
        }
    }
}
