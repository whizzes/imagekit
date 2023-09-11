use std::{borrow::Cow, fmt::Display};

use anyhow::{bail, Result};
use async_trait::async_trait;
use reqwest::StatusCode;

use crate::{client::FILES_ENDPOINT, upload::types::Response, ErrorResponse, ImageKit};

/// Options for list/search files.
///
/// Refer: https://docs.imagekit.io/api-reference/media-api/list-and-search-files
#[derive(Default)]
pub struct Options {
    search_query: Option<SearchQuery>,
    path: Option<String>,
    tags: Option<String>,
    skip: Option<u32>,
    limit: Option<u32>,
}

impl Options {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn search_query(mut self, val: SearchQuery) -> Self {
        self.search_query = Some(val);
        self
    }

    pub fn path<T: ToString>(mut self, val: T) -> Self {
        self.path = Some(val.to_string());
        self
    }

    pub fn tags<T: ToString>(mut self, val: T) -> Self {
        self.tags = Some(val.to_string());
        self
    }

    pub fn skip(mut self, val: u32) -> Self {
        self.skip = Some(val);
        self
    }

    pub fn limit(mut self, val: u32) -> Self {
        self.limit = Some(val);
        self
    }
}

pub struct SearchQueryBuilder {
    query_string: String,
}

impl SearchQueryBuilder {
    pub fn build(self) -> SearchQuery {
        SearchQuery(self.query_string.into())
    }

    pub fn and(mut self, search_query: SearchQuery) -> Self {
        self.query_string
            .push_str(&format!(" and ({})", search_query));
        self
    }

    pub fn or(mut self, search_query: SearchQuery) -> Self {
        self.query_string
            .push_str(&format!(" or ({})", search_query));
        self
    }

    pub fn raw_query_string<T: ToString>(val: T) -> Self {
        Self {
            query_string: val.to_string(),
        }
    }

    pub fn name<T: ToString>(operator: Operator, val: T) -> Self {
        let val = val.to_string();
        Self::raw_query_string(format!("name {operator} {val}"))
    }

    pub fn tags<T: ToString>(operator: Operator, val: &[T]) -> Self {
        let tags = val.iter().fold(String::default(), |acc, tag| {
            format!("{acc},\"{}\"", tag.to_string())
        });
        let tags = tags.trim_start_matches(',');
        Self::raw_query_string(format!("tags {operator} [{tags}]"))
    }

    pub fn created_at<T: ToString>(operator: Operator, val: T) -> Self {
        Self::raw_query_string(format!("createdAt {operator} {}", val.to_string()))
    }

    pub fn updated_at<T: ToString>(operator: Operator, val: T) -> Self {
        Self::raw_query_string(format!("updatedAt {operator} {}", val.to_string()))
    }

    pub fn height<T: ToString>(operator: Operator, val: T) -> Self {
        Self::raw_query_string(format!("height {operator} {}", val.to_string()))
    }

    pub fn width<T: ToString>(operator: Operator, val: T) -> Self {
        Self::raw_query_string(format!("width {operator} {}", val.to_string()))
    }

    /// size in bytes
    pub fn size(operator: Operator, val: u32) -> Self {
        Self::raw_query_string(format!("size {operator} {val}"))
    }

    /// size in kb, mb, etc. e.g., 1mb
    pub fn size_special<T: ToString>(operator: Operator, val: T) -> Self {
        Self::raw_query_string(format!("size {operator} \"{}\"", val.to_string()))
    }

    pub fn private(val: bool) -> Self {
        Self::raw_query_string(format!("private = {val}"))
    }

    pub fn published(val: bool) -> Self {
        Self::raw_query_string(format!("published = {val}"))
    }

    pub fn transparency(val: bool) -> Self {
        Self::raw_query_string(format!("transparency = {val}"))
    }
}

pub struct SearchQuery(pub Cow<'static, str>);

impl Display for SearchQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub enum Operator {
    EqualTo,
    NotEqualTo,
    Colon,
    In,
    NotIn,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::EqualTo => write!(f, "="),
            Operator::NotEqualTo => write!(f, "NOT ="),
            Operator::Colon => write!(f, ":"),
            Operator::In => write!(f, "IN"),
            Operator::NotIn => write!(f, "NOT IN"),
            Operator::GreaterThan => write!(f, ">"),
            Operator::GreaterThanOrEqualTo => write!(f, ">="),
            Operator::LessThan => write!(f, "<"),
            Operator::LessThanOrEqualTo => write!(f, "<="),
        }
    }
}

#[async_trait]
pub trait ListFiles {
    /// list and search files
    async fn list_files(&self, opts: Options) -> Result<Vec<Response>>;
}

#[async_trait]
impl ListFiles for ImageKit {
    async fn list_files(&self, opts: Options) -> Result<Vec<Response>> {
        let mut query_params = Vec::new();
        if let Some(search_query) = opts.search_query {
            query_params.push(("searchQuery", search_query.to_string()))
        }
        if let Some(path) = opts.path {
            query_params.push(("path", path))
        }
        if let Some(tags) = opts.tags {
            query_params.push(("tags", tags))
        }
        if let Some(skip) = opts.skip {
            query_params.push(("skip", skip.to_string()))
        }
        if let Some(limit) = opts.limit {
            query_params.push(("limit", limit.to_string()))
        };

        let response = self
            .client
            .get(FILES_ENDPOINT.to_string())
            .query(&query_params)
            .send()
            .await?;

        if matches!(response.status(), StatusCode::OK) {
            let result = response.json::<Vec<Response>>().await?;
            return Ok(result);
        }

        let result = response.json::<ErrorResponse>().await?;

        bail!(result.message);
    }
}
