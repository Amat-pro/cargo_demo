use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};


#[derive(Clone, Debug, Default)]
pub struct Labels(Arc<Map>);

pub type Map = BTreeMap<String, String>;

pub type Expressions = Vec<Expression>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
pub enum Operator {
    In,
    NotIn,
    Exists,
    DoesNotExists,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
pub struct Expression {
    key: String,
    operator: Operator,
    values: Option<BTreeSet<String>>,
}

/// Selects a set of pods that expose a server. The result of `match_labels` and `match_expressions` are ANDed.
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Selector {
    match_labels: Option<Map>,
    match_expressions: Option<Expressions>,
}

// === Expression ===

impl Expression {
    fn matches(&self, labels: &Map) -> bool {
        match (&self.key, self.operator, self.values.as_ref()) {
            (key, Operator::In, Some(values)) => match labels.get(key) {
                Some(v) => values.contains(v),
                None => false,
            },

            (key, Operator::NotIn, Some(values)) => match labels.get(key) {
                Some(v) => !values.contains(v),
                None => true,
            },

            (key, Operator::Exists, None) => labels.contains_key(key),

            (key, Operator::DoesNotExists, None) => !labels.contains_key(key),

            (key, operator, values) => {
                tracing::warn!(?operator, %key, ?values, "illegal match expression");
                false
            }
        }
    }
}

// === Labels ===

impl From<Option<Map>> for Labels {
    #[inline]
    fn from(labels: Option<Map>) -> Self {
        labels.unwrap_or_default().into()
    }
}

impl From<Map> for Labels {
    #[inline]
    fn from(labels: Map) -> Self {
        Self(Arc::new(labels))
    }
}

impl AsRef<Map> for Labels {
    #[inline]
    fn as_ref(&self) -> &Map {
        self.0.as_ref()
    }
}

impl PartialEq<Self> for Labels {
    #[inline]
    fn eq(&self, t: &Self) -> bool {
        self.0.as_ref().eq(t.as_ref())
    }
}

impl PartialEq<Option<Map>> for Labels {
    #[inline]
    fn eq(&self, t: &Option<Map>) -> bool {
        match t {
            Some(t) => t.eq(self.0.as_ref()),
            None => self.0.is_empty(),
        }
    }
}

impl FromIterator<(String, String)> for Labels {
    fn from_iter<T: IntoIterator<Item=(String, String)>>(iter: T) -> Self {
        Self(Arc::new(iter.into_iter().collect()))
    }
}

impl FromIterator<(&'static str, &'static str)> for Labels {
    fn from_iter<T: IntoIterator<Item=(&'static str, &'static str)>>(iter: T) -> Self {
        iter.into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }
}

// === Selector ===

impl Selector {
    fn new(labels: Map, expressions: Expressions) -> Self {
        Self {
            match_labels: Some(labels),
            match_expressions: Some(expressions),
        }
    }

    fn from_map(labels: Map) -> Self {
        Self {
            match_labels: Some(labels),
            match_expressions: None,
        }
    }

    fn from_expressions(expressions: Expressions) -> Self {
        Self {
            match_labels: None,
            match_expressions: Some(expressions),
        }
    }

    // Indicates whether this label selector matches all pods
    pub fn selects_all(&self) -> bool {
        match (self.match_labels.as_ref(), self.match_expressions.as_ref()) {
            (None, None) => true,
            (Some(labels), None) => labels.is_empty(),
            (None, Some(expressions)) => expressions.is_empty(),
            (Some(labels), Some(expressions)) => labels.is_empty() && expressions.is_empty(),
        }
    }

    pub fn matches(&self, labels: &Labels) -> bool {
        for expression in self.match_expressions.iter().flatten() {
            if !expression.matches(labels.as_ref()) {
                return false;
            }
        }

        if let Some(match_labels) = self.match_labels.as_ref() {
            for (k, v) in match_labels {
                if labels.0.get(k) != Some(v) {
                    return false;
                }
            }
        }

        true
    }
}

impl FromIterator<(String, String)> for Selector {
    fn from_iter<T: IntoIterator<Item=(String, String)>>(iter: T) -> Self {
        Self::from_map(iter.into_iter().collect())
    }
}

impl FromIterator<(&'static str, &'static str)> for Selector {
    fn from_iter<T: IntoIterator<Item=(&'static str, &'static str)>>(iter: T) -> Self {
        Self::from_map(
            iter.into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect()
        )
    }
}

impl FromIterator<Expression> for Selector {
    fn from_iter<T: IntoIterator<Item=Expression>>(iter: T) -> Self {
        Self::from_expressions(iter.into_iter().collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn test_matches() {
        for (selector, labels, matches, msg) in &[
            //
            (Selector::default(), Labels::default(), true, "empty match"),

            //
            (
                Selector::from_iter(Some(("foo", "bar"))),
                Labels::from_iter(Some(("foo", "bar"))),
                true,
                "extract label match",
            ),

            //
            (
                Selector::from_iter(Some(("foo", "bar"))),
                Labels::from_iter(vec![("foo", "bar"), ("bah", "baz")]),
                true,
                "sufficient label match",
            ),

            //
            (
                Selector::from_iter(Some(Expression {
                    key: "foo".into(),
                    operator: Operator::In,
                    values: Some(Some("bar".to_string()).into_iter().collect()),
                })),
                Labels::from_iter(vec![("foo", "bar"), ("bah", "baz")]),
                true,
                "In expression match"
            ),

            //
            (
                Selector::from_iter(Some(Expression {
                    key: "foo".into(),
                    operator: Operator::NotIn,
                    values: Some(Some("quux".to_string()).into_iter().collect()),
                })),
                Labels::from_iter(vec![("foo", "bar"), ("bah", "baz")]),
                true,
                "NotIn expression match"
            ),

            //
            (
                Selector::from_iter(Some(Expression {
                    key: "foo".into(),
                    operator: Operator::NotIn,
                    values: Some(Some("bar".to_string()).into_iter().collect()),
                })),
                Labels::from_iter(vec![("foo", "bar"), ("bah", "baz")]),
                false,
                "NotIn expression non-match"
            ),

            //
            (
                Selector::new(
                    Map::from([("foo".to_string(), "bar".to_string())]),
                    vec![Expression {
                        key: "bah".into(),
                        operator: Operator::In,
                        values: Some(Some("bar".to_string()).into_iter().collect()),
                    }],
                ),
                Labels::from_iter(vec![("foo", "bar"), ("bah", "baz")]),
                false,
                "match labels but not expressions"
            ),
            (
                Selector::new(
                    Map::from([("foo".to_string(), "bar".to_string())]),
                    vec![Expression {
                        key: "bah".into(),
                        operator: Operator::In,
                        values: Some(Some("bar".to_string()).into_iter().collect()),
                    }],
                ),
                Labels::from_iter(vec![("foo", "bar"), ("bah", "bar")]),
                true,
                "matches both labels and expressions"
            ),
        ] {
            assert_eq!(selector.matches(labels), *matches, "{}", msg);
        }
    }
}
