use std::sync::Arc;

use rclrust::{qos::QoSProfile, rclrust_info};
use rclrust_msg::std_msgs::msg::String as String_;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ctx = rclrust::init()?;
    let mut node = ctx.create_node("minimal_subscriber")?;
    let logger = node.logger();

    let _subscription = node.create_subscription(
        "topic",
        move |msg: Arc<String_>| {
            rclrust_info!(logger, "I heard: '{}'", msg.data);
        },
        &QoSProfile::default(),
    )?;

    node.wait();
    Ok(())
}
