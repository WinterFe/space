use self::{
    nekoslife::client::NekosLifeClient,
};

pub mod nekoslife;

lazy_static! {
    static ref NEKOSLIFE_API: NekosLifeClient = NekosLifeClient::default();
}

pub fn get_nekoslife_api() -> &'static NekosLifeClient {
    &NEKOSLIFE_API
}