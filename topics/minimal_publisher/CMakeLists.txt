cmake_minimum_required(VERSION 3.5)
project(rclrust_examples_minimal_publisher)

find_package(ament_cmake REQUIRED)

ament_package()

add_custom_target(${PROJECT_NAME}
  ALL
  COMMAND
    cargo build
      --manifest-path ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml
      --features rclrust/$ENV{ROS_DISTRO}
      --release
  BYPRODUCTS
    ${CMAKE_CURRENT_SOURCE_DIR}/target/release/${PROJECT_NAME}
)

install(
  FILES
    ${CMAKE_CURRENT_SOURCE_DIR}/target/release/${PROJECT_NAME}
  PERMISSIONS
    OWNER_READ OWNER_WRITE OWNER_EXECUTE GROUP_READ GROUP_EXECUTE WORLD_READ WORLD_EXECUTE
  DESTINATION
    lib/${PROJECT_NAME}
)
