use serde::Deserialize;
use std::collections::BTreeMap;
use std::collections::HashMap;

impl Bible {
    pub fn normalize(self) -> BibleNumeric {
        BibleNumeric {
            books: self
                .books
                .into_iter()
                .map(|(book_name, book)| (book_name, book.normalize()))
                .collect(),
        }
    }
}

impl Book {
    fn normalize(self) -> BookNumeric {
        BookNumeric {
            chapters: self
                .chapters
                .into_iter()
                .filter_map(|(k, v)| k.parse::<u32>().ok().map(|key| (key, v.normalize())))
                .collect(),
        }
    }
}

impl Chapter {
    fn normalize(self) -> ChapterNumeric {
        ChapterNumeric {
            verses: self
                .verses
                .into_iter()
                .filter_map(|(k, v)| k.parse::<u32>().ok().map(|key| (key, v)))
                .collect(),
        }
    }
}


#[derive(Deserialize, Debug)]
pub struct Bible {
    #[serde(flatten)]
    pub books: HashMap<String, Book>,
}

#[derive(Deserialize, Debug)]
pub struct Book {
    #[serde(flatten)]
    pub chapters: HashMap<String, Chapter>,
}

#[derive(Deserialize, Debug)]
pub struct Chapter {
    #[serde(flatten)]
    pub verses: HashMap<String, String>,
}

pub struct BibleNumeric {
  pub books: BTreeMap<String, BookNumeric>,
}

pub struct BookNumeric {
  pub chapters: BTreeMap<u32, ChapterNumeric>,
}

pub struct ChapterNumeric {
  pub verses: BTreeMap<u32, String>,
}