# rclrust-examples

This repository shows examples of using [rclrust](https://github.com/rclrust/rclrust) with colcon.

## Requirements

- ROS2 foxy+
- Rust 1.53+

## Build

```sh-session
$ cd <your/workspace>/src
$ git clone https://github.com/rclrust/rclrust-examples
$ cd ../
$ colcon build
```

## Pub/Sub

### Publisher

```sh-session
$ ros2 run rclrust_examples_minimal_publisher rclrust_examples_minimal_publisher
[INFO] [1630516491.396324233] [minimal_publisher]: Publishing: 'Hello, world! 1'
[INFO] [1630516491.896318225] [minimal_publisher]: Publishing: 'Hello, world! 2'
[INFO] [1630516492.396316312] [minimal_publisher]: Publishing: 'Hello, world! 3'
[INFO] [1630516492.896360752] [minimal_publisher]: Publishing: 'Hello, world! 4'
[INFO] [1630516493.396322000] [minimal_publisher]: Publishing: 'Hello, world! 5'
:
:
```

### Subscriber

```sh-session
$ ros2 run rclrust_examples_minimal_subscriber rclrust_examples_minimal_subscriber
[INFO] [1630516575.877511586] [minimal_subscriber]: I heard: 'Hello, world! 1'
[INFO] [1630516576.377395436] [minimal_subscriber]: I heard: 'Hello, world! 2'
[INFO] [1630516576.877378362] [minimal_subscriber]: I heard: 'Hello, world! 3'
[INFO] [1630516577.377380077] [minimal_subscriber]: I heard: 'Hello, world! 4'
[INFO] [1630516577.877374734] [minimal_subscriber]: I heard: 'Hello, world! 5'
:
:
```
