use crate::dataset::models::DatasetConfig;

pub fn find_dataset_conf_for_pipeline_hash(db: &sled::Db, pipeline_hash: &str) -> Option<DatasetConfig> {
    db.iter()
        .map(|elem| elem.unwrap())
        .map(|(_key, value)| {
            // let dataset_name: String = key.into();
            let dataset_conf: DatasetConfig = value.into();
            dataset_conf
        })
        .find(|conf| {
            conf.backend
                .get_pipeline_executions(&conf.name)
                .unwrap()
                .contains(&pipeline_hash.to_owned())
        })
}

pub fn find_all_pipelines(db: &sled::Db) -> Vec<String> {
    db.iter()
        .map(|elem| elem.unwrap())
        .map(|(_key, value)| {
            // let dataset_name: String = key.into();
            let dataset_conf: DatasetConfig = value.into();
            dataset_conf
        })
        .fold(Vec::new(), |mut acc, conf| {
            let mut pipeline_hashes = conf.backend.get_pipeline_executions(&conf.name).unwrap();
            acc.append(&mut pipeline_hashes);
            acc
        })
}
