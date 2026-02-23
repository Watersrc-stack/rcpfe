use eframe::egui;
use eframe::egui::{Label, Response, Ui, Vec2};
use egui::Image;


pub enum ElementStyle {
    Icons,
    List,
}


pub struct Element <'a> {
    pub img: Image <'a>,
    pub image_size: [f32; 2],
    pub text: String,
    pub style: ElementStyle
}

impl Element <'_> {
    pub fn new<'a>(img: Image<'a>, image_size: [f32; 2], rtext: &'a String) -> Element <'a> {
        Element {
            img,
            image_size,
            text: rtext.clone(),
            style: ElementStyle::Icons
        }
    }

    pub fn set_style(&mut self, style: ElementStyle) {
        self.style = style;
    }


}
impl egui::Widget for Element<'_> {
    fn ui(self, ui: &mut Ui) -> Response {

        match self.style {
            ElementStyle::Icons => {
                ui.add_sized(self.image_size, self.img);
                ui.add(Label::new(self.text))
            }
            ElementStyle::List => {
                let btn: egui::Button = egui::Button::image_and_text(self.img, self.text);
                ui.add(btn)
            }
        }

    }
}