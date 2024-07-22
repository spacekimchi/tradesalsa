use std::sync::Arc;
use crate::utils::{e500, ErrorResponse};
use crate::constants::{
    strings,
    html_templates,
};

pub struct RenderTemplateParams<'a> {
    pub template_path: &'static str,
    pub tera_store: &'a Arc<tera::Tera>,
    pub template_context: Option<&'a tera::Context>,
}

impl<'a> RenderTemplateParams<'a> {
    pub fn new(template_path: &'static str, tera_store: &'a Arc<tera::Tera>) -> Self {
        Self {
            template_path,
            tera_store,
            template_context: None,
        }
    }

    pub fn with_context(mut self, data: &'a tera::Context) -> Self {
        self.template_context = Some(data);
        self
    }
}

pub fn render_content(render_template_params: &RenderTemplateParams<'_>) -> Result<String, ErrorResponse> {
    // First set the context data
    let context: tera::Context;
    if let Some(data) = render_template_params.template_context {
        context = data.clone(); // assuming `tera::Context` implements the Clone trait
    } else {
        context = tera::Context::new();
    }

    Ok(render_template_params.tera_store.render(&render_template_params.template_path, &context).map_err(e500)?)
}

pub fn err_500_template<E: std::fmt::Display>(tr: &Arc<tera::Tera>, error: E) -> String {
    let error_description = format!("{}", error);
    let mut context = tera::Context::new();
    context.insert("error_description", &error_description);
    tr.render(html_templates::E500, &context).unwrap_or_else(|_| String::from(strings::INTERNAL_SERVER_ERROR))
}

pub fn currency_format(value: &tera::Value, _: &std::collections::HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    match value.as_f64() {
        Some(num) => {
            // Format the number as currency here. This is a simple example.
            let formatted = if num > 0.0 {
                format!("+${:.2}", num.abs())
            } else if 0.0 > num {
                format!("-${:.2}", num.abs())
            } else {
                "$0.00".to_string()
            };
            Ok(tera::Value::String(formatted))
        },
        None => Err("Failed to format value as currency".into()),
    }
}

pub fn round_hundreths(value: &tera::Value, _: &std::collections::HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    match value.as_f64() {
        Some(num) => {
            // Format the number as currency here. This is a simple example.
            let formatted = if num > 0.0 {
                format!("+{:.2}", num.abs())
            } else if 0.0 > num {
                format!("-{:.2}", num.abs())
            } else {
                "0.00".to_string()
            };
            Ok(tera::Value::String(formatted))
        },
        None => Err("Failed to format value as f64 in round_hundreths".into()),
    }
}
