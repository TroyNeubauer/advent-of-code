use anyhow::{anyhow, bail, Result};
use log::{error, warn};
use select::{
    document::Document,
    node::Node,
    predicate::{Any, Class, Name, Predicate},
};

use super::Edition;

pub struct Low {
    doc: Document,
    p1_index: usize,
    p2_index: Option<usize>,
    /// The edition that this was parsed as
    edition: Edition,
}

pub enum Query {
    /// Queries only within the description of the part 1 problem
    Part1,
    /// Queries only within the description of the part 2 problem
    ///
    /// NOTE: Querying for `Part2Only` in a problem that only has part one revealed will yield no
    /// results
    Part2,
    /// Queries in both the part 1 and part 2 descriotions of the problem
    Both,
    /// Queries the entire html document
    EntirePage,
}

/// Returns the p1 and p2 nodes set or not so that `query` is ready to be executed
macro_rules! prep_query {
    ($this:ident, $query:ident, $closure:ident) => {{
        let p1 = match $query {
            Query::Part1 | Query::Both => $this.p1_node(),
            Query::Part2 => None,
            Query::EntirePage => None,
        };

        let p2 = match $query {
            Query::Part1 => None,
            Query::Part2 | Query::Both => $this.p2_node(),
            Query::EntirePage => None,
        };

        p1.into_iter().chain(p2.into_iter())
    }};
}

impl Low {
    /// Tries to create a new low level parser
    pub fn new(html: &str) -> Result<Self> {
        let doc = Document::from(html);

        if let Some(edition) = Edition::guess(&doc) {
            // If we suspect a given edition, then parse using that editions rules
            return inner::parse(edition, doc).map_err(|(_, e)| e);
        }
        warn!("failed to guess edition");

        let mut doc = doc;
        // Try parsing as each edition
        for edition in Edition::all() {
            match inner::parse(edition, doc) {
                Err((new_doc, _err)) => doc = new_doc,
                Ok(low) => return Ok(low),
            }
        }
        bail!("could not parse as any editions");
    }

    pub fn p1_node(&self) -> Option<Node<'_>> {
        self.doc.nth(self.p1_index)
    }

    pub fn p2_node(&self) -> Option<Node<'_>> {
        self.p2_index.map(|i| self.doc.nth(i)).flatten()
    }

    /// Returns all code blocks within the scope of query, specifically this function searches for
    /// all `<pre>` tags that contain `<code>` tags inside of `query`
    pub fn code_blocks(&self, query: Query) -> impl Iterator<Item = Node<'_>> + '_ {
        prep_query!(self, query, f)
            .map(|node| node.find(Name("pre").descendant(Name("code"))))
            .flatten()
    }

    /// Returns all code blocks within the scope of query, specifically this function searches for
    /// all `<pre>` tags that contain `<code>` tags inside of `query`
    pub fn test_case_answer_blocks(&self, query: Query) -> impl Iterator<Item = Node<'_>> + '_ {
        let a = prep_query!(self, query, f)
            .map(|node| node.find(Name("code").descendant(Name("em"))))
            .flatten();
        // most answers are <code><em>, but 2022 day1 has the test case answer as <em><code> sadly
        let b = prep_query!(self, query, f)
            .map(|node| node.find(Name("em").descendant(Name("code"))))
            .flatten();
        a.chain(b)
    }

    /// Returns all puzzel answers by matching paragraphs with text `Your puzzle answer was:`
    pub fn puzzel_answers(&self) -> impl Iterator<Item = String> + '_ {
        self.doc
            .find(Name("p").descendant(Name("code")))
            .filter_map(|code_node| {
                let parent = code_node.parent()?;
                if parent.text().starts_with("Your puzzle answer was") {
                    Some(code_node.text())
                } else {
                    None
                }
            })
    }

    pub fn embedded_puzzel_input(&self) -> Option<String> {
        self.doc
            .find(Name("code").and(Class("puzzle-input")))
            .next()
            .map(|node| node.text())
    }

    pub fn day_success(&self) -> impl Iterator<Item = Node<'_>> + '_ {
        self.doc.find(Name("p").and(Class("day-success")))
    }
}

mod inner {
    use super::*;

    type ParseError = (Document, anyhow::Error);
    type Result = std::result::Result<Low, ParseError>;

    pub fn parse(edition: Edition, doc: Document) -> Result {
        match edition {
            Edition::Pre2020 => parse_pre_2020(doc),
            Edition::Post2020 => parse_post_2020(doc),
        }
    }

    fn parse_pre_2020(doc: Document) -> Result {
        todo!()
    }

    fn parse_post_2020(doc: Document) -> Result {
        let mut articles = doc.find(Name("article").and(Class("day-desc")));
        let p1_index = articles.next().map(|n| n.index());
        let Some(p1_index) = p1_index else {
            return Err((doc, anyhow!("failed to find part 1 article")));
        };
        let p2_index = articles.next().map(|n| n.index());

        for extra_node in articles {
            warn!(
                "html has extra part!? {:?}: `{}`",
                extra_node,
                extra_node.text()
            );
        }

        Ok(Low {
            doc,
            p1_index,
            p2_index,
            edition: Edition::Post2020,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[track_caller]
    fn assert_found_nodes<'a>(strs: &[&str], mut iter: impl Iterator<Item = Node<'a>>) {
        let mut strs_iter = strs.into_iter();
        let mut i = 0;
        loop {
            let expected = strs_iter.next().copied();
            let actual = iter.next();
            match (expected, actual) {
                (Some(expected), Some(actual)) => {
                    if expected != actual.text() {
                        println!("at index: {i}");
                        assert_eq!(expected, actual.text())
                    }
                }
                (None, Some(actual)) => {
                    println!("found extra value. expected nothing more. value:");
                    panic!("{:?}", actual.text())
                }
                (Some(expected), None) => {
                    println!("expected additional value. expected:");
                    panic!("{:?}", expected)
                }
                (None, None) => break,
            }
            i += 1;
        }
    }

    fn assert_test_case_answers(l: &Low, p1: &str, p2: &str) {
        assert_found_nodes(&[p1], l.test_case_answer_blocks(Query::Part1));
        assert_found_nodes(&[p2], l.test_case_answer_blocks(Query::Part2));
        assert_found_nodes(&[p1, p2], l.test_case_answer_blocks(Query::Both));
    }

    #[test_log::test]
    fn day10_2015_part1() {
        let l = Low::new(include_str!("../../test_files/part1/2015/day10.html")).unwrap();
        assert!(l.p1_node().is_some());
        assert!(l.p2_node().is_none());
        assert_eq!(l.embedded_puzzel_input().unwrap(), "1321131112");

        assert_found_nodes(&[], l.code_blocks(Query::Both));

        assert_found_nodes(&[], l.test_case_answer_blocks(Query::Part1));
        assert_found_nodes(&[], l.test_case_answer_blocks(Query::Part2));
        assert_found_nodes(&[], l.test_case_answer_blocks(Query::Both));
    }

    #[test_log::test]
    fn day11_2018_part1() {
        let l = Low::new(include_str!("../../test_files/part1/2018/day11.html")).unwrap();
        assert!(l.p1_node().is_some());
        assert!(l.p2_node().is_none());
        assert_eq!(l.embedded_puzzel_input().unwrap(), "3031");

        assert_found_nodes(
            &[
                r#"
-2  -4   4   4   4
-4   4   4   4  -5
 4   3   3   4  -4
 1   1   2   4  -3
-1   0   2  -5  -2
"#
                .trim_start(),
                r#"
-3   4   2   2   2
-4   4   3   3   4
-5   3   3   4  -4
 4   3   3   4  -3
 3   3   3  -5  -1
"#
                .trim_start(),
            ],
            l.code_blocks(Query::Both),
        );

        // this one is complicated and doesn't contain any easy test cases, but we still want
        // `answer_blocks` to return all the `<code><em>` blocks so that we don't have seperate
        // logic from the easy case
        let anwsers: Vec<_> = l.test_case_answer_blocks(Query::Part1).collect();
        assert_eq!(anwsers.len(), 18);

        assert_found_nodes(&[], l.test_case_answer_blocks(Query::Part2));
    }

    #[test_log::test]
    fn day1_2019_part2() {
        let l = Low::new(include_str!("../../test_files/part2/2019/day1.html")).unwrap();
        assert!(l.p1_node().is_some());
        assert!(l.p2_node().is_some());
        assert!(l.embedded_puzzel_input().is_none());

        assert_found_nodes(&[], l.code_blocks(Query::Both));
        assert_eq!(l.puzzel_answers().collect::<Vec<_>>(), &["3412531"]);

        assert_found_nodes(&[], l.test_case_answer_blocks(Query::Both));
    }

    #[test_log::test]
    fn day2_2021_complete() {
        let l = Low::new(include_str!("../../test_files/complete/2021/day2.html")).unwrap();
        assert!(l.p1_node().is_some());
        assert!(l.p2_node().is_some());
        assert!(l.embedded_puzzel_input().is_none());

        assert_found_nodes(
            &[r#"
forward 5
down 5
forward 8
up 3
down 8
forward 2
"#
            .trim_start()],
            l.code_blocks(Query::Both),
        );

        assert_test_case_answers(&l, "150", "900");
    }

    #[test_log::test]
    fn day1_2022_complete() {
        let l = Low::new(include_str!("../../test_files/complete/2022/day1.html")).unwrap();
        assert!(l.p1_node().is_some());
        assert!(l.p2_node().is_some());
        assert!(l.embedded_puzzel_input().is_none());

        assert_found_nodes(
            &[r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#
            .trim_start()],
            l.code_blocks(Query::Both),
        );

        assert_found_nodes(
            &["6000", "4000", "11000", "24000", "10000", "24000"],
            l.test_case_answer_blocks(Query::Part1),
        );

        assert_found_nodes(&["45000"], l.test_case_answer_blocks(Query::Part2));
    }

    #[test_log::test]
    fn day2_2022_complete() {
        let l = Low::new(include_str!("../../test_files/complete/2022/day2.html")).unwrap();
        assert!(l.p1_node().is_some());
        assert!(l.p2_node().is_some());
        assert!(l.embedded_puzzel_input().is_none());

        assert_found_nodes(
            &[r#"
A Y
B X
C Z
"#
            .trim_start()],
            l.code_blocks(Query::Both),
        );

        assert_test_case_answers(&l, "15", "12");
    }

    #[test_log::test]
    fn day3_2022_complete() {
        let l = Low::new(include_str!("../../test_files/complete/2022/day3.html")).unwrap();
        assert!(l.p1_node().is_some());
        assert!(l.p2_node().is_some());
        assert!(l.embedded_puzzel_input().is_none());

        assert_found_nodes(
            &[
                r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#
                .trim_start(),
                r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
"#
                .trim_start(),
                r#"
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#
                .trim_start(),
            ],
            l.code_blocks(Query::Both),
        );

        assert_found_nodes(
            &["p", "L", "P", "v", "t", "s", "157"],
            l.test_case_answer_blocks(Query::Part1),
        );
        assert_found_nodes(&["70"], l.test_case_answer_blocks(Query::Part2));
    }

    #[test_log::test]
    fn day4_2022_complete() {
        let l = Low::new(include_str!("../../test_files/complete/2022/day4.html")).unwrap();
        assert!(l.p1_node().is_some());
        assert!(l.p2_node().is_some());
        assert!(l.embedded_puzzel_input().is_none());

        assert_found_nodes(
            &[
                r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#
                .trim_start(),
                r#"
.234.....  2-4
.....678.  6-8

.23......  2-3
...45....  4-5

....567..  5-7
......789  7-9

.2345678.  2-8
..34567..  3-7

.....6...  6-6
...456...  4-6

.23456...  2-6
...45678.  4-8
"#
                .trim_start(),
            ],
            l.code_blocks(Query::Both),
        );

        assert_test_case_answers(&l, "2", "4");
    }
}
