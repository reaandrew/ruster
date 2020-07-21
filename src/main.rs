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
