// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeAlarmsForMetric`](crate::operation::describe_alarms_for_metric::builders::DescribeAlarmsForMetricFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`metric_name(impl Into<String>)`](crate::operation::describe_alarms_for_metric::builders::DescribeAlarmsForMetricFluentBuilder::metric_name) / [`set_metric_name(Option<String>)`](crate::operation::describe_alarms_for_metric::builders::DescribeAlarmsForMetricFluentBuilder::set_metric_name): <p>The name of the metric.</p>
    ///   - [`namespace(impl Into<String>)`](crate::operation::describe_alarms_for_metric::builders::DescribeAlarmsForMetricFluentBuilder::namespace) / [`set_namespace(Option<String>)`](crate::operation::describe_alarms_for_metric::builders::DescribeAlarmsForMetricFluentBuilder::set_namespace): <p>The namespace of the metric.</p>
    ///   - [`statistic(Statistic)`](crate::operation::describe_alarms_for_metric::builders::DescribeAlarmsForMetricFluentBuilder::statistic) / [`set_statistic(Option<Statistic>)`](crate::operation::describe_alarms_for_metric::builders::DescribeAlarmsForMetricFluentBuilder::set_statistic): <p>The statistic for the metric, other than percentiles. For percentile statistics, use <code>ExtendedStatistics</code>.</p>
    ///   - [`extended_statistic(impl Into<String>)`](crate::operation::describe_alarms_for_metric::builders::DescribeAlarmsForMetricFluentBuilder::extended_statistic) / [`set_extended_statistic(Option<String>)`](crate::operation::describe_alarms_for_metric::builders::DescribeAlarmsForMetricFluentBuilder::set_extended_statistic): <p>The percentile statistic for the metric. Specify a value between p0.0 and p100.</p>
    ///   - [`dimensions(Dimension)`](crate::operation::describe_alarms_for_metric::builders::DescribeAlarmsForMetricFluentBuilder::dimensions) / [`set_dimensions(Option<Vec<Dimension>>)`](crate::operation::describe_alarms_for_metric::builders::DescribeAlarmsForMetricFluentBuilder::set_dimensions): <p>The dimensions associated with the metric. If the metric has any associated dimensions, you must specify them in order for the call to succeed.</p>
    ///   - [`period(i32)`](crate::operation::describe_alarms_for_metric::builders::DescribeAlarmsForMetricFluentBuilder::period) / [`set_period(Option<i32>)`](crate::operation::describe_alarms_for_metric::builders::DescribeAlarmsForMetricFluentBuilder::set_period): <p>The period, in seconds, over which the statistic is applied.</p>
    ///   - [`unit(StandardUnit)`](crate::operation::describe_alarms_for_metric::builders::DescribeAlarmsForMetricFluentBuilder::unit) / [`set_unit(Option<StandardUnit>)`](crate::operation::describe_alarms_for_metric::builders::DescribeAlarmsForMetricFluentBuilder::set_unit): <p>The unit for the metric.</p>
    /// - On success, responds with [`DescribeAlarmsForMetricOutput`](crate::operation::describe_alarms_for_metric::DescribeAlarmsForMetricOutput) with field(s):
    ///   - [`metric_alarms(Option<Vec<MetricAlarm>>)`](crate::operation::describe_alarms_for_metric::DescribeAlarmsForMetricOutput::metric_alarms): <p>The information for each alarm with the specified metric.</p>
    /// - On failure, responds with [`SdkError<DescribeAlarmsForMetricError>`](crate::operation::describe_alarms_for_metric::DescribeAlarmsForMetricError)
    pub fn describe_alarms_for_metric(&self) -> crate::operation::describe_alarms_for_metric::builders::DescribeAlarmsForMetricFluentBuilder {
        crate::operation::describe_alarms_for_metric::builders::DescribeAlarmsForMetricFluentBuilder::new(self.handle.clone())
    }
}
