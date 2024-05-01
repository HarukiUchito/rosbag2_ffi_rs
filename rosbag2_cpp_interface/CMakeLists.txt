cmake_minimum_required(VERSION 3.5)
project(rosbag2_cpp_interface)
set(CMAKE_C_STANDARD 99)
set(CMAKE_CXX_STANDARD 17)

set(AMENT_PREFIX_PATH /opt/ros/humble/)

if(CMAKE_COMPILER_IS_GNUCXX OR CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

find_package(ament_cmake REQUIRED)
#find_package(rclcpp REQUIRED)
find_package(rosbag2_cpp REQUIRED)

add_library(rosbag2_cpp_interface library.cpp)

ament_target_dependencies(rosbag2_cpp_interface rosbag2_cpp)
install (
  TARGETS rosbag2_cpp_interface
  DESTINATION lib/${PROJECT_NAME}
  DESTINATION .
)
