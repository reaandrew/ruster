mod models;
mod finders;
mod utils;
mod adapters;
mod factories;
mod executors;
mod app;

fn main() -> Result<(), std::io::Error> {
    let spec_finder = finders::FileSpecFinder{
        path: String::from("."),
    };

    let executor_factory = factories::DefaultExecutorFactory{};

    println!("creating app");
    let app = app::App{
        spec_finder: &spec_finder,
        executor_factory: &executor_factory
    };
    println!("executing app");
    app.execute()?; 
    return Ok(());
}
