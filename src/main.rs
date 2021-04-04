use acidalia::ui::{imgui, ImguiElement};
use acidalia::{screen, EngineBuilder};
use image::DynamicImage;
use imgui::im_str;

mod drawing_canvas;

#[derive(Default)]
pub struct Data {
    img: Option<DynamicImage>,
}

fn main() {
    println!("{:?}", std::env::current_dir());
    let engine = EngineBuilder::new(|wb| wb.with_maximized(true))
        .bg_color(acidalia::wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        })
        .build();
    let ui_el = ImguiElement::new(
        |ui, e, data: &mut Data| {
            imgui::Window::new(im_str!("Main")).build(ui, || {
                if ui.small_button(im_str!("Load image")) {
                    if let Ok(nfd2::Response::Okay(fp)) = nfd2::open_file_dialog(None, None) {
                        match image::open(fp) {
                            Ok(im) => data.img = Some(im),
                            Err(e) => eprintln!("Error opening image: {:?}", e),
                        }
                    }
                }
            });
        },
        &engine,
    );

    let data = Data::default();
    engine.run(screen!(ui_el), data);
}
