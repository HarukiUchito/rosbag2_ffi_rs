use core::fmt;
use r2r::WrappedTypesupport;
use std::collections::HashMap;

pub enum RosBag2CppReaderImpl {}

#[repr(C)]
pub struct Rosbag2Topic {
    pub topic_name: *const std::os::raw::c_char,
    pub time_stamp: std::os::raw::c_longlong,
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
    pub fn seek_bag_reader(reader: *mut RosBag2CppReaderImpl, seek_t: std::os::raw::c_longlong);
    pub fn has_next_topic(reader: *mut RosBag2CppReaderImpl) -> bool;
    pub fn get_next_topic(reader: *mut RosBag2CppReaderImpl) -> Rosbag2Topic;
    pub fn close_bag(reader: *mut RosBag2CppReaderImpl);
}

#[derive(Debug)]
pub struct RosTopicInfo {
    topic_name: String,
    topic_type: String,
    topic_count: usize,
}

#[derive(Debug)]
pub struct Rosbag2Reader {
    impl_ptr: *mut RosBag2CppReaderImpl,
    name_to_info_map: HashMap<String, RosTopicInfo>,
}

unsafe impl Send for Rosbag2Reader {}

impl Rosbag2Reader {
    pub fn new(filepath: &str) -> Self {
        let ptr = unsafe { create_bag_reader() };
        let mut name_to_info_map = HashMap::new();

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

            name_to_info_map.insert(
                name.to_string(),
                RosTopicInfo {
                    topic_name: name.to_string(),
                    topic_type: topic_type.to_string(),
                    topic_count: 0,
                },
            );
        }

        while unsafe { has_next_topic(ptr) } {
            if !unsafe { has_next_topic(ptr) } {
                break;
            }
            let cand = unsafe { get_next_topic(ptr) };
            let topic_name = unsafe { std::ffi::CStr::from_ptr(cand.topic_name) }
                .to_str()
                .unwrap();
            if let Some(info) = name_to_info_map.get_mut(&topic_name.to_string()) {
                info.topic_count += 1;
            }
        }
        unsafe {
            seek_bag_reader(ptr, 0);
        }

        Rosbag2Reader {
            impl_ptr: ptr,
            name_to_info_map: name_to_info_map,
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
        let topic_type = self
            .name_to_info_map
            .get(topic_name)
            .unwrap()
            .topic_type
            .as_str();
        let msg = match topic_type {
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
                if self
                    .name_to_info_map
                    .get(topic_name)
                    .unwrap()
                    .topic_type
                    .as_str()
                    == "geometry_msgs/msg/Pose2D"
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
                let topic_type = self
                    .name_to_info_map
                    .get(topic_name)
                    .unwrap()
                    .topic_type
                    .clone();
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

impl fmt::Display for Rosbag2Reader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let h_name = "Topic Name";
        let h_type = "Topic Type";
        let h_count = "Topic Count";
        let mut w_name = h_name.len();
        let mut w_type = h_type.len();
        let mut w_count = h_type.len();
        for (_, info) in self.name_to_info_map.iter() {
            w_name = std::cmp::max(w_name, info.topic_name.len());
            w_type = std::cmp::max(w_type, info.topic_type.len());
            w_count = std::cmp::max(w_count, info.topic_count.to_string().len());
        }
        writeln!(
            f,
            "{:w_name$}, {:w_type$}, {:w_count$}",
            h_name, h_type, h_count
        )?;
        for (_, info) in self.name_to_info_map.iter() {
            writeln!(
                f,
                "{:w_name$}, {:w_type$}, {:w_count$}",
                info.topic_name, info.topic_type, info.topic_count
            )?
        }
        write!(f, "")
    }
}

impl Drop for Rosbag2Reader {
    #[inline]
    fn drop(&mut self) {
        unsafe { close_bag(self.impl_ptr) }
    }
}

//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
