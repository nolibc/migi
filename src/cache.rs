use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    time::SystemTime, rc::Rc,
};

use crate::source;
use std::cell::RefCell;

#[derive(Serialize, Deserialize, Debug)]
pub struct CacheData {
    pub name: PathBuf,
    pub title: String,
    pub created: SystemTime,
    pub last_modified: SystemTime,
    pub tags: Vec<String>,
}

pub struct DataManager {
    pub source_files: RefCell<HashMap<PathBuf, SystemTime>>,
    pub cache: RefCell<Vec<CacheData>>,
    pub required_changes: RefCell<Vec<PathBuf>>,
    cache_name: Rc<str>,
}

trait ReturnJson {
    type Output;
    fn populate_struct(&self) -> Self::Output;
}

impl ReturnJson for PathBuf {
    type Output = CacheData;

    fn populate_struct(&self) -> Self::Output {
        let file_contents = fs::read_to_string(self).unwrap();
        let file_data = source::HeaderParser::get_data(&file_contents).unwrap();

        CacheData {
            name: self.into(),
            title: file_data.title,
            last_modified: modification_time(self.into()),
            tags: file_data.tags,
            created: created_date(self.into()),
        }
    }
}

impl DataManager {
    pub fn remove_missing_entries(&mut self) {
        let mut cache = self.cache.borrow_mut();
        cache.retain(|item| self.source_files.borrow().contains_key(&item.name));
    }

    pub fn update_outdated_entries(&mut self) {
        let mut cache = self.cache.borrow_mut();
        for entry in cache.iter_mut() {
            if &entry.last_modified != self.source_files.borrow().get(&entry.name).unwrap() {
                *entry = entry.name.populate_struct();
                self.required_changes
                    .borrow_mut()
                    .push(entry.name.clone());
            }
        }
    }

    pub fn add_new_entries(&mut self) {
        let cache_set: HashSet<PathBuf> = self
            .cache
            .borrow()
            .iter()
            .map(|item| item.name.clone())
            .collect();

        for file in self.source_files.borrow().keys() {
            if !cache_set.contains(file) {
                self.cache.borrow_mut().push(file.populate_struct());
                self.required_changes.borrow_mut().push(file.clone());
            }
        }
    }

    pub fn write_to_json(&self) -> Result<(), io::Error> {
        let mut buffer = File::create(self.cache_name.as_ref()).unwrap();
        serde_json::to_writer_pretty(&mut buffer, &self.cache.take()).unwrap();

        Ok(())
    }
    /// A convience method that abstracts the work of the other available caching methods.
    ///
    /// Note: this method does not include the write to file provided by `write_to_json`.
    pub fn process_data(&mut self) -> Result<(), io::Error> {
        self.remove_missing_entries();
        self.update_outdated_entries();
        self.add_new_entries();

        Ok(())
    }
}

/// Get the time of (since UNIX_EPOCH) of when a given file was last modified in seconds.
///
/// In order for the caching system to work, we need to detect out of date files (i.e. files the
/// user has cached already but decided to modify). We can detect if the user modified a file by
/// caching the time the file was last modified, and that is this function's primary use case.
fn modification_time(path: PathBuf) -> SystemTime {
    let metadata = fs::metadata(&path).unwrap();

    let date_modified = metadata
        .modified()
        .unwrap();

    date_modified
}

fn created_date(path: PathBuf) -> SystemTime {
    let metadata = fs::metadata(&path).unwrap();
    return metadata.created().unwrap();
}

impl CacheData {
    fn read(cache_file: &str) -> Result<Vec<Self>, io::Error> {
        if !Path::new(cache_file).exists() {
            fs::create_dir_all("cache")?;
            let mut f = File::create(cache_file)?;
            f.write_all("[]".as_bytes())?;
        }

        let mut buffer = String::new();
        File::open(cache_file)?.read_to_string(&mut buffer)?;

        Ok(serde_json::from_str(&buffer)?)
    }

    pub fn create_manager(raw_files: Vec<PathBuf>, cache_file: &str) -> Result<DataManager, io::Error> {
        let mut source_file_cache_info: HashMap<PathBuf, SystemTime> = HashMap::new();

        for file in raw_files {
            source_file_cache_info.insert(file.clone(), modification_time(file));
        }

        let manager = DataManager {
            source_files: RefCell::new(source_file_cache_info),
            cache: RefCell::new(CacheData::read(cache_file).unwrap()),
            required_changes: RefCell::new(Vec::new()),
            cache_name: Rc::from(cache_file),
        };

        Ok(manager)
    }
}
