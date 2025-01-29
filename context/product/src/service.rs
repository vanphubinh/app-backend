use infra::meta::PaginationMeta;
use sea_orm::TransactionError;
use sea_orm::{
  ActiveModelTrait, DbConn, DbErr, EntityTrait, PaginatorTrait, Set, TransactionTrait,
};

use crate::dto::attribute_option::AttributeOption as AttributeOptionDto;
use crate::dto::category::Category as CategoryDto;
use crate::entity::attribute_option;
use crate::entity::attribute_option_value;
use crate::entity::category;
use crate::validator::CreateAttributeOptionPayload;
use crate::validator::ListPaginatedAttributeOptionsParams;
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

  pub async fn list_attribute_options(
    db: &DbConn,
    params: ListPaginatedAttributeOptionsParams,
  ) -> Result<(Vec<AttributeOptionDto>, PaginationMeta), DbErr> {
    let per_page = params.per_page.unwrap_or(30);
    let page = params.page.unwrap_or(1) - 1;

    let attribute_option_pages = attribute_option::Entity::find()
      .into_partial_model::<AttributeOptionDto>()
      .paginate(db, per_page);
    let attribute_options = attribute_option_pages.fetch_page(page).await?;
    let items_and_pages = attribute_option_pages.num_items_and_pages().await?;
    let total = items_and_pages.number_of_items;
    let total_pages = items_and_pages.number_of_pages;

    Ok((
      attribute_options,
      PaginationMeta {
        total,
        total_pages,
        page: page + 1,
        per_page,
      },
    ))
  }

  pub async fn create_attribute_option(
    db: &DbConn,
    payload: CreateAttributeOptionPayload,
  ) -> Result<attribute_option::Model, TransactionError<DbErr>> {
    let attribute = db
      .transaction::<_, attribute_option::Model, DbErr>(move |txn| {
        Box::pin(async move {
          let attribute = attribute_option::ActiveModel {
            name: Set(payload.name),
            ..Default::default()
          };
          let attribute = attribute.insert(txn).await?;
          if payload.option_values.len() > 0 {
            let options = payload
              .option_values
              .into_iter()
              .map(|option| attribute_option_value::ActiveModel {
                name: Set(option.name.to_string()),
                attribute_option_id: Set(attribute.id),
                ..Default::default()
              })
              .collect::<Vec<_>>();

            attribute_option_value::Entity::insert_many(options)
              .exec(txn)
              .await?;
          }

          Ok(attribute)
        })
      })
      .await?;
    Ok(attribute)
  }
}
