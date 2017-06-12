pub type UserID = usize;
pub type ItemID = usize;

pub trait WeightedInteraction {
    fn get_user_id(&self) -> UserID;
    fn get_item_id(&self) -> ItemID;
    fn get_weight(&self) -> f32;
}
