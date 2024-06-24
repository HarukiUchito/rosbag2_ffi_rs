use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filepath: String,
}

fn main() {
    let args = Args::parse();

    let reader = rosbag2_ffi_rs::Rosbag2Reader::new(args.filepath.as_str());
    println!("{}", reader);

    let topics = reader.parse_topic::<r2r::geometry_msgs::msg::Pose2D>("/MachinePose2D");
    println!("{:?}", topics);
    println!("topic number: {:?}", topics.len());
}
