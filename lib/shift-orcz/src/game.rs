/// Borderlands Games
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Game {
    Borderlands,
    Borderlands2,
    BorderlandsPreSequel,
    Borderlands3,
}

impl Game {
    /// Get the orcz page url
    pub(crate) fn page_url(&self) -> &'static str {
        match self {
            Self::Borderlands => "http://orcz.com/Borderlands:_Golden_Key",
            Self::Borderlands2 => "http://orcz.com/borderlands_2:_Golden_Key",
            Self::BorderlandsPreSequel => "http://orcz.com/Borderlands_Pre-Sequel:_Shift_Codes",
            Self::Borderlands3 => "http://orcz.com/Borderlands_3:_Shift_Codes",
        }
    }

    /// Check whether this game is bl3
    pub fn is_bl3(self) -> bool {
        matches!(self, Self::Borderlands3)
    }
}
