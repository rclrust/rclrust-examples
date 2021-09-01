use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

use rclrust::{qos::QoSProfile, rclrust_info};
use rclrust_msg::std_msgs::msg::String as String_;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ctx = rclrust::init()?;
    let mut node = ctx.create_node("minimal_publisher")?;
    let logger = node.logger();
    let count = AtomicUsize::new(0);
    let publisher = node.create_publisher::<String_>("topic", &QoSProfile::default())?;

    let _timer = node.create_wall_timer(Duration::from_millis(500), move || {
        count.fetch_add(1, Ordering::Relaxed);
        let message = String_ {
            data: format!("Hello, world! {}", count.load(Ordering::Relaxed)),
        };
        rclrust_info!(logger, "Publishing: '{}'", message.data);
        publisher.publish(&message).unwrap();
    })?;

    node.wait();

    Ok(())
}
