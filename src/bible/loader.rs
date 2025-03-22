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

impl BibleNumeric {
  pub fn canonical_order(&self) -> Vec<String> {
    let keys = [
        "genesis", "exodus", "leviticus", "numbers", "deuteronomy",
        "joshua", "judges", "ruth", "1-samuel", "2-samuel", "1-kings", "2-kings",
        "1-chronicles", "2-chronicles", "ezra", "nehemiah", "esther", "job", "psalms",
        "proverbs", "ecclesiastes", "song-of-solomon", "isaiah", "jeremiah", "lamentations",
        "ezekiel", "daniel", "hosea", "joel", "amos", "obadiah", "jonah", "micah", "nahum",
        "habakkuk", "zephaniah", "haggai", "zechariah", "malachi",
        "matthew", "mark", "luke", "john", "acts", "romans", "1-corinthians", "2-corinthians",
        "galatians", "ephesians", "philippians", "colossians", "1-thessalonians", "2-thessalonians",
        "1-timothy", "2-timothy", "titus", "philemon", "hebrews", "james", "1-peter", "2-peter",
        "1-john", "2-john", "3-john", "jude", "revelation",
    ];
    let mut ordered_books = Vec::new();
    for &book in keys.iter() {
        if self.books.contains_key(book) {
            ordered_books.push(book.to_string());
        }
    }
    ordered_books
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

