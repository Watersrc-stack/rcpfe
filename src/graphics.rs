use iced::border;
use iced::mouse;
use iced::widget::{canvas, center_y, column, container, row, scrollable, space, text, Button, Column};
use iced::{
    Center, Element, Fill, Length, Point, Rectangle, Renderer, Theme,
};
use crate::App;


#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
    ChangeDir(std::path::PathBuf)
}


pub struct GUI {
    app: App
}

impl Default for GUI {
    fn default() -> Self {
        let app = App::default();

        GUI {
            app
        }
    }
}

impl GUI {

    pub fn update(&mut self, _message: Message) {

    }

    pub fn view(&self) -> Element<'_, Message> {
        let header = container(
            row![
            GUI::square(40),
            space::horizontal(),
            "Header!",
            space::horizontal(),
            GUI::square(40),
        ]
                .padding(10)
                .align_y(Center),
        )
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();

                container::Style::default().border(border::color(palette.background.strong.color).width(1))
            });

        let sidebar = center_y(
            column!["Sidebar!", GUI::square(50), GUI::square(50)]
                .spacing(40)
                .padding(10)
                .width(200)
                .align_x(Center),
        )
            .style(container::rounded_box);

        let content = container(
            scrollable(


                self.app.entries.iter().fold(
                    Column::new(),
                    |column, ent| {
                        column.push(
                            Button::new(text(&ent.name))
                                .on_press(Message::ChangeDir(ent.item.path()))
                        )
                    }
                )



                    .spacing(40)
                    .align_x(Center)
                    .width(Fill),
            )
                .height(Fill),
        )
            .padding(10);

        column![header, row![sidebar, content]].into()
    }


    fn square(size: impl Into<Length> + Copy) -> Element<'static, Message> {
        struct Square;

        impl canvas::Program<Message> for Square {
            type State = ();

            fn draw(
                &self,
                _state: &Self::State,
                renderer: &Renderer,
                theme: &Theme,
                bounds: Rectangle,
                _cursor: mouse::Cursor,
            ) -> Vec<canvas::Geometry> {
                let mut frame = canvas::Frame::new(renderer, bounds.size());

                let palette = theme.extended_palette();

                frame.fill_rectangle(
                    Point::ORIGIN,
                    bounds.size(),
                    palette.background.strong.color,
                );

                vec![frame.into_geometry()]
            }
        }

        canvas(Square).width(size).height(size).into()
    }

}