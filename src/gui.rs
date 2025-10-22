use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, Image};
use walkdir::WalkDir;

pub fn gui() -> glib::ExitCode {
    let app = Application::builder().application_id("LinuxWEGUI").build();

    app.connect_activate(|app| {
        let image = find_wallpapers();
        build_ui(app, image);
    });

    app.run()
}

pub fn find_wallpapers() -> Image {
    let wallpaper_dir = "/home/user/Desktop/coding/LinuxWallpaperEngineGUI/431960.o";
    let image = Image::new();
    image.set_pixel_size(150);

    for entry in WalkDir::new(wallpaper_dir)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file()
            && path.file_name().is_some_and(|name| {
                name == "preview.jpg" || name == "preview.gif" || name == "preview.png"
            })
        {
            let path_str = path.to_str().unwrap();
            image.set_from_file(Some(path_str));
            println!("Found wallpaper preview: {}", path_str);
        }
    }

    image
}

fn build_ui(app: &Application, image: Image) {
    let button = Button::builder()
        .label("button")
        .margin_top(50)
        .margin_bottom(50)
        .margin_start(50)
        .margin_end(50)
        .build();

    button.connect_clicked(|_| {
        println!("Button clicked!");
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Linux Wallpaper Engine")
        .default_width(300)
        .default_height(100)
        .child(&image)
        .build();

    window.present();
}
