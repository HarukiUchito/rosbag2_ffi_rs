#ifndef CMAKE_LIBRARY_H
#define CMAKE_LIBRARY_H
#include <rosbag2_cpp/reader.hpp>

typedef struct
{
    rosbag2_cpp::Reader impl;
    std::vector<rosbag2_storage::TopicMetadata> metadata;
} Rosbag2CppReaderImpl;

typedef struct
{
    const char *topic_name;
    int64_t time_stamp;
    size_t topic_buffer_size;
    const uint8_t *topic_buffer;
} Rosbag2Topic;

#ifdef __cplusplus
extern "C"
{
#endif

    Rosbag2CppReaderImpl *create_bag_reader();
    void open_bag(Rosbag2CppReaderImpl *reader, const char *filepath);

    size_t get_number_of_topics(Rosbag2CppReaderImpl *reader);
    const char *get_topic_name(Rosbag2CppReaderImpl *reader, size_t num);
    const char *get_topic_type(Rosbag2CppReaderImpl *reader, size_t num);

    void seek_bag_reader(Rosbag2CppReaderImpl *reader, int64_t t);
    bool has_next_topic(Rosbag2CppReaderImpl *);
    Rosbag2Topic get_next_topic(Rosbag2CppReaderImpl *reader);

    void close_bag(Rosbag2CppReaderImpl *reader);

#ifdef __cplusplus
}
#endif

#endif // CMAKE_LIBRARY_H
