//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use async_trait::async_trait;
use chrono::Utc;
use infra::uuid::Uuid;
use sea_orm::{entity::prelude::*, ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "attribute_option")]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  #[sea_orm(column_type = "Text")]
  pub name: String,
  pub created_at: ChronoDateTimeWithTimeZone,
  #[sea_orm(nullable)]
  pub updated_at: Option<ChronoDateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::attribute_option_value::Entity")]
  AttributeOptionValue,
}

impl Related<super::attribute_option_value::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::AttributeOptionValue.def()
  }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
  fn new() -> Self {
    Self {
      id: Set(Uuid::new()),
      ..ActiveModelTrait::default()
    }
  }

  async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
  where
    C: ConnectionTrait,
  {
    let _ = db;
    let mut this = self;
    if !insert {
      this.updated_at = Set(Some(Utc::now().into()));
    }
    Ok(this)
  }
}
