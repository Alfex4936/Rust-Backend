#[macro_use]
extern crate criterion;

use criterion::{criterion_group, criterion_main, Bencher, Criterion};
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};

pub const AJOU_LINK: &str = "https://www.ajou.ac.kr/kr/ajou/notice.do";

#[derive(Clone, Default)]
pub struct Notice {
    pub id: i32,
    pub title: String,
    pub date: String,
    pub link: String,
    pub writer: String,
}

pub fn notice_parse(_nums: Option<usize>) -> Result<Vec<Notice>, reqwest::Error> {
    let mut ajou =
        "https://www.ajou.ac.kr/kr/ajou/notice.do?mode=list&article.offset=0&articleLimit="
            .to_string();

    let nums_int = _nums.unwrap_or(5);
    let nums_str = nums_int.to_string();

    ajou.push_str(&nums_str);

    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    // header 없이 보내면 404
    let res = client.get(ajou).header(USER_AGENT, "User-Agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.114 Safari/537.36").send().unwrap();
    let body = res.text().unwrap();

    // println!("Body:\n{}", body);

    // HTML Parse
    let document = Html::parse_document(&body);
    let a_selector = Selector::parse("a").unwrap();

    // Notice has id, title, date, link, writer
    let ids = Selector::parse("td.b-num-box").unwrap();
    let titles = Selector::parse("div.b-title-box").unwrap(); // includes links
    let dates = Selector::parse("span.b-date").unwrap();
    let writers = Selector::parse("span.b-writer").unwrap();

    let mut notices = vec![Notice::default(); nums_int];

    let mut id_elements = document.select(&ids);
    let mut title_elements = document.select(&titles);
    let mut date_elements = document.select(&dates);
    let mut writer_elements = document.select(&writers);

    // struct Notice
    for index in 0..nums_int {
        let id_element = id_elements.next().unwrap();
        let id = id_element.text().collect::<Vec<_>>()[0]
            .trim() // " 12345 "
            .parse::<i32>()
            .unwrap();

        let date_element = date_elements.next().unwrap();
        let date = date_element.text().collect::<Vec<_>>()[0]
            .trim()
            .to_string(); // "2021-07-15"

        let writer_element = writer_elements.next().unwrap();
        let writer = writer_element.text().collect::<Vec<_>>()[0]
            .trim()
            .to_string(); // "가나다라마"

        let title_element = title_elements.next().unwrap();
        let inner_a = title_element.select(&a_selector).next().unwrap();

        let mut title = inner_a.value().attr("title").unwrap().to_string();
        let link = AJOU_LINK.to_string() + inner_a.value().attr("href").unwrap();
        // Check duplication. title: [writer] blah -> title: [blah]
        let dup = "[".to_string() + &writer + "]";
        if title.contains(&dup) {
            title = title.replace(&dup, "");
            // title.replace_range(0..dup.len(), "");
        }
        // println!("id: {}, title: {}, link: {}, date: {}, writer: {}", id, title, link, date, writer);

        let useless = " 자세히 보기".to_string();
        if title.contains(&useless) {
            title = title.replace(&useless, "");
        }

        let useless = "(재공지)".to_string();
        if title.contains(&useless) {
            title = title.replace(&useless, "");
        }

        title = title.trim().to_string();

        // title.retain(|c| !r#"~「」"#.contains(c));

        notices[index].id = id;
        notices[index].title = title;
        notices[index].link = link;
        notices[index].date = date;
        notices[index].writer = writer;
    }
    // println!("{:?}", notices);
    Ok(notices)
}

fn bench_parse_notice(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..10 {
            notice_parse(Some(30)).unwrap();
        }
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse_notice", bench_parse_notice);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);