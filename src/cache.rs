use std::{
    collections::{hash_map::DefaultHasher, HashSet},
    fs::{self, File, OpenOptions},
    hash::{Hash, Hasher},
    io,
    path::PathBuf,
};

use crate::WordQuery;

pub struct Cache {
    cache_file: PathBuf,
    cache_dir: PathBuf,
    table: HashSet<String>,
}

type WordSegment = Vec<(String, WordQuery)>;

#[derive(Debug)]
struct CacheMiss;

impl CacheMiss {
    fn new() -> anyhow::Error {
        anyhow::Error::new(CacheMiss)
    }
}

impl std::fmt::Display for CacheMiss {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for CacheMiss {}

impl Cache {
    pub fn new(cache_file: PathBuf, cache_dir: PathBuf) -> Self {
        let table = HashSet::new();
        Self {
            cache_file,
            cache_dir,
            table,
        }
    }
    pub fn of_path(&mut self) -> io::Result<()> {
        let content = fs::read_to_string(&self.cache_file)?;
        let table = serde_json::from_str(&content)?;
        self.table = table;
        Ok(())
    }
    pub fn to_file(&self) -> io::Result<()> {
        let s = serde_json::to_string_pretty(&self.table)?;
        fs::write(&self.cache_file, s)?;
        Ok(())
    }
    fn str_hash(s: impl AsRef<str>) -> u64 {
        let mut hasher = DefaultHasher::new();
        s.as_ref().hash(&mut hasher);
        hasher.finish()
    }
    fn get_file_path(&self, word: impl AsRef<str>) -> PathBuf {
        let num = Cache::str_hash(&word);
        let mut path = self.cache_dir.clone();
        let s = format!("{:06x}", num);
        let (s, _) = s.split_at(6);
        let s = format!("{}.bin", s);
        path.push(s);
        path
    }
    fn read_file_vec(&self, word: impl AsRef<str>) -> anyhow::Result<WordSegment> {
        let path = self.get_file_path(&word);

        let file = File::open(path)?;
        let vec = bincode::deserialize_from(&file)?;

        Ok(vec)
    }
    fn read_word_from_file(&self, word: impl AsRef<str>) -> anyhow::Result<WordQuery> {
        self.read_file_vec(&word)?
            .into_iter()
            .find_map(|(s, word_query)| {
                if s == word.as_ref() {
                    Some(word_query)
                } else {
                    None
                }
            })
            .ok_or_else(|| CacheMiss::new())
    }
    pub fn query(&mut self, word: impl AsRef<str>) -> anyhow::Result<WordQuery> {
        self.table
            .get(word.as_ref())
            .cloned()
            .and_then(|_| {
                self.read_word_from_file(word.as_ref()).map_or_else(
                    |_err| {
                        self.table.remove(word.as_ref());
                        None
                    },
                    |word_query| Some(word_query),
                )
            })
            .ok_or_else(|| CacheMiss::new())
    }
    fn write_word_to_file(&self, word: String, word_query: WordQuery) -> anyhow::Result<()> {
        let mut vec = self.read_file_vec(&word).unwrap_or_default();
        // only write when not contained already
        let should_write = vec.iter().find(|(s, _)| s == &word).is_none();
        if should_write {
            let path = self.get_file_path(&word);
            let file = OpenOptions::new().create(true).write(true).open(path)?;

            vec.push((word, word_query));
            bincode::serialize_into(file, &vec)?;
        }
        Ok(())
    }
    pub fn store(&mut self, word: impl AsRef<str>, word_query: WordQuery) -> anyhow::Result<()> {
        // only update when not in cache table
        if !self.table.contains(word.as_ref()) {
            let word = word.as_ref().to_owned();
            self.write_word_to_file(word.to_owned(), word_query)?;
            self.table.insert(word);
            self.to_file()?;
        }
        Ok(())
    }
}
