use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use patina::Patina;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(list)
            .service(get_book)
            .service(get_chapter)
            .service(get_verse)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

static STYLE: &str = r#"
<style>
  body {
    font-family: sans-serif;
    margin: 0;
    padding: 1em;
    line-height: 1.3;
  }
  a {
    text-decoration: none;
  }
  a:hover {
    text-decoration: underline;
  }
  h2 {
    margin-bottom: 0;
    padding: 0;
  }
  hr {
    border-top: 0.5px solid #ccc;
  }
</style>
"#;

#[get("/")]
async fn list() -> impl Responder {
    let patina = Patina::new();
    let books = patina.list_books();
    let body = books
        .iter()
        .map(|book| format!(r#"<a href="/{}">{}</a>"#, book, kebab_to_title_case(book)))
        .collect::<Vec<_>>()
        .join("<br/>");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!(
          r#"
            <html>
              <head>
                {}
              </head>
              <body>
                <h2>{}</h2>
                <hr />
                {}
              </body>
            </html>
          "#,STYLE, "The Bible", body))
}


#[get("/{book}")]
async fn get_book(path: web::Path<String>) -> impl Responder {
    let patina = Patina::new();
    let book = path.into_inner();
    let chapters = patina.list_chapters(&book).unwrap_or_default();

    let body = chapters
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let chapter_num = i + 1;
            format!(r#"<a href="/{}/{}">Chapter {}</a>"#, book, chapter_num, chapter_num)
        })
        .collect::<Vec<_>>()
        .join("<br/>");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!(
          r#"
            <html>
              <head>
                {}
              </head>
              <body>
                <h2><a href="/">{}</a></h2>
                <hr />
                {}
              </body>
            </html>
          "#,STYLE, kebab_to_title_case(&book), body)
        )
}


#[get("/{book}/{chapter}")]
async fn get_chapter(path: web::Path<(String, u32)>) -> impl Responder {
  let patina = Patina::new();
  let (book, chapter) = path.into_inner();

  let verses = patina.list_verses(&book, chapter).unwrap_or_default();

  let body = verses
      .iter()
      .enumerate()
      .map(|(i, _)| {
          let verse_num = i + 1;
          format!(r#"<a href="/{}/{}/{}">Verse {}</a>"#, book, chapter, verse_num, verse_num)
      })
      .collect::<Vec<_>>()
      .join("<br/>");

  HttpResponse::Ok()
      .content_type("text/html; charset=utf-8")
      .body(format!(
        r#"
          <html>
            <head>
              {}
            </head>
            <body>
              <h2><a href="/{}">{} {}</a></h2>
              <hr />
              {}
            </body>
          </html>
        "#,STYLE, &book, kebab_to_title_case(&book), chapter, body)
      )
}

#[get("/{book}/{chapter}/{verse}")]
async fn get_verse(path: web::Path<(String, String, String)>) -> impl Responder {
    let (book, chapter, verse) = path.into_inner();

    let chapter: Option<u32> = if chapter.is_empty() {
        None
    } else {
        match chapter.parse() {
            Ok(num) => Some(num),
            Err(_) => return HttpResponse::BadRequest().body("Invalid chapter format"),
        }
    };

    let verse: Option<u32> = if verse.is_empty() {
        None
    } else {
        match verse.parse() {
            Ok(num) => Some(num),
            Err(_) => return HttpResponse::BadRequest().body("Invalid verse format"),
        }
    };

    let body = Patina::new()
        .search_by_reference(&book, chapter, verse)
        .unwrap_or_else(|| "Reference not found.".to_string());

    let chapter_display = chapter.map(|ch| ch.to_string()).unwrap_or_default();
    let verse_display = verse.map(|v| v.to_string()).unwrap_or_default();

    HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(format!(
      r#"
        <html>
          <head>
            {}
          </head>
          <body>
            {}<hr/><span><a href="/{}">{}</a> <a href="/{}/{}">{}:{}</a></span>
          </body>
        </html>
      "#,STYLE, body, &book, kebab_to_title_case(&book), &book, chapter_display, chapter_display, verse_display)
    )
}

fn kebab_to_title_case(s: &str) -> String {
  s.split('-')
      .map(|word| {
          let mut c = word.chars();
          match c.next() {
              Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
              None => String::new(),
          }
      })
      .collect::<Vec<_>>()
      .join(" ")
}