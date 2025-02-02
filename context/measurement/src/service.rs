use infra::meta::PaginationMeta;
use sea_orm::{
  ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, PaginatorTrait, QueryFilter, Set,
};

use crate::{
  dto::uom::Uom as UomDto,
  entity::uom,
  entity::uom::Model as Uom,
  validator::{CreateUomPayload, ListPaginatedUomsParams},
};

pub struct MeasurementService;

impl MeasurementService {
  pub async fn list_paginated_uoms(
    db: &DbConn,
    params: ListPaginatedUomsParams,
  ) -> Result<(Vec<UomDto>, PaginationMeta), DbErr> {
    let per_page = params.per_page.unwrap_or(30);
    let page = params.page.unwrap_or(1) - 1;
    let search = params.search.unwrap_or_default();

    let uom_pages = uom::Entity::find()
      .filter(uom::Column::Name.contains(search))
      .into_partial_model::<UomDto>()
      .paginate(db, per_page);
    let uoms = uom_pages.fetch_page(page).await?;
    let items_and_pages = uom_pages.num_items_and_pages().await?;
    let total = items_and_pages.number_of_items;
    let total_pages = items_and_pages.number_of_pages;

    Ok((
      uoms,
      PaginationMeta {
        page: page + 1,
        per_page,
        total,
        total_pages,
      },
    ))
  }

  pub async fn create_uom(db: &DbConn, payload: CreateUomPayload) -> Result<Uom, DbErr> {
    let uom = uom::ActiveModel {
      name: Set(payload.name),
      ..Default::default()
    };
    let uom = uom.insert(db).await?;
    Ok(uom)
  }
}
