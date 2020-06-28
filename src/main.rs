mod models;
mod finders;
mod utils;
mod adapters;
mod app;

fn main() -> Result<(), std::io::Error> {
    let dir = std::env::current_exe()?;
    let spec_finder = finders::FileSpecFinder{
        path: dir.display().to_string(),
    };
    let app = app::App{
        spec_finder: &spec_finder,
    };
    app.execute()?; 
    return Ok(());
}
