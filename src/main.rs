use iced::Theme;
use iced::widget::{row, button, column, text, Row}; // Импортируем row и Row вместо Column

pub fn main() -> iced::Result {
    iced::application(u64::default, update, view)
        .theme(Theme::Dark)
        .title(|value: &u64| value.to_string())
        .centered()
        .run()
}

#[derive(Debug, Clone)]
enum Events {
    Increment,
    Decrement,
    Reset,
}

fn update(value: &mut u64, message: Events) {
    match message {
        Events::Increment => *value += 1,
        Events::Decrement => {
            if *value > 0 {
                *value -= 1;
            }
        }
        Events::Reset => *value = 0,
    }
}

fn view(value: &u64) -> Row<'_, Events> { // Теперь возвращаем Row вместо Column
    // Кнопка "-" становится неактивной (серой), если её on_press получает None
    let on_decrement = if *value > 0 {
        Some(Events::Decrement)
    } else {
        None
    };

    row![
        button("+").on_press(Events::Increment),
        text(value),
        button("-").on_press_maybe(on_decrement), // Используем on_press_maybe
        button("Reset").on_press(Events::Reset),
    ]
    .spacing(20)
    .padding(20)
    .into()
}
