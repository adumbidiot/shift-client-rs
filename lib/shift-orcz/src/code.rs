use scraper::{
    ElementRef,
    Selector,
};

/// A Shift Code
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Code {
    /// A valid code
    Valid(String),

    /// An invalid code
    Expired(String),
}

impl Code {
    /// Is this code valid?
    pub fn is_valid(&self) -> bool {
        match self {
            Self::Valid(_) => true,
            Self::Expired(_) => false,
        }
    }

    /// Is this code expired?
    pub fn is_expired(&self) -> bool {
        match self {
            Self::Valid(_) => false,
            Self::Expired(_) => true,
        }
    }

    /// Get this code as a string
    pub fn as_str(&self) -> &str {
        match self {
            Self::Valid(s) => s.as_str(),
            Self::Expired(s) => s.as_str(),
        }
    }

    /// Get this code as a mutable string
    pub fn as_mut_string(&mut self) -> &mut String {
        match self {
            Self::Valid(s) => s,
            Self::Expired(s) => s,
        }
    }

    /// Parse this code from an element
    pub(crate) fn from_element(element: ElementRef) -> Option<Self> {
        let span_selector =
            Selector::parse("span[style=\"color:red\"]").expect("invalid span selector");
        let maybe_span = element.select(&span_selector).next();

        match maybe_span {
            Some(el) => {
                let code = el.text().next()?.trim();
                Some(Self::Expired(code.into()))
            }
            None => {
                let code = element.text().next()?.trim();
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
