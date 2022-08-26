use druid::widget::{Flex, Padding};
use druid_ui::widget::{Button,ButtonStyle};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WindowDesc};

const WINDOW_TITLE: LocalizedString<ButtonState> = LocalizedString::new("Hello World!");

#[derive(Clone, Data, Lens)]
struct ButtonState {

}

fn main() {

    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((700.0, 400.0));

    let initial_state = ButtonState {
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<ButtonState> {
    
    let button = Button::new("普通按钮",ButtonStyle::RED).on_click(|_,_,_|{
        println!("click");
    });
    let button2 = Button::new("普通按钮",ButtonStyle::ORANGE);
    let button3 = Button::new("普通按钮",ButtonStyle::YELLOW);
    let button4 = Button::new("普通按钮",ButtonStyle::PURPLE);
    let button5 = Button::new("普通按钮",ButtonStyle::BLUE);
    let button6 = Button::new("普通按钮",ButtonStyle::GREEN);

    let padd1 = Padding::new((10.,10.), button);
    let padd2 = Padding::new((10.,10.), button2);
    let padd3 = Padding::new((10.,10.), button3);
    let padd4 = Padding::new((10.,10.), button4);
    let padd5 = Padding::new((10.,10.), button5);
    let padd6 = Padding::new((10.,10.), button6);

    Flex::row()
        .with_child(padd1)
        .with_child(padd2)
        .with_child(padd3)
        .with_child(padd4)
        .with_child(padd5)
        .with_child(padd6)
}





