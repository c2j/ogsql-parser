use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;
use serde::{Deserialize, Serialize, Deserializer, Serializer};
use serde::de::{self, Visitor};

/// An identifier, decomposed into its value and quote style.
///
/// `quote_style` is `None` for unquoted identifiers (e.g. `mytable`),
/// `Some('"')` for double-quoted identifiers (e.g. `"MyTable"`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    /// The identifier value without quotes.
    pub value: String,
    /// The starting quote if any. `None` = unquoted, `Some('"')` = double-quoted.
    pub quote_style: Option<char>,
}

impl Ident {
    /// Create a new unquoted identifier.
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into(), quote_style: None }
    }

    /// Create a new quoted identifier.
    pub fn quoted(value: impl Into<String>, quote_style: char) -> Self {
        Self { value: value.into(), quote_style: Some(quote_style) }
    }

    /// Returns the identifier value as &str.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl Deref for Ident {
    type Target = str;
    fn deref(&self) -> &str { &self.value }
}

impl Borrow<str> for Ident {
    fn borrow(&self) -> &str { &self.value }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.quote_style {
            None => f.write_str(&self.value),
            Some(q) => {
                let escaped = self.value.replace(q, &format!("{q}{q}"));
                write!(f, "{q}{escaped}{q}")
            }
        }
    }
}

impl From<String> for Ident {
    fn from(s: String) -> Self { Self::new(s) }
}

impl From<&str> for Ident {
    fn from(s: &str) -> Self { Self::new(s) }
}

impl From<&String> for Ident {
    fn from(s: &String) -> Self { Self::new(s) }
}

impl PartialEq<str> for Ident {
    fn eq(&self, other: &str) -> bool { self.value == other }
}

impl PartialEq<&str> for Ident {
    fn eq(&self, other: &&str) -> bool { self.value == *other }
}

impl PartialEq<String> for Ident {
    fn eq(&self, other: &String) -> bool { self.value == *other }
}

impl Default for Ident {
    fn default() -> Self {
        Self { value: String::new(), quote_style: None }
    }
}

// --- Custom Serde ---

// Serialize: unquoted → plain string, quoted → object {value, quote_style}
impl Serialize for Ident {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.quote_style {
            None => serializer.serialize_str(&self.value),
            Some(q) => {
                use serde::ser::SerializeStruct;
                let mut s = serializer.serialize_struct("Ident", 2)?;
                s.serialize_field("value", &self.value)?;
                s.serialize_field("quote_style", &q)?;
                s.end()
            }
        }
    }
}

// Deserialize: accept both plain string AND object {value, quote_style}
impl<'de> Deserialize<'de> for Ident {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct IdentVisitor;

        impl<'de> Visitor<'de> for IdentVisitor {
            type Value = Ident;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a string or an object with value and quote_style")
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Ident, E> {
                Ok(Ident::new(value.to_string()))
            }

            fn visit_string<E: de::Error>(self, value: String) -> Result<Ident, E> {
                Ok(Ident::new(value))
            }

            fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Ident, A::Error> {
                let mut value = None;
                let mut quote_style: Option<Option<char>> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "value" => value = Some(map.next_value()?),
                        "quote_style" => quote_style = Some(map.next_value()?),
                        _ => return Err(de::Error::unknown_field(&key, &["value", "quote_style"])),
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(Ident { value, quote_style: quote_style.unwrap_or(None) })
            }
        }
        deserializer.deserialize_any(IdentVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deref_str_methods() {
        let ident = Ident::new("MyTable");
        assert_eq!(ident.to_lowercase(), "mytable");
        assert_eq!(ident.len(), 7);
        assert!(!ident.is_empty());
        assert!(ident.starts_with("My"));
    }

    #[test]
    fn test_borrow_str_for_join() {
        // This is the critical test: Vec<Ident>.join(".") must work
        let parts: Vec<Ident> = vec![Ident::new("schema"), Ident::new("table")];
        assert_eq!(parts.join("."), "schema.table");
    }

    #[test]
    fn test_display_unquoted() {
        let ident = Ident::new("mytable");
        assert_eq!(format!("{}", ident), "mytable");
    }

    #[test]
    fn test_display_quoted() {
        let ident = Ident::quoted("MyTable", '"');
        assert_eq!(format!("{}", ident), "\"MyTable\"");
    }

    #[test]
    fn test_display_quoted_with_escaped_quotes() {
        let ident = Ident::quoted("my\"table", '"');
        assert_eq!(format!("{}", ident), "\"my\"\"table\"");
    }

    #[test]
    fn test_from_string() {
        let ident: Ident = "foo".to_string().into();
        assert_eq!(ident.value, "foo");
        assert_eq!(ident.quote_style, None);
    }

    #[test]
    fn test_partial_eq_str() {
        let ident = Ident::new("foo");
        assert!(ident == "foo");
        assert!(ident != "bar");
    }

    #[test]
    fn test_default() {
        let ident = Ident::default();
        assert_eq!(ident.value, "");
        assert_eq!(ident.quote_style, None);
    }

    #[test]
    fn test_serde_unquoted_roundtrip() {
        let ident = Ident::new("mytable");
        let json = serde_json::to_string(&ident).unwrap();
        assert_eq!(json, r#""mytable""#);
        let de: Ident = serde_json::from_str(&json).unwrap();
        assert_eq!(de, ident);
    }

    #[test]
    fn test_serde_quoted_roundtrip() {
        let ident = Ident::quoted("MyTable", '"');
        let json = serde_json::to_string(&ident).unwrap();
        assert_eq!(json, r#"{"value":"MyTable","quote_style":"\""}"#);
        let de: Ident = serde_json::from_str(&json).unwrap();
        assert_eq!(de, ident);
    }

    #[test]
    fn test_deserialize_plain_string_backward_compat() {
        let de: Ident = serde_json::from_str(r#""mytable""#).unwrap();
        assert_eq!(de.value, "mytable");
        assert_eq!(de.quote_style, None);
    }

    #[test]
    fn test_deserialize_object_without_quote_style() {
        let de: Ident = serde_json::from_str(r#"{"value":"mytable"}"#).unwrap();
        assert_eq!(de.value, "mytable");
        assert_eq!(de.quote_style, None);
    }

    #[test]
    fn test_cloned_unwrap_or_default_to_lowercase() {
        // Simulates the pattern: name.last().cloned().unwrap_or_default().to_lowercase()
        let name: Vec<Ident> = vec![Ident::new("MyTable")];
        let result = name.last().cloned().unwrap_or_default().to_lowercase();
        assert_eq!(result, "mytable");
    }
}
