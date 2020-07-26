mod models;
mod finders;
mod utils;
mod adapters;
mod factories;
mod executors;
mod app;
mod core;
mod errors;
  
use self::app::App; 

use self::errors::{RusterError};

fn main() {
    use rusty_v8 as v8;

    let platform = v8::new_default_platform().unwrap();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);

    let code = v8::String::new(scope, "'Hello' + ' World!'").unwrap();
    println!("javascript code: {}", code.to_rust_string_lossy(scope));

    let script = v8::Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).unwrap();
    let result = result.to_string(scope).unwrap();
    println!("result: {}", result.to_rust_string_lossy(scope));

    let spec_finder = finders::FileSpecFinder{
        path: String::from("."),
    };

    let executor_factory = factories::DefaultExecutorFactory{};

    let app_factory = factories::DefaultAppFactory{
        spec_finder: &spec_finder,
        executor_factory: &executor_factory
    };
    match app_factory.create(){
        Ok(app) => {
            match app.execute(){
                Ok(result) => {
                    println!("Success: {}", result.success);
                },
                Err(e) => {
                    match e {
                        RusterError::Of(error_type) => {
                            println!("{}", error_type);
                        }
                    }
                }
            }; 
        },
        Err(e) => {
            println!("{}", e);
        }
    };
}
