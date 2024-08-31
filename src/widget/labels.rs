use iced_futures::core::Element;


pub enum LabelListMessage {
    Focus(usize),
    Unfocus(usize),
}

pub struct LabelList {
    labels: &'static str,
}

impl LabelList {
    pub fn update(&mut self, message: LabelListMessage) {
        todo!()
    }

    pub fn view(&self) -> Element<LabelListMessage> {

    }
} 

pub enum