use std::sync::Arc;

use minijinja::{path_loader, Environment};

use wshared::types::TResult;

use super::{generate_id, generate_random_color};


pub type TSharedTemplates = Arc<Environment<'static>>; // for readonly

pub fn init_templates() -> TResult<TSharedTemplates> {
    let mut env = Environment::new();

    env.set_loader(path_loader("templates"));
    env.add_function("generate_id", generate_id);
    env.add_function("random_color", generate_random_color);

    Ok(Arc::new(env))
}
