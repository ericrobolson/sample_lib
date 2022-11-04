use std::ops::Deref;

/// Errors that occur during CSV parsing.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    NoHeaders,
    MismatchedItemLength {
        line: usize,
        item_count: usize,
        expected_item_count: usize,
    },
}

/// A CSV that can be build from a string.
#[derive(Debug, Clone, PartialEq)]
pub struct Csv<const WIDTH: usize> {
    headers: Box<[String; WIDTH]>,
    lines: Vec<[String; WIDTH]>,
}
impl<const WIDTH: usize> Csv<WIDTH> {
    /// Creates a new CSV.
    pub fn new<'a>(headers: &'a [String; WIDTH]) -> Self {
        Self {
            headers: Box::new(headers.clone()),
            lines: vec![],
        }
    }

    /// Attempts to parse a CSV from a string.
    pub fn from_string(s: String) -> Result<Self, Error> {
        let csv_lines = s
            .lines()
            .enumerate()
            .map(|(line, l)| {
                let items = l.split(",").collect::<Vec<&str>>();
                if items.len() != WIDTH {
                    return Err(Error::MismatchedItemLength {
                        line,
                        item_count: items.len(),
                        expected_item_count: WIDTH,
                    });
                }
                const EMPTY_STRING: String = String::new();

                let mut line: [String; WIDTH] = [EMPTY_STRING; WIDTH];
                for (idx, item) in items.iter().enumerate() {
                    line[idx] = item.to_string()
                }

                Ok(line)
            })
            .collect::<Vec<Result<[String; WIDTH], Error>>>();

        let mut lines = Vec::with_capacity(csv_lines.len());
        for line in csv_lines.iter() {
            match line {
                Ok(l) => lines.push(l.clone()),
                Err(e) => return Err(e.clone()),
            }
        }

        if lines.is_empty() {
            return Err(Error::NoHeaders);
        }

        Ok(Self {
            headers: Box::new(lines.remove(0)),
            lines,
        })
    }

    /// The headers in the CSV.
    pub fn headers(&self) -> &Box<[String; WIDTH]> {
        &self.headers
    }

    /// The values in the CSV.
    pub fn values(&self) -> impl Iterator<Item = &[String; WIDTH]> {
        self.lines.iter()
    }

    /// Adds an item to the given CSV object.
    pub fn append(&mut self, item: [String; WIDTH]) {
        self.lines.push(item);
    }

    /// Converts the CSV to a string.
    pub fn to_string(&self) -> String {
        let mut csv = self.clone();

        // Put header into lines so we can iterate over it all as once.
        csv.lines.insert(0, csv.headers.as_ref().clone());

        csv.lines
            .iter()
            .map(|l| l.join(","))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
