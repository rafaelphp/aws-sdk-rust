// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides information that defines an Amazon Timestream endpoint.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TimestreamSettings {
    /// <p>Database name for the endpoint.</p>
    pub database_name: ::std::option::Option<::std::string::String>,
    /// <p>Set this attribute to specify the length of time to store all of the tables in memory that are migrated into Amazon Timestream from the source database. Time is measured in units of hours. When Timestream data comes in, it first resides in memory for the specified duration, which allows quick access to it.</p>
    pub memory_duration: ::std::option::Option<i32>,
    /// <p>Set this attribute to specify the default magnetic duration applied to the Amazon Timestream tables in days. This is the number of days that records remain in magnetic store before being discarded. For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/storage.html">Storage</a> in the <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/">Amazon Timestream Developer Guide</a>.</p>
    pub magnetic_duration: ::std::option::Option<i32>,
    /// <p>Set this attribute to <code>true</code> to specify that DMS only applies inserts and updates, and not deletes. Amazon Timestream does not allow deleting records, so if this value is <code>false</code>, DMS nulls out the corresponding record in the Timestream database rather than deleting it.</p>
    pub cdc_inserts_and_updates: ::std::option::Option<bool>,
    /// <p>Set this attribute to <code>true</code> to enable memory store writes. When this value is <code>false</code>, DMS does not write records that are older in days than the value specified in <code>MagneticDuration</code>, because Amazon Timestream does not allow memory writes by default. For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/storage.html">Storage</a> in the <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/">Amazon Timestream Developer Guide</a>.</p>
    pub enable_magnetic_store_writes: ::std::option::Option<bool>,
}
impl TimestreamSettings {
    /// <p>Database name for the endpoint.</p>
    pub fn database_name(&self) -> ::std::option::Option<&str> {
        self.database_name.as_deref()
    }
    /// <p>Set this attribute to specify the length of time to store all of the tables in memory that are migrated into Amazon Timestream from the source database. Time is measured in units of hours. When Timestream data comes in, it first resides in memory for the specified duration, which allows quick access to it.</p>
    pub fn memory_duration(&self) -> ::std::option::Option<i32> {
        self.memory_duration
    }
    /// <p>Set this attribute to specify the default magnetic duration applied to the Amazon Timestream tables in days. This is the number of days that records remain in magnetic store before being discarded. For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/storage.html">Storage</a> in the <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/">Amazon Timestream Developer Guide</a>.</p>
    pub fn magnetic_duration(&self) -> ::std::option::Option<i32> {
        self.magnetic_duration
    }
    /// <p>Set this attribute to <code>true</code> to specify that DMS only applies inserts and updates, and not deletes. Amazon Timestream does not allow deleting records, so if this value is <code>false</code>, DMS nulls out the corresponding record in the Timestream database rather than deleting it.</p>
    pub fn cdc_inserts_and_updates(&self) -> ::std::option::Option<bool> {
        self.cdc_inserts_and_updates
    }
    /// <p>Set this attribute to <code>true</code> to enable memory store writes. When this value is <code>false</code>, DMS does not write records that are older in days than the value specified in <code>MagneticDuration</code>, because Amazon Timestream does not allow memory writes by default. For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/storage.html">Storage</a> in the <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/">Amazon Timestream Developer Guide</a>.</p>
    pub fn enable_magnetic_store_writes(&self) -> ::std::option::Option<bool> {
        self.enable_magnetic_store_writes
    }
}
impl TimestreamSettings {
    /// Creates a new builder-style object to manufacture [`TimestreamSettings`](crate::types::TimestreamSettings).
    pub fn builder() -> crate::types::builders::TimestreamSettingsBuilder {
        crate::types::builders::TimestreamSettingsBuilder::default()
    }
}

/// A builder for [`TimestreamSettings`](crate::types::TimestreamSettings).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct TimestreamSettingsBuilder {
    pub(crate) database_name: ::std::option::Option<::std::string::String>,
    pub(crate) memory_duration: ::std::option::Option<i32>,
    pub(crate) magnetic_duration: ::std::option::Option<i32>,
    pub(crate) cdc_inserts_and_updates: ::std::option::Option<bool>,
    pub(crate) enable_magnetic_store_writes: ::std::option::Option<bool>,
}
impl TimestreamSettingsBuilder {
    /// <p>Database name for the endpoint.</p>
    pub fn database_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.database_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Database name for the endpoint.</p>
    pub fn set_database_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.database_name = input;
        self
    }
    /// <p>Database name for the endpoint.</p>
    pub fn get_database_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.database_name
    }
    /// <p>Set this attribute to specify the length of time to store all of the tables in memory that are migrated into Amazon Timestream from the source database. Time is measured in units of hours. When Timestream data comes in, it first resides in memory for the specified duration, which allows quick access to it.</p>
    pub fn memory_duration(mut self, input: i32) -> Self {
        self.memory_duration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Set this attribute to specify the length of time to store all of the tables in memory that are migrated into Amazon Timestream from the source database. Time is measured in units of hours. When Timestream data comes in, it first resides in memory for the specified duration, which allows quick access to it.</p>
    pub fn set_memory_duration(mut self, input: ::std::option::Option<i32>) -> Self {
        self.memory_duration = input;
        self
    }
    /// <p>Set this attribute to specify the length of time to store all of the tables in memory that are migrated into Amazon Timestream from the source database. Time is measured in units of hours. When Timestream data comes in, it first resides in memory for the specified duration, which allows quick access to it.</p>
    pub fn get_memory_duration(&self) -> &::std::option::Option<i32> {
        &self.memory_duration
    }
    /// <p>Set this attribute to specify the default magnetic duration applied to the Amazon Timestream tables in days. This is the number of days that records remain in magnetic store before being discarded. For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/storage.html">Storage</a> in the <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/">Amazon Timestream Developer Guide</a>.</p>
    pub fn magnetic_duration(mut self, input: i32) -> Self {
        self.magnetic_duration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Set this attribute to specify the default magnetic duration applied to the Amazon Timestream tables in days. This is the number of days that records remain in magnetic store before being discarded. For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/storage.html">Storage</a> in the <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/">Amazon Timestream Developer Guide</a>.</p>
    pub fn set_magnetic_duration(mut self, input: ::std::option::Option<i32>) -> Self {
        self.magnetic_duration = input;
        self
    }
    /// <p>Set this attribute to specify the default magnetic duration applied to the Amazon Timestream tables in days. This is the number of days that records remain in magnetic store before being discarded. For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/storage.html">Storage</a> in the <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/">Amazon Timestream Developer Guide</a>.</p>
    pub fn get_magnetic_duration(&self) -> &::std::option::Option<i32> {
        &self.magnetic_duration
    }
    /// <p>Set this attribute to <code>true</code> to specify that DMS only applies inserts and updates, and not deletes. Amazon Timestream does not allow deleting records, so if this value is <code>false</code>, DMS nulls out the corresponding record in the Timestream database rather than deleting it.</p>
    pub fn cdc_inserts_and_updates(mut self, input: bool) -> Self {
        self.cdc_inserts_and_updates = ::std::option::Option::Some(input);
        self
    }
    /// <p>Set this attribute to <code>true</code> to specify that DMS only applies inserts and updates, and not deletes. Amazon Timestream does not allow deleting records, so if this value is <code>false</code>, DMS nulls out the corresponding record in the Timestream database rather than deleting it.</p>
    pub fn set_cdc_inserts_and_updates(mut self, input: ::std::option::Option<bool>) -> Self {
        self.cdc_inserts_and_updates = input;
        self
    }
    /// <p>Set this attribute to <code>true</code> to specify that DMS only applies inserts and updates, and not deletes. Amazon Timestream does not allow deleting records, so if this value is <code>false</code>, DMS nulls out the corresponding record in the Timestream database rather than deleting it.</p>
    pub fn get_cdc_inserts_and_updates(&self) -> &::std::option::Option<bool> {
        &self.cdc_inserts_and_updates
    }
    /// <p>Set this attribute to <code>true</code> to enable memory store writes. When this value is <code>false</code>, DMS does not write records that are older in days than the value specified in <code>MagneticDuration</code>, because Amazon Timestream does not allow memory writes by default. For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/storage.html">Storage</a> in the <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/">Amazon Timestream Developer Guide</a>.</p>
    pub fn enable_magnetic_store_writes(mut self, input: bool) -> Self {
        self.enable_magnetic_store_writes = ::std::option::Option::Some(input);
        self
    }
    /// <p>Set this attribute to <code>true</code> to enable memory store writes. When this value is <code>false</code>, DMS does not write records that are older in days than the value specified in <code>MagneticDuration</code>, because Amazon Timestream does not allow memory writes by default. For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/storage.html">Storage</a> in the <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/">Amazon Timestream Developer Guide</a>.</p>
    pub fn set_enable_magnetic_store_writes(mut self, input: ::std::option::Option<bool>) -> Self {
        self.enable_magnetic_store_writes = input;
        self
    }
    /// <p>Set this attribute to <code>true</code> to enable memory store writes. When this value is <code>false</code>, DMS does not write records that are older in days than the value specified in <code>MagneticDuration</code>, because Amazon Timestream does not allow memory writes by default. For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/storage.html">Storage</a> in the <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/">Amazon Timestream Developer Guide</a>.</p>
    pub fn get_enable_magnetic_store_writes(&self) -> &::std::option::Option<bool> {
        &self.enable_magnetic_store_writes
    }
    /// Consumes the builder and constructs a [`TimestreamSettings`](crate::types::TimestreamSettings).
    pub fn build(self) -> crate::types::TimestreamSettings {
        crate::types::TimestreamSettings {
            database_name: self.database_name,
            memory_duration: self.memory_duration,
            magnetic_duration: self.magnetic_duration,
            cdc_inserts_and_updates: self.cdc_inserts_and_updates,
            enable_magnetic_store_writes: self.enable_magnetic_store_writes,
        }
    }
}
