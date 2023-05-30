use watcher::{watch, WatchOpts};

pub fn main() {
    let _ = watch(WatchOpts{
        path: "./public".to_owned(),
    }, |x| {
        println!("Changed {:?}", x);
    });
}
