mod models;
mod finders;
mod utils;
mod adapters;
mod factories;
mod executors;
mod app;
mod core;


fn main() -> core::Result<()> {
    let spec_finder = finders::FileSpecFinder{
        path: String::from("."),
    };

    let executor_factory = factories::DefaultExecutorFactory{};

    println!("creating app");
    let app = app::App{
        spec_finder: &spec_finder,
        executor_factory: &executor_factory
    };

    let app_factory = factories::DefaultAppFactory{
        spec_finder: &spec_finder,
        executor_factory: &executor_factory
    };
    let app = app_factory.create()
    println!("executing app");
    app.execute()?; 
    return Ok(());
}
