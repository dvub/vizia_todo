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
                .padding(Percentage(10.0));
            Textbox::new(cx, AppData::value);
        })
        .class("main-content")
        .alignment(Alignment::TopCenter);
    })
    .title("Counter")
    .inner_size((800, 600))
    .run()
}
