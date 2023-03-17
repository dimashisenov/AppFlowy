#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_variables)]
use crate::services::persistence::rev_sqlite::SQLiteDatabaseRevisionPersistence;
use bytes::Bytes;
use database_model::DatabaseRevision;
use flowy_client_sync::client_database::{
  make_database_rev_json_str, DatabaseOperationsBuilder, DatabaseRevisionPad,
};
use flowy_error::FlowyResult;
use flowy_revision::reset::{RevisionResettable, RevisionStructReset};
use flowy_sqlite::kv::KV;
use flowy_sqlite::ConnectionPool;
use lib_infra::util::md5;
use revision_model::Revision;
use std::sync::Arc;

const V1_MIGRATION: &str = "DATABASE_V1_MIGRATION";

pub async fn migration_database_rev_struct(
  user_id: &str,
  database_ids: &Vec<String>,
  pool: Arc<ConnectionPool>,
) -> FlowyResult<()> {
  let key = migration_flag_key(&user_id, V1_MIGRATION);
  if KV::get_bool(&key) {
    return Ok(());
  }
  tracing::trace!("Migrate database");
  for database_id in database_ids {
    let object = DatabaseRevisionResettable {
      database_id: database_id.to_owned(),
    };
    let disk_cache = SQLiteDatabaseRevisionPersistence::new(&user_id, pool.clone());
    let reset = RevisionStructReset::new(&user_id, object, Arc::new(disk_cache));
    reset.run().await?;
  }
  KV::set_bool(&key, true);
  Ok(())
}

fn migration_flag_key(user_id: &str, version: &str) -> String {
  md5(format!("{}{}", user_id, version,))
}

struct DatabaseRevisionResettable {
  database_id: String,
}

impl RevisionResettable for DatabaseRevisionResettable {
  fn target_id(&self) -> &str {
    &self.database_id
  }

  fn reset_data(&self, revisions: Vec<Revision>) -> FlowyResult<Bytes> {
    let pad = DatabaseRevisionPad::from_revisions(revisions)?;
    let json = pad.json_str()?;
    let bytes = DatabaseOperationsBuilder::new()
      .insert(&json)
      .build()
      .json_bytes();
    Ok(bytes)
  }

  fn default_target_rev_str(&self) -> FlowyResult<String> {
    let grid_rev = DatabaseRevision::default();
    let json = make_database_rev_json_str(&grid_rev)?;
    Ok(json)
  }

  fn read_record(&self) -> Option<String> {
    KV::get_str(self.target_id())
  }

  fn set_record(&self, record: String) {
    KV::set_str(self.target_id(), record);
  }
}
