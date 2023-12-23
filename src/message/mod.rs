use crate::model::Model;

pub enum Message {
    StateUpdate(Model),
    Terminate,
}
