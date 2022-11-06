/// A type id for a de/serializable type.
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct TypeIdString(String);
impl TypeIdString {
    /// Converts this to a string.
    pub fn to_string(self) -> String {
        self.0
    }
}
impl<'a> From<&'a str> for TypeIdString {
    /// Creates a type id from the given string. Process it.
    fn from(s: &'a str) -> Self {
        Self(
            s.trim()
                .replace('\r', "")
                .replace('\n', "")
                .to_lowercase()
                .to_string(),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    MismatchedType {
        expected: TypeIdString,
        got: TypeIdString,
    },
    MissingTypeId,
}

pub trait Deserializable {
    fn type_id() -> TypeIdString;
    fn from_values(values: Vec<String>) -> Self;
    fn from_string<'a, T>(s: &'a str) -> Result<T, Error>
    where
        T: Deserializable,
    {
        let mut set_type_id = false;
        let mut values: Vec<String> = vec![];
        for line in s.lines() {
            if !set_type_id {
                set_type_id = true;

                let type_id = line.into();
                if type_id != T::type_id() {
                    return Err(Error::MismatchedType {
                        expected: T::type_id(),
                        got: type_id,
                    });
                }
            }

            values.push(line.to_string());
        }

        Ok(T::from_values(values))
    }
}

pub trait Serializable {
    fn type_id() -> TypeIdString;
    fn generate_values(&self) -> Vec<String>;
    fn to_string<'a, T>(&self) -> String
    where
        T: Deserializable,
    {
        let mut values = self.generate_values();
        values.insert(0, Self::type_id().to_string());

        values.join("\n")
    }
}
