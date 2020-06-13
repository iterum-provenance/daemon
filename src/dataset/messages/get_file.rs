use super::super::actor::DatasetActor;
use crate::backend::storable::Storable;
use actix::prelude::Message;
use actix::prelude::{Context, Handler};

pub struct GetFileMessage {
    pub file_name: String,
    pub commit_hash: String,
}

impl Message for GetFileMessage {
    type Result = Option<Vec<u8>>;
}

impl Handler<GetFileMessage> for DatasetActor {
    type Result = Option<Vec<u8>>;

    fn handle(&mut self, msg: GetFileMessage, _ctx: &mut Context<Self>) -> Self::Result {
        info!("Retrieving file {}", msg.file_name);

        match self
            .ds
            .dataset
            .backend
            .get_file(&self.dataset_path, &msg.commit_hash, &msg.file_name)
        {
            Ok(file_content) => Some(file_content),
            Err(err) => {
                error!("Something went wrong retrieving the file: {}", err);
                None
            }
        }
    }
}
