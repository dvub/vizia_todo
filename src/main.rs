use vizia::prelude::*;

#[derive(Debug, Clone)]
struct Todo {
    title: String,
}
impl Data for Todo {
    // TODO: this needs to be improved at some point
    fn same(&self, other: &Self) -> bool {
        self.title == other.title
    }
}

#[derive(Lens)]
struct AppData {
    todos: Vec<Todo>,
    current_data: String,
}
impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            AppEvent::AddTodo => {
                self.todos.push(Todo {
                    title: self.current_data.clone(),
                });
                println!("{:?}", self.todos)
            }
            AppEvent::SetInputValue(value) => self.current_data = value.to_string(),
        });
    }
}

pub enum AppEvent {
    AddTodo,
    SetInputValue(String),
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData {
            todos: Vec::new(),
            current_data: "Hello, world".to_owned(),
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

                Textbox::new(cx, AppData::current_data)
                    .width(Pixels(100.0))
                    .color(Color::black())
                    .on_submit(|cx, value, _| cx.emit(AppEvent::SetInputValue(value)));
                Button::new(cx, |cx| Label::new(cx, "Add"))
                    .background_color(Color::green())
                    .on_press(|cx| cx.emit(AppEvent::AddTodo));
            })
            .gap(Pixels(10.0));

            Binding::new(cx, AppData::todos, |cx, s| {
                s.get(cx).iter().for_each(|x| {
                    // println!("{}", x.title.clone());
                    Label::new(cx, x.title.clone());
                });
            });
        })
        .class("main-content");
    })
    .title("Counter")
    .inner_size((800, 600))
    .run()
}
