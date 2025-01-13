use std::future::Future;

use crate::time::Instant;

/// Returns a wrapped future that records the time it takes to complete the future as a histogram metric.
pub async fn timed<Fut>(future: Fut, operation: &'static str, route: &'static str) -> Fut::Output
where
    Fut: Future,
{
    let start = Instant::now();
    let out = future.await;
    metrics::histogram!("riot_api", "operation" => operation, "route" => route)
        .record(start.elapsed());
    out
}
