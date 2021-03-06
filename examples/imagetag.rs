// cargo run --example imagetag img repo tag

use shiplift::{Docker, Image, TagOptions};
use std::env;
use tokio::prelude::Future;

fn main() {
    env_logger::init();
    let docker = Docker::new();
    let img = env::args()
        .nth(1)
        .expect("You need to specify an image name");

    let repo = env::args()
        .nth(2)
        .expect("You need to specify a repository name");

    let tag = env::args().nth(3).expect("You need to specify a tag name");

    let tag_opts = TagOptions::builder().repo(repo).tag(tag).build();

    let image = Image::new(&docker, img);

    let fut = image.tag(&tag_opts).map_err(|e| eprintln!("Error: {}", e));

    tokio::run(fut);
}
