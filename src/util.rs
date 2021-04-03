use scraper::{
    ElementRef,
    Html,
    Selector,
};

/// Extract the value of an element by the name attr
pub(crate) fn extract_by_name<'a>(element: ElementRef<'a>, name: &str) -> Option<&'a str> {
    let selector = Selector::parse(&format!("[name=\"{}\"][value]", name)).ok()?;
    element.select(&selector).next()?.value().attr("value")
}

/// Extract the csrf token
pub(crate) fn extract_csrf_token(html: &Html) -> Option<&str> {
    let selector =
        Selector::parse("meta[name=\"csrf-token\"][content]").expect("invalid csrf selector");
    html.select(&selector).next()?.value().attr("content")
}
