use iced::{widget::text, Element};

#[derive(Debug, Clone)]
pub enum Message {
    NONE
}

pub fn view(_state: &usize) -> Element<Message> {
    text(String::from("ksflksmflkf")).into()
}

pub fn update(_state: &mut usize, _: Message) {}