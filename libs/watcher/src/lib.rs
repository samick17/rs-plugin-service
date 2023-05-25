use std::{time::Duration};
use notify::{RecursiveMode, Watcher};
use notify::event::{EventKind, CreateKind, ModifyKind, RenameMode, MetadataKind};
use notify_debouncer_full::new_debouncer;
use std::path::{Path, PathBuf};

// type Callback = fn(Vec::<String>);
type Callback = fn(Vec::<PathBuf>);

pub struct WatchOpts {
    pub path: String,
}

pub fn watch(opts: WatchOpts, callback: Callback) -> Result<(), Box<dyn std::error::Error>> {

    // setup debouncer
    let (tx, rx) = std::sync::mpsc::channel();

    // no specific tickrate, max debounce time 1 seconds
    let mut debouncer = new_debouncer(Duration::from_secs(1), None, tx)?;

    let file_path = Path::new(&opts.path);
    debouncer
        .watcher()
        .watch(file_path, RecursiveMode::Recursive)?;

    debouncer
        .cache()
        .add_root(file_path, RecursiveMode::Recursive);

    // print all events and errors
    for result in rx {
        match result {
            Ok(events) => events.iter().for_each(|event| {
                // println!("{event:?}");
                if let EventKind::Create(CreateKind::File) = event.kind {
                    // println!("[Watcher] Create File");
                    // println!("{:?}", event.paths);
                    // callback(event.paths.iter().map(|x| x.clone().into_os_string().to_str().unwrap().to_string()).collect::<Vec<String>>());
                    callback(event.paths.iter().map(|x| x.clone()).collect::<Vec<PathBuf>>());
                } else if let EventKind::Modify(ModifyKind::Metadata(MetadataKind::Any)) = event.kind {
                    // println!("[Watcher] Modify Metadata");
                    // println!("{:?}", event.paths);
                    // callback(event.paths.iter().map(|x| x.clone().into_os_string().to_str().unwrap().to_string()).collect::<Vec<String>>());
                    callback(event.paths.iter().map(|x| x.clone()).collect::<Vec<PathBuf>>());
                } else if let EventKind::Modify(ModifyKind::Name(RenameMode::Both)) = event.kind {
                    // println!("[Watcher] Name Changed: Both");
                    // println!("{:?}", event.paths);
                    // callback(event.paths.iter().map(|x| x.clone().into_os_string().to_str().unwrap().to_string()).collect::<Vec<String>>());
                    callback(event.paths.iter().map(|x| x.clone()).collect::<Vec<PathBuf>>());
                } else if let EventKind::Modify(ModifyKind::Name(RenameMode::Any)) = event.kind {
                    // println!("[Watcher] Name Changed: Any");
                    // callback(event.paths.iter().map(|x| x.clone().into_os_string().to_str().unwrap().to_string()).collect::<Vec<String>>());
                    callback(event.paths.iter().map(|x| x.clone()).collect::<Vec<PathBuf>>());
                }
            }),
            Err(errors) => errors.iter().for_each(|error| {
                println!("{error:?}")
            }),
        }
        println!();
    }

    Ok(())
}
