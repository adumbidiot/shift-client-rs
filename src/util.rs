use once_cell::sync::Lazy;
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
    static META_SELECTOR: Lazy<Selector> = Lazy::new(|| {
        Selector::parse("meta[name=\"csrf-token\"][content]").expect("invalid META_SELECTOR")
    });
    html.select(&META_SELECTOR).next()?.value().attr("content")
}
