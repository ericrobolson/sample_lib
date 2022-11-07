pub trait Actor {
    type UpdateMsg;
    type Outcome;

    fn handle(&mut self, msg: Self::UpdateMsg) -> Self::Outcome;
}
