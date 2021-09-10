use rclrust::{qos::QoSProfile, rclrust_info};
use rclrust_msg::rclrust_custom_msgs::msg::Custom;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ctx = rclrust::init()?;
    let node = ctx.create_node("rclrust_custom_msgs_publisher")?;
    let publisher = node.create_publisher::<Custom>("topic", &QoSProfile::default())?;

    let msg = Custom { data: 0 };
    publisher.publish(&msg).unwrap();
    rclrust_info!(node.logger(), "{:?}", msg);

    Ok(())
}
