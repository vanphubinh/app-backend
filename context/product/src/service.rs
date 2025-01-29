use infra::meta::PaginationMeta;
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait, PaginatorTrait, Set};

use crate::dto::category::Category as CategoryDto;
use crate::entity::category;
use crate::validator::{CreateCategoryPayload, ListPaginatedCategoriesParams};

pub struct ProductService;

impl ProductService {
  pub async fn list_paginated_categories(
    db: &DbConn,
    params: ListPaginatedCategoriesParams,
  ) -> Result<(Vec<CategoryDto>, PaginationMeta), DbErr> {
    let per_page = params.per_page.unwrap_or(30);
    let page = params.page.unwrap_or(1) - 1;

    let category_pages = category::Entity::find()
      .into_partial_model::<CategoryDto>()
      .paginate(db, per_page);
    let categories = category_pages.fetch_page(page).await?;
    let items_and_pages = category_pages.num_items_and_pages().await?;
    let total = items_and_pages.number_of_items;
    let total_pages = items_and_pages.number_of_pages;

    Ok((
      categories,
      PaginationMeta {
        total,
        total_pages,
        page: page + 1,
        per_page,
      },
    ))
  }

  pub async fn create_category(
    db: &DbConn,
    payload: CreateCategoryPayload,
  ) -> Result<category::Model, DbErr> {
    let category = category::ActiveModel {
      name: Set(payload.name),
      parent_category_id: Set(payload.parent_category_id),
      ..Default::default()
    };

    let category = category.insert(db).await?;

    Ok(category)
  }
}
