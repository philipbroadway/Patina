use crate::bible::Bible;
use crate::bible::{BibleNumeric, ChapterNumeric};
use crate::utils::kebob_to_title;
use std::collections::BTreeMap;

pub fn find_verse<'a>(bible: &'a Bible, book: &str, chapter: &str, verse: &str) -> Option<&'a str> {
    // println!("Searching for book: {}, chapter: {}, verse: {}", book, chapter, verse);

    let book_entry = bible.books.get(book);
    if book_entry.is_none() {
        println!("Book '{}' not found.", kebob_to_title(book));
        return None;
    }

    let chapter_entry = book_entry.unwrap().chapters.get(chapter);
    if chapter_entry.is_none() {
        println!(
            "Chapter '{}' not found in book '{}'.\n{} has {} chapters.",
            chapter,
            kebob_to_title(book),
            kebob_to_title(book),
            book_entry.unwrap().chapters.len()
        );
        return None;
    }

    let verse_entry = chapter_entry.unwrap().verses.get(verse);
    if verse_entry.is_none() {
        println!(
            "Verse '{}' not found in book '{}' chapter '{}'.\nChapter {} of {} has {} verses",
            verse,
            kebob_to_title(book),
            chapter,
            chapter,
            kebob_to_title(book),
            chapter_entry.unwrap().verses.len()
        );
        return None;
    }
    // println!("Found verse: {}", verse_entry.unwrap());
    Some(verse_entry.unwrap().as_str())
}

pub enum SearchResult<'a> {
    Book(&'a BTreeMap<u32, ChapterNumeric>),
    Chapter(&'a ChapterNumeric),
    Verse(&'a String),
}

pub fn search_by_reference<'a>(
    bible: &'a BibleNumeric,
    book: &str,
    chapter: Option<u32>,
    verse: Option<u32>,
) -> Option<SearchResult<'a>> {
    let book_data = bible.books.get(book)?;
    match (chapter, verse) {
        (None, _) => Some(SearchResult::Book(&book_data.chapters)),
        (Some(ch), None) => {
            let chapter_data = book_data.chapters.get(&ch)?;
            Some(SearchResult::Chapter(chapter_data))
        }
        (Some(ch), Some(v)) => {
            let chapter_data = book_data.chapters.get(&ch)?;
            let verse_text = chapter_data.verses.get(&v)?;
            Some(SearchResult::Verse(verse_text))
        }
    }
}
