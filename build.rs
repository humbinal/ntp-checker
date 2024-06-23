use std::io;
use winresource;
use winresource::WindowsResource;

fn main() -> io::Result<()> {
    slint_build::compile("ui/app.slint").unwrap();
    #[cfg(windows)]{
        WindowsResource::new()
            .set_icon("resources/logo.ico")
            .compile()?;
    }
    Ok(())
}
