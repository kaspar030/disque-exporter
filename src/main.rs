use disque::{Disque, FromRedisValue};

use metrics::{absolute_counter, gauge};
use rouille::router;

use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::env;

fn main() {
    let listen_address = env::var("DISQUE_EXPORTER_LISTEN_ADDR").unwrap_or("127.0.0.1:8000".into());
    let disque_url =
        env::var("DISQUE_EXPORTER_DISQUE_URL").unwrap_or("redis://localhost:7711".into());

    tracing_subscriber::fmt::init();

    let builder = PrometheusBuilder::new();
    let handle = builder
        .install_recorder()
        .expect("failed to install Prometheus recorder");

    rouille::start_server(listen_address, move |request| {
        router!(request,
            (GET) (/metrics) => {
                rouille::Response::text(disque_queue_metrics(&handle, &disque_url))
            },
            _ => rouille::Response::empty_404()
        )
    });

    fn disque_queue_metrics(handle: &PrometheusHandle, disque_url: &str) -> String {
        let disque = Disque::open(disque_url).unwrap();
        let queues = disque.qscan(0, 128, true, None, None, None).unwrap();
        for queue in queues {
            let stats = disque.qstat(&queue).unwrap();
            let name = String::from_redis_value(stats.get("name").unwrap()).unwrap();

            let jobs_in = u64::from_redis_value(stats.get("jobs-in").unwrap()).unwrap();
            let jobs_out = u64::from_redis_value(stats.get("jobs-out").unwrap()).unwrap();
            let age = u64::from_redis_value(stats.get("age").unwrap()).unwrap() as f64;
            let idle = u64::from_redis_value(stats.get("idle").unwrap()).unwrap() as f64;
            let blocked = u64::from_redis_value(stats.get("blocked").unwrap()).unwrap() as f64;

            absolute_counter!("queue_jobs_in", jobs_in, "queue" => name.clone());
            absolute_counter!("queue_jobs_out", jobs_out, "queue" => name.clone());

            gauge!("queue_age", age, "queue" => name.clone());
            gauge!("queue_blocked", blocked, "queue" => name.clone());
            gauge!("queue_idle", idle, "queue" => name.clone());
        }
        handle.render()
    }
}
