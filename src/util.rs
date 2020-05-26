use select::{
    document::Document,
    node::Node,
    predicate::{
        And,
        Attr,
        Name,
    },
};

pub(crate) fn extract_name_value<'a>(node: Node<'a>, name: &str) -> Option<&'a str> {
    node.find(Attr("name", name)).next()?.attr("value")
}

pub(crate) fn extract_csrf_token(doc: &Document) -> Option<&str> {
    doc.find(And(Name("meta"), Attr("name", "csrf-token")))
        .next()?
        .attr("content")
}
