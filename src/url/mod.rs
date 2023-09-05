use std::collections::HashMap;

use anyhow::Result;

use crate::ImageKit;

#[derive(Default)]
pub struct Transformation {
    width: Option<u32>,
    height: Option<u32>,
    aspect_ratio: Option<String>,
}

impl Transformation {
    pub fn new() -> Self {
        Self { ..Self::default() }
    }

    pub fn width(mut self, val: u32) -> Self {
        self.width = Some(val);
        self
    }

    pub fn height(mut self, val: u32) -> Self {
        self.height = Some(val);
        self
    }

    pub fn aspect_raio<T: AsRef<str> + Into<String>>(mut self, val: T) -> Self {
        self.aspect_ratio = Some(val.into());
        self
    }

    pub fn transform(&self) -> Result<String> {
        let mut output = String::default();

        if let Some(width) = self.width {
            output.push_str(&format!(",w-{width}"))
        }

        if let Some(height) = self.height {
            output.push_str(&format!(",h-{height}"))
        }

        if let Some(aspect_ratio) = &self.aspect_ratio {
            output.push_str(&format!(",ar-{aspect_ratio}"))
        }

        if output.is_empty() {
            return Err(anyhow::anyhow!("No transformation applied"));
        }

        let output = output.trim_matches(',');
        Ok(output.into())
    }
}

pub enum TransformationPosition {
    Path,
    Query,
}

impl TransformationPosition {
    fn default() -> TransformationPosition {
        Self::Path
    }
}

/// Options for generating the url.
///
/// Refer: https://docs.imagekit.io/features/image-transformations
pub struct Options {
    /// URL endpoint for this particular image
    /// By default the url used in sdk initialization is used
    url_endpoint: Option<String>,
    path: Option<String>,
    src: Option<String>,
    transformation: Transformation,
    /// position for url transformation. i.e., query or path
    transformation_position: TransformationPosition,
    /// any other query parameters that need to be added to the URL
    query_parameters: Option<HashMap<String, String>>,
}

impl Options {
    /// Creates a new instance of `Options`
    pub fn new(transformation: Transformation) -> Self {
        Self {
            transformation,
            ..Default::default()
        }
    }

    /// Sets the endpoint for the image
    pub fn url_endpoint<T: AsRef<str> + Into<String>>(mut self, val: T) -> Self {
        self.url_endpoint = Some(val.into());
        self
    }

    /// Sets the path
    pub fn path<T: AsRef<str> + Into<String>>(mut self, val: T) -> Self {
        self.path = Some(val.into());
        self
    }

    /// Sets the src
    pub fn src<T: AsRef<str> + Into<String>>(mut self, val: T) -> Self {
        self.src = Some(val.into());
        self
    }

    /// Sets the src
    pub fn query_parameters(mut self, val: HashMap<String, String>) -> Self {
        self.query_parameters = Some(val);
        self
    }

    /// Sets the transformation position
    pub fn transformation_position(mut self, val: TransformationPosition) -> Self {
        self.transformation_position = val;
        self
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            url_endpoint: None,
            path: None,
            src: None,
            transformation: Transformation::new(),
            transformation_position: TransformationPosition::default(),
            query_parameters: None,
        }
    }
}

pub trait Url {
    /// Generates image transformation urls with the provided `Options`
    fn url(&self, opts: Options) -> Result<String>;
}

impl Url for ImageKit {
    fn url(&self, opts: Options) -> Result<String> {
        if opts.path.is_some() && opts.src.is_some() {
            return Err(anyhow::anyhow!("Either path or src is required"));
        }

        let transformed = opts.transformation.transform()?;
        let transformation_position = if opts.src.is_some() {
            //  If src parameter is being used, then always force the addition of transformation paramters in query
            &TransformationPosition::Query
        } else {
            &opts.transformation_position
        };

        let query_parameters_str = Utils::concat_query_parameters(&opts);
        let url_endpoint = opts
            .url_endpoint
            .clone()
            .unwrap_or(self.url_endpoint.clone());

        let generated_url = match transformation_position {
            TransformationPosition::Path => {
                let Some(path) = &opts.path else {
                    return Err(anyhow::anyhow!("path should be set for transformation position path"));
                };
                let path = path.trim_matches('/');
                let mut generated_url = format!("{url_endpoint}/tr:{transformed}/{path}");
                if !query_parameters_str.is_empty() {
                    generated_url.push_str(&format!("?{query_parameters_str}"));
                }
                generated_url
            }
            TransformationPosition::Query => {
                let mut generated_url = if let Some(src) = opts.src {
                    format!("{src}?tr={transformed}")
                } else {
                    let path = opts.path.expect("path is expected when src is not given");
                    format!("{url_endpoint}/{path}?tr={transformed}")
                };
                if !query_parameters_str.is_empty() {
                    generated_url.push_str(&format!("&{query_parameters_str}"));
                }
                generated_url
            }
        };
        Ok(generated_url)
    }
}

pub struct Utils;

impl Utils {
    fn concat_query_parameters(opts: &Options) -> String {
        let Some(query_parameters) = opts.query_parameters.as_ref() else {
        return String::default();
    };

        let mut query = String::default();

        for (param, value) in query_parameters {
            let parameter_str = format!("{param}={value}");
            query.push_str(&format!("&{parameter_str}"));
        }

        query = query.trim_matches('&').into();

        query
    }
}
