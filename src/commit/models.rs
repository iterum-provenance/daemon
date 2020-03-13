use crate::api_error::ApiError;
use crate::db;
use crate::schema::commit;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ChangeType {
    Add,
    Remove,
    Update,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Diff {
    pub change_type: ChangeType,
    pub files: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable)]
#[table_name = "commit"]
pub struct Commit {
    pub hash: String,
    pub name: Option<String>,
    pub parent: Option<String>,
    pub branch: String,
    pub description: Option<String>,
    pub deprecated: Option<String>,
}

impl Commit {
    pub fn retrieve_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        use crate::schema::commit as asdf;
        // commit.filter();

        let commits = asdf::table.load::<Commit>(&conn)?;

        // .load::<Commit>(&conn)?

        // let commits = commit::table.load::<Commit>(&conn)?;

        Ok(commits)
    }

    pub fn create(commit: Commit) -> Result<Self, ApiError> {
        use crate::schema::commit as asdf;

        let conn = db::connection()?;

        let result = diesel::insert_into(asdf::table)
            .values(&commit)
            .execute(&conn)?;

        // .get_result(&conn)?
        // .get_result(&conn)?;

        Ok(commit)
    }
}
