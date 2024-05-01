use r2r::WrappedTypesupport;
use std::collections::HashMap;

pub enum RosBag2CppReaderImpl {}

#[repr(C)]
pub struct Rosbag2Topic {
    pub topic_name: *const std::os::raw::c_char,
    pub topic_buffer_size: std::os::raw::c_uint,
    pub topic_buffer: *const std::os::raw::c_uchar,
}

#[link(name = "rosbag2_cpp_interface")]
extern "C" {
    pub fn create_bag_reader() -> *mut RosBag2CppReaderImpl;
    pub fn open_bag(reader: *mut RosBag2CppReaderImpl, filepath: *const std::os::raw::c_char);
    pub fn get_number_of_topics(reader: *mut RosBag2CppReaderImpl) -> usize;
    pub fn get_topic_name(
        reader: *mut RosBag2CppReaderImpl,
        num: usize,
    ) -> *const std::os::raw::c_char;
    pub fn get_topic_type(
        reader: *mut RosBag2CppReaderImpl,
        num: usize,
    ) -> *const std::os::raw::c_char;
    pub fn has_next_topic(reader: *mut RosBag2CppReaderImpl) -> bool;
    pub fn get_next_topic(reader: *mut RosBag2CppReaderImpl) -> Rosbag2Topic;
    pub fn close_bag(reader: *mut RosBag2CppReaderImpl);
}

#[derive(Debug)]
pub struct Rosbag2Reader {
    impl_ptr: *mut RosBag2CppReaderImpl,
    name_to_type_map: HashMap<String, String>,
}

impl Rosbag2Reader {
    pub fn new(filepath: &str) -> Self {
        let ptr = unsafe { create_bag_reader() };
        let mut name_to_type_map = HashMap::new();

        let filepath = std::ffi::CString::new(filepath).unwrap();
        unsafe { open_bag(ptr, filepath.as_ptr()) };

        let t_len = unsafe { get_number_of_topics(ptr) };
        for i in 0..t_len {
            let name = unsafe { std::ffi::CStr::from_ptr(get_topic_name(ptr, i)) }
                .to_str()
                .unwrap();

            let topic_type = unsafe { std::ffi::CStr::from_ptr(get_topic_type(ptr, i)) }
                .to_str()
                .unwrap();

            name_to_type_map.insert(name.to_string(), topic_type.to_string());
        }

        Rosbag2Reader {
            impl_ptr: ptr,
            name_to_type_map: name_to_type_map,
        }
    }

    pub fn next(&self) {
        let next_topic = unsafe { get_next_topic(self.impl_ptr) };
        let topic_data = unsafe {
            std::slice::from_raw_parts(
                next_topic.topic_buffer,
                next_topic.topic_buffer_size as usize,
            )
        };

        let topic_name = unsafe { std::ffi::CStr::from_ptr(next_topic.topic_name) }
            .to_str()
            .unwrap();
        let topic_type = self.name_to_type_map.get(topic_name).unwrap();
        let msg = match topic_type.as_str() {
            "geometry_msgs/msg/Pose2D" => {
                r2r::geometry_msgs::msg::Pose2D::from_serialized_bytes(topic_data)
            }
            _ => panic!("not supported type {}", topic_name),
        };
        println!("{:?}", msg);
    }

    pub fn geometry_msgs_pose_2d_topic(
        &self,
        topic_name: &str,
    ) -> Vec<r2r::geometry_msgs::msg::Pose2D> {
        let mut ret = Vec::new();
        while unsafe { has_next_topic(self.impl_ptr) } {
            let next_topic = loop {
                if !unsafe { has_next_topic(self.impl_ptr) } {
                    break None;
                }
                let cand = unsafe { get_next_topic(self.impl_ptr) };
                let topic_name = unsafe { std::ffi::CStr::from_ptr(cand.topic_name) }
                    .to_str()
                    .unwrap();
                if self.name_to_type_map.get(topic_name).unwrap().clone()
                    == "geometry_msgs/msg/Pose2D".to_string()
                {
                    break Some(cand);
                }
            };
            if let Some(next_topic) = next_topic {
                let topic_data = unsafe {
                    std::slice::from_raw_parts(
                        next_topic.topic_buffer,
                        next_topic.topic_buffer_size as usize,
                    )
                };

                let next_topic_name = unsafe { std::ffi::CStr::from_ptr(next_topic.topic_name) }
                    .to_str()
                    .unwrap();
                if topic_name != next_topic_name {
                    continue;
                }
                let topic_type = self.name_to_type_map.get(topic_name).unwrap();
                let msg = match topic_type.as_str() {
                    "geometry_msgs/msg/Pose2D" => {
                        r2r::geometry_msgs::msg::Pose2D::from_serialized_bytes(topic_data)
                    }
                    _ => panic!("not supported type {}", topic_name),
                };
                if let Ok(msg) = msg {
                    ret.push(msg);
                }
            }
        }
        return ret;
    }
}

impl Drop for Rosbag2Reader {
    #[inline]
    fn drop(&mut self) {
        unsafe { close_bag(self.impl_ptr) }
    }
}

//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
