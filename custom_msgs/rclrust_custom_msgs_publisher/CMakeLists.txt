cmake_minimum_required(VERSION 3.5)
project(rclrust_custom_msgs_publisher)

find_package(ament_cmake REQUIRED)

ament_package()

add_custom_target(${PROJECT_NAME}
  ALL
  COMMAND
    CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR}/target
    cargo install --force -q
      --features rclrust/$ENV{ROS_DISTRO}
      --path ${CMAKE_CURRENT_SOURCE_DIR}
      --root ${CMAKE_INSTALL_PREFIX}/lib/${PROJECT_NAME}
)
