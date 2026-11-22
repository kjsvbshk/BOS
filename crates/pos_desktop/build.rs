//! Incrusta el ícono y los datos de versión en el ejecutable de Windows.

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=assets/icon/app.ico");
    #[cfg(windows)]
    {
        let mut resource = winresource::WindowsResource::new();
        resource
            .set_icon("assets/icon/app.ico")
            .set("ProductName", "Compraventa 3 Diamantes")
            .set("FileDescription", "Sistema de Gestión de Empeños (demo)")
            .set("OriginalFilename", "Compraventa3Diamantes.exe");
        resource.compile()?;
    }
    Ok(())
}
