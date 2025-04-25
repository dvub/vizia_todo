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
    edit_window_open: bool,
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
            AppEvent::UpdateItem(i) => {
                self.todos[*i].title = "ok".to_owned();
                println!("{:?}", self.todos)
            }

            AppEvent::OpenEditWindow => self.edit_window_open = true,
        });
    }
}

pub enum AppEvent {
    AddTodo,
    SetInputValue(String),
    UpdateItem(usize),
    OpenEditWindow,
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData {
            todos: Vec::new(),
            current_data: "Hello, world".to_owned(),
            edit_window_open: false,
        }
        .build(cx);

        cx.add_stylesheet(include_style!("src/style.css"))
            .expect("Failed to load stylesheet");

        VStack::new(cx, |cx| {
            Button::new(cx, |cx| Label::new(cx, "Create new Todo"))
                .on_press(|cx| cx.emit(AppEvent::OpenEditWindow));

            Binding::new(cx, AppData::edit_window_open, |cx, show_subwindow| {
                if show_subwindow.get(cx) {
                    Window::new(cx, |cx| {
                        Label::new(cx, "Hello, world!");
                    })
                    .title("Set color...")
                    .inner_size((400, 200));
                }
            });

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
                s.get(cx).iter().enumerate().for_each(|(i, x)| {
                    // println!("{}", x.title.clone());
                    Label::new(cx, x.title.clone());
                    Button::new(cx, |cx| {
                        Svg::new(cx, *include_bytes!("../resources/images/pencil.svg"))
                            .size(Pixels(25.0))

                        //.border_color(Color::red())
                        //.border_width(Pixels(1.0))
                        // background_color(Color::white())
                    })
                    .on_press(move |ex| ex.emit(AppEvent::UpdateItem(i)));
                });
            });
        })
        .class("main-content");
    })
    .title("Counter")
    .inner_size((800, 600))
    .run()
}
