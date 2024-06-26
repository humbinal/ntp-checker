use std::io;
use winresource;
use winresource::WindowsResource;

fn main() -> io::Result<()> {
    // slint_build::compile("ui/app.slint").unwrap();
    let config = slint_build::CompilerConfiguration::new()
        .with_style("fluent-light".into());
    slint_build::compile_with_config("ui/app.slint", config).unwrap();
    #[cfg(windows)]{
        WindowsResource::new()
            .set_icon("resources/logo.ico")
            .compile()?;
    }
    Ok(())
}
