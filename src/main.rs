extern crate shiplift;

use shiplift::Docker;
use std::thread::sleep_ms;

pub fn main() {
  let mut docker = Docker::new();
  loop {
    let mut images = docker.images().list();
    for image in images.unwrap() {
      if "<none>:<none>" == image.RepoTags[0] {
        println!("deleting image {}", image.Id);
        docker.images().get(&image.Id[..]).delete().unwrap();
      } else {
        println!("all clear");
      }
    }
    sleep_ms(5000);
  }
}
