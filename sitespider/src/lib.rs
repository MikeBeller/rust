
//use reqwest::blocking::get;
use select::document::Document;
use select::predicate::Name;

fn extract_links(body: &str) -> Vec<String> {
    Document::from(body)
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .map(|u| u.to_string())
        .collect::<Vec<String>>()
}

#[test]
fn test_extract_links() {
    let body = r#"<html><head></head>
    <body>blah blah 
    <a href="/foo/bar"></a>
    <a>missing</a>
    blah blah
    <a href="http://example.com/x/y">title</a>
    </body></html>
    "#;

    let r = extract_links(body);
    assert_eq!(r, vec!["/foo/bar", "http://example.com/x/y"]);
    println!("{:?}", r);
}

