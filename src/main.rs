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
    println!("{:?}", reader);

    println!(
        "{:?}",
        reader.geometry_msgs_pose_2d_topic("/MachinePose2D").len()
    );
}
