// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchGetCrawlers`](crate::operation::batch_get_crawlers::builders::BatchGetCrawlersFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`crawler_names(impl Into<String>)`](crate::operation::batch_get_crawlers::builders::BatchGetCrawlersFluentBuilder::crawler_names) / [`set_crawler_names(Option<Vec<String>>)`](crate::operation::batch_get_crawlers::builders::BatchGetCrawlersFluentBuilder::set_crawler_names): <p>A list of crawler names, which might be the names returned from the <code>ListCrawlers</code> operation.</p>
    /// - On success, responds with [`BatchGetCrawlersOutput`](crate::operation::batch_get_crawlers::BatchGetCrawlersOutput) with field(s):
    ///   - [`crawlers(Option<Vec<Crawler>>)`](crate::operation::batch_get_crawlers::BatchGetCrawlersOutput::crawlers): <p>A list of crawler definitions.</p>
    ///   - [`crawlers_not_found(Option<Vec<String>>)`](crate::operation::batch_get_crawlers::BatchGetCrawlersOutput::crawlers_not_found): <p>A list of names of crawlers that were not found.</p>
    /// - On failure, responds with [`SdkError<BatchGetCrawlersError>`](crate::operation::batch_get_crawlers::BatchGetCrawlersError)
    pub fn batch_get_crawlers(&self) -> crate::operation::batch_get_crawlers::builders::BatchGetCrawlersFluentBuilder {
        crate::operation::batch_get_crawlers::builders::BatchGetCrawlersFluentBuilder::new(self.handle.clone())
    }
}
