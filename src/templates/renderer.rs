use once_cell::sync::Lazy;
use serde_json::Value;
use tera::{Context, Tera};

static TEMPLATES: Lazy<Tera> = Lazy::new(|| match Tera::new("templates/**/*.html") {
    Ok(t) => t,
    Err(e) => {
        println!("Parsing error(s): {}", e);
        ::std::process::exit(1);
    }
});

pub fn render_template(template_name: &str, context: &Value) -> String {
    let mut tera_context = Context::new();

    if let Value::Object(map) = context {
        for (key, value) in map {
            tera_context.insert(key, value);
        }
    }

    TEMPLATES
        .render(template_name, &tera_context)
        .unwrap_or_else(|err| format!("Template error: {}", err))
}

