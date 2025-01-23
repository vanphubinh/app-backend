use infra::meta::PaginationMeta;
use sea_orm::{DbConn, DbErr, EntityTrait, PaginatorTrait};

use crate::{dto::uom::Uom, entity::uom, validator::ListPaginatedUomsParams};

pub struct MeasurementService;

impl MeasurementService {
  pub async fn list_paginated_uoms(
    db: &DbConn,
    params: ListPaginatedUomsParams,
  ) -> Result<(Vec<Uom>, PaginationMeta), DbErr> {
    let per_page = params.per_page.unwrap_or(30);
    let page = params.page.unwrap_or(1) - 1;

    let uom_pages = uom::Entity::find()
      .into_partial_model::<Uom>()
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
}
