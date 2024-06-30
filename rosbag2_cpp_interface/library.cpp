#include "library.hpp"
#include <limits.h>
#include <iostream>

#ifdef __cplusplus
extern "C"
{
#endif

    Rosbag2CppReaderImpl *create_bag_reader()
    {
        return new Rosbag2CppReaderImpl;
    }

    void open_bag(Rosbag2CppReaderImpl *reader, const char *filepath)
    {
        std::string filepath_str = std::string(filepath);
        reader->impl.open(filepath_str);
        reader->metadata = reader->impl.get_all_topics_and_types();
    }

    size_t get_number_of_topics(Rosbag2CppReaderImpl *reader)
    {
        return reader->metadata.size();
    }

    const char *get_topic_name(Rosbag2CppReaderImpl *reader, size_t num)
    {
        return reader->metadata[num].name.c_str();
    }

    const char *get_topic_type(Rosbag2CppReaderImpl *reader, size_t num)
    {
        // const std::string topic_type = reader->impl.get_all_topics_and_types()[num].type;
        return reader->metadata[num].type.c_str();
    }

    void seek_bag_reader(Rosbag2CppReaderImpl *reader, int64_t t)
    {
        reader->impl.seek(t);
    }

    Rosbag2Topic get_next_topic(Rosbag2CppReaderImpl *reader)
    {
        reader->current_msg = reader->impl.read_next();
        return Rosbag2Topic{.topic_name = reinterpret_cast<const char *>(reader->current_msg->topic_name.c_str()), .time_stamp = reader->current_msg->time_stamp, .topic_buffer_size = reader->current_msg->serialized_data->buffer_length, .topic_buffer = reader->current_msg->serialized_data->buffer};
    }

    bool has_next_topic(Rosbag2CppReaderImpl *reader)
    {
        return reader->impl.has_next();
    }

    void close_bag(Rosbag2CppReaderImpl *reader)
    {
        reader->impl.close();
        delete reader;
    }

#ifdef __cplusplus
}
#endif
