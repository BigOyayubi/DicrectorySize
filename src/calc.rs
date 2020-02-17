use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

use rayon::prelude::*;

use crate::args::Args;
type DirSizeVec = Vec<(PathBuf, u64)>;

pub fn calc_dir_size(args: &Args) -> Result<DirSizeVec, String> {
    dump_args(&args);

    let root = Path::new(&args.dir_path);
    if !root.is_dir() {
        return Err("args path must be dir.".into());
    }

    calc_dir_size_impl(root, 0, args.depth)
}

//
fn calc_dir_size_impl(root: &Path, cur_depth: i32, max_depth: i32)
 -> Result<DirSizeVec, String> {
    let mut dir_size_vec: DirSizeVec = Vec::with_capacity(1);
    let entry = fs::read_dir(root);
    if entry.is_err() {
        return Err(format!("can not read dir {}", root.to_string_lossy()));
    }

    let mut dirs: Vec<DirEntry> = Vec::new();
    let mut files:  Vec<DirEntry> = Vec::new();
    for path in entry.unwrap() {
        match path {
            Ok(de) => {
                if !FileInfo::is_symlink(&de) {
                    if de.path().is_dir() {
                        dirs.push(de);
                    } else {
                        files.push(de);
                    }
                }
            },
            Err(_) => {},
        }
    }

    let mut dir_sizes: Vec<_> = dirs.par_iter()
                    .map( |de| calc_dir_size_impl(&de.path(), cur_depth+1, max_depth) )
                    .collect();

    let files_size = files.par_iter()
                    .map( |de| FileInfo::filesize(de) )
                    .sum();
    
    let mut root_total = files_size;
    for result in &dir_sizes {
        match result {
            Ok(v) => {
                let (_, size) = v.first().unwrap();
                root_total += size;
            },
            Err(_) => {},
        }
    }
    dir_size_vec.push( (root.to_path_buf(), root_total) );
    debug!("{} is {}",root.to_string_lossy(), root_total);
    
    if cur_depth <= max_depth {
        for result in &mut dir_sizes {
            match result {
                Ok(v) => {
                    v.remove(0);
                    for d in v {
                        let (path, size) = d;
                        dir_size_vec.push( (path.to_path_buf(), *size) );
                    }

                },
                Err(_) => {},
            }
        }
    }
    Ok(dir_size_vec)
}

trait FileInfo{
    fn is_symlink(&self) -> bool;
    fn filesize(&self) -> u64;
}

impl FileInfo for Path {
    fn is_symlink(&self) -> bool {
        match fs::symlink_metadata(self) {
            Ok(m) => m.file_type().is_symlink(),
            Err(_) => false,
        }
    }
    fn filesize(&self) -> u64 {
        match fs::metadata(self) {
            Ok(m) => m.len(),
            Err(_) => 0,
        }
    }
}
impl FileInfo for DirEntry {
    fn is_symlink(&self) -> bool {
        match self.file_type() {
            Ok(t) => t.is_symlink(),
            Err(_) => false,
        }
    }
    fn filesize(&self) -> u64 {
        match self.metadata() {
            Ok(m) => m.len(),
            Err(_) => 0,
        }
    }
}

fn dump_args(args: &Args) {
    debug!("\
cli args 
    path : {}
    kilo : {}
    depth: {}
    include: {}
    exclude: {}", 
    args.dir_path, 
    args.kilo, 
    args.depth, 
    args.include_pattern, 
    args.exclude_pattern);
}
