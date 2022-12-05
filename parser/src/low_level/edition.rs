use select::{document::Document, predicate::Name};

/// An edition is a groups of incompatable page layouts
///
/// All advent of code days within the same edition are similar to parse
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Edition {
    /// Pre 2020 challenges don't usually have full test cases
    Post2015,
}

impl Edition {
    /// Guesses which advent of code edition `doc` uses
    pub fn guess(doc: &Document) -> Option<Self> {
        let title = doc
            .find(Name("title"))
            .next()
            .map(|title| title.inner_html())?;

        Some(
            if title.contains("2015")
                || title.contains("2016")
                || title.contains("2017")
                || title.contains("2018")
                || title.contains("2019")
                || title.contains("2020")
                || title.contains("2021")
                || title.contains("2022")
                || title.contains("2023")
            {
                Edition::Post2015
            } else {
                return None;
            },
        )
    }

    /// Returns all editions
    pub fn all() -> impl Iterator<Item = Self> {
        [Self::Post2015].into_iter()
    }
}
