use {
    traits::Layout,
    dom::{Dom, NodeType},
};

#[derive(Copy, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Checkbox {
    pub content: bool
}

impl Checkbox {
    pub fn new(checked: bool)
                      -> Self
    {
        Self {
            content: checked
        }
    }

    pub fn dom<T>(self)
                  -> Dom<T> where T: Layout
    {
        let mut checkbox_text = Dom::new(NodeType::Label(String::from("checkbox")))
            .with_class("__azul-native-checkbox-text");
        let mut checkbox_box = Dom::new(NodeType::Div)
            .with_class("__azul-native-checkbox-box");
        if (self.content) {
            let mut checkbox_checkmark = Dom::new(NodeType::Div)
                .with_class("__azul-native-checkbox-checkmark");

            checkbox_box = checkbox_box.with_child(checkbox_checkmark);
        }
        let mut checkbox_root = Dom::new(NodeType::Div)
            .with_class("__azul-native-checkbox")
            .with_child(checkbox_box)
            .with_child(checkbox_text);
        checkbox_root
    }
}