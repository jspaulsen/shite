use uuid::Uuid;

pub trait Trackable {
    fn get_handle(&self) -> &Uuid;
}
