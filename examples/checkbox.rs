extern crate azul;

use azul::prelude::*;
use azul::widgets::checkbox::Checkbox;
use azul::widgets::button::Button;

struct MyDataModel { }

impl Layout for MyDataModel {
    fn layout(&self, _: LayoutInfo<Self>) -> Dom<Self> {
        let checkbox = Checkbox::new(true).dom();
        let button = Button::with_label("123").dom();
        Dom::new(NodeType::Div)
            .with_child(checkbox)
            .with_child(button)
    }
}

fn main() {
    let app = App::new(MyDataModel { }, AppConfig::default());
    let window = Window::new(WindowCreateOptions::default(), css::native()).unwrap();
    app.run(window).unwrap();
}