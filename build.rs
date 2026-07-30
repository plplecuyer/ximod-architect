//! Build script for XIMOD Architect
//!
//! This script handles platform-specific build tasks:
//! - Windows: Embeds the application icon into the executable
//! - Linux/macOS: No special build steps required

fn main() {
    // Windows-specific: Embed icon and metadata into executable
    #[cfg(target_os = "windows")]
    {
        // Only run if the icon file exists
        let icon_path = "assets/icons/ximod-architect.ico";
        if std::path::Path::new(icon_path).exists() {
            let mut res = winres::WindowsResource::new();
            
            // Set the application icon
            res.set_icon(icon_path);
            
            // Set executable metadata
            res.set("ProductName", "XIMOD Architect");
            res.set("FileDescription", "FOMOD Installer Creation Tool");
            res.set("LegalCopyright", "Copyright © 2024 XIMOD Team");
            res.set("CompanyName", "XIMOD");
            
            // Compile the resources
            if let Err(e) = res.compile() {
                eprintln!("Warning: Failed to compile Windows resources: {}", e);
                eprintln!("The application will build but without the custom icon.");
            }
        } else {
            println!("cargo:warning=Icon file not found at {}. Building without custom icon.", icon_path);
        }
    }
    
    // Rerun build script if icon changes
    println!("cargo:rerun-if-changed=assets/icons/ximod-architect.ico");
    println!("cargo:rerun-if-changed=build.rs");
}
