use vizia::prelude::*;

#[derive(Lens)]
struct AppData {
    value: String,
}
impl Model for AppData {}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData {
            value: String::new(),
        }
        .build(cx);

        cx.add_stylesheet(include_style!("src/style.css"))
            .expect("Failed to load stylesheet");

        VStack::new(cx, |cx| {
            Label::new(cx, "Vizia TODO")
                .class("header")
                .font_size(25)
                .padding_bottom(Pixels(20.0))
                .alignment(Alignment::TopCenter);

            HStack::new(cx, |cx| {
                Label::new(cx, "Add a new item:").font_size(15);

                Textbox::new(cx, AppData::value)
                    .width(Pixels(100.0))
                    .color(Color::black());
                Button::new(cx, |cx| Label::new(cx, "Add")).background_color(Color::green());
            })
            .gap(Pixels(10.0));

            Button::new(cx, |cx| Label::new(cx, "Add")).background_color(Color::green());
        })
        .class("main-content");
    })
    .title("Counter")
    .inner_size((800, 600))
    .run()
}
