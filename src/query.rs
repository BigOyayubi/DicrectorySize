use std::io::{self, DirEntry};
use std::collections::HashMap;

trait Runner {
    fn run(&self) -> (u64/*size*/, HashMap<DirEntry, u64>/*child dir info*/);
}

pub struct DirQuery{
    dirEntry: DirEntry,
}

pub struct FileQuery{
    dirEntries : Vec<DirEntry>;
}

impl Runner for DirQuery {
    fn run(&self) -> (u64, HashMap<DirEntry, u64>){

        
    }

}

fn toRunner(dirEntry: &DirEntry) -> Option<Runner> {
    if is_symlink(dirEntry) {
        return None;
    }

    let readdir = match fs::read_dir(self.dirEntry.path()) {
            Ok(readdir) => readdir,
            Err(raeddir) => { return (0, ()) }
    }

    let dirs = readdir.

    if dirEntry.is_dir() {

    } else {
        
    }
}
fn is_symlink(dirEntry: &DirEntry) -> bool {
        dirEntry.file_type().map_or(false, |ft| ft.is_symlink())
}
