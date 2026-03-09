use eframe::egui;
use eframe::egui::{Label, Layout, Response, Sense, Ui, Vec2};
use egui::Image;


pub enum ElementStyle {
    Icons,
    List,
}


pub struct Element <'a> {
    pub img: Image <'a>,
    pub image_size: [f32; 2],
    pub text: String,
    pub style: ElementStyle,
    icon_size: Vec2
}

impl Element <'_> {
    pub fn new<'a>(img: Image<'a>, image_size: [f32; 2], rtext: &'a String, style: ElementStyle) -> Element <'a> {
        Element {
            img,
            image_size,
            text: rtext.clone(),
            style,
            icon_size: Vec2::new(120.0, 120.0)
        }
    }
}

impl egui::Widget for Element<'_> {
    fn ui(self, ui: &mut Ui) -> Response {

        match self.style {
            ElementStyle::Icons => {

                let (rect, response) = ui.allocate_exact_size(self.icon_size, Sense::click());

                ui.scope_builder(
                    egui::UiBuilder::new()
                        .max_rect(rect)
                        .layout(Layout::top_down(egui::Align::Center)),
                    |ui| {
                        ui.add(self.img.fit_to_exact_size(self.image_size.into()));
                        ui.add(Label::new(self.text).wrap_mode(egui::TextWrapMode::Truncate));
                    },
                );

                response
            }
            ElementStyle::List => {
                let btn: egui::Button = egui::Button::image_and_text(self.img, self.text);
                ui.add(btn)
            }
        }

    }
}