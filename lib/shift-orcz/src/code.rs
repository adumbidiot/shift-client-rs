use select::{
    node::Node,
    predicate::{
        And,
        Attr,
        Name,
        Text,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Code {
    Valid(String),
    Expired(String),
}

impl Code {
    pub fn is_valid(&self) -> bool {
        match self {
            Self::Valid(_) => true,
            Self::Expired(_) => false,
        }
    }

    pub fn is_expired(&self) -> bool {
        match self {
            Self::Valid(_) => false,
            Self::Expired(_) => true,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Valid(s) => s.as_str(),
            Self::Expired(s) => s.as_str(),
        }
    }

    pub fn as_mut_string(&mut self) -> &mut String {
        match self {
            Self::Valid(s) => s,
            Self::Expired(s) => s,
        }
    }

    pub(crate) fn from_node(node: Node) -> Option<Self> {
        let maybe_span = node
            .find(And(Name("span"), Attr("style", "color:red")))
            .next();

        match maybe_span {
            Some(el) => {
                let code = el.find(Text).next()?.as_text()?.trim();
                Some(Self::Expired(code.into()))
            }
            None => {
                let code = node.find(Text).next()?.as_text()?.trim();
                Some(Self::Valid(code.into()))
            }
        }
    }
}

impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
