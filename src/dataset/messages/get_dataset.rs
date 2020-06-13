use super::super::actor::DatasetActor;
use crate::dataset::Dataset;
use actix::prelude::Message;
use actix::prelude::{Context, Handler};

pub struct GetDatasetMessage {}

impl Message for GetDatasetMessage {
    type Result = Option<Dataset>;
}

impl Handler<GetDatasetMessage> for DatasetActor {
    type Result = Option<Dataset>;

    fn handle(&mut self, _msg: GetDatasetMessage, _ctx: &mut Context<Self>) -> Self::Result {
        info!("Retrieving dataset");
        Some(self.ds.dataset.clone())
    }
}
