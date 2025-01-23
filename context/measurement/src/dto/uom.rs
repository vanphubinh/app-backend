use crate::entity::uom;
use infra::uuid::Uuid;
use sea_orm::{DerivePartialModel, FromQueryResult, ModelTrait};
use serde::Serialize;

#[derive(Debug, DerivePartialModel, Serialize, FromQueryResult)]
#[sea_orm(entity = "<uom::Model as ModelTrait>::Entity")]
#[serde(rename_all = "camelCase")]
pub struct Uom {
  pub id: Uuid,
  pub name: String,
}
