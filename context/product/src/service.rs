use infra::meta::PaginationMeta;
use infra::uuid::Uuid;
use measurement::entity::uom;
use sea_orm::prelude::Expr;
use sea_orm::sea_query::extension::postgres::PgExpr;
use sea_orm::sea_query::{Alias, Query};
use sea_orm::{
  ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DbConn, DbErr, EntityTrait,
  FromQueryResult, PaginatorTrait, QueryFilter, Set, TransactionError, TransactionTrait,
};

use crate::dto::{
  attribute_option::AttributeOption as AttributeOptionDto,
  category::Category as CategoryDto,
  product::{Product as ProductDto, ProductQueryResult},
};
use crate::entity::{
  attribute_option, attribute_option_value, category, product, product_template,
};
use crate::validator::{
  CreateAttributeOptionPayload, CreateCategoryPayload, CreateProductPayload,
  ListPaginatedAttributeOptionsParams, ListPaginatedCategoriesParams, ListPaginatedProductsParams,
};
use measurement::dto::uom::Uom as UomDto;

pub struct ProductService;

impl ProductService {
  pub async fn list_paginated_categories(
    db: &DbConn,
    params: ListPaginatedCategoriesParams,
  ) -> Result<(Vec<CategoryDto>, PaginationMeta), DbErr> {
    let per_page = params.per_page.unwrap_or(30);
    let page = params.page.unwrap_or(1) - 1;
    let search = params.search.unwrap_or_default();

    let category_pages = category::Entity::find()
      .filter(category::Column::Name.contains(search))
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
                value: Set(option.value.to_string()),
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

  pub async fn list_paginated_products(
    db: &DbConn,
    params: ListPaginatedProductsParams,
  ) -> Result<(Vec<ProductDto>, PaginationMeta), DbErr> {
    let per_page = params.per_page.unwrap_or(30);
    let page = params.page.unwrap_or(1) - 1;
    let search = params.search.unwrap_or_default();

    let product_query = Query::select()
      .column((product::Entity, product::Column::Id))
      .column((product::Entity, product::Column::ProductTemplateId))
      .column((product_template::Entity, product_template::Column::Name))
      .column((product_template::Entity, product_template::Column::UomId))
      .expr_as(
        Expr::col((uom::Entity, uom::Column::Name)),
        Alias::new("uom_name"),
      )
      .column((
        product_template::Entity,
        product_template::Column::CategoryId,
      ))
      .expr_as(
        Expr::col((category::Entity, category::Column::Name)),
        Alias::new("category_name"),
      )
      .column((
        product_template::Entity,
        product_template::Column::IsTrackInventory,
      ))
      .expr_as(
        Expr::col((
          product_template::Entity,
          product_template::Column::ProductType,
        ))
        .cast_as(Alias::new("TEXT")),
        Alias::new("product_type"),
      )
      .expr_as(
        Expr::col((
          product_template::Entity,
          product_template::Column::ProductSubtype,
        ))
        .cast_as(Alias::new("TEXT")),
        Alias::new("product_subtype"),
      )
      .from(product::Entity)
      .left_join(
        product_template::Entity,
        Expr::col((product::Entity, product::Column::ProductTemplateId))
          .equals((product_template::Entity, product_template::Column::Id)),
      )
      .left_join(
        category::Entity,
        Expr::col((
          product_template::Entity,
          product_template::Column::CategoryId,
        ))
        .equals((category::Entity, category::Column::Id)),
      )
      .left_join(
        uom::Entity,
        Expr::col((product_template::Entity, product_template::Column::UomId))
          .equals((uom::Entity, uom::Column::Id)),
      )
      .offset(page * per_page)
      .limit(per_page)
      .to_owned();
    let builder = db.get_database_backend();
    let product_result = ProductQueryResult::find_by_statement(builder.build(&product_query))
      .all(db)
      .await?;

    let mut product_map: std::collections::HashMap<Uuid, ProductDto> =
      std::collections::HashMap::new();
    for product in product_result {
      product_map.entry(product.id).or_insert_with(|| ProductDto {
        id: product.id,
        name: product.name.clone(),
        product_template_id: product.product_template_id,
        category: product.category_id.map(|id| CategoryDto {
          id,
          name: product.category_name.clone().unwrap(),
        }),
        uom: UomDto {
          id: product.uom_id,
          name: product.uom_name.clone(),
        },
        is_track_inventory: product.is_track_inventory,
        product_type: product.product_type,
        product_subtype: product.product_subtype,
      });
    }

    let products: Vec<ProductDto> = product_map.into_values().collect();
    let total = product::Entity::find().count(db).await?;
    let total_pages = (total as f64 / per_page as f64).ceil() as u64;

    Ok((
      products,
      PaginationMeta {
        total,
        total_pages,
        page: page + 1,
        per_page,
      },
    ))
  }

  pub async fn create_product(
    db: &DbConn,
    payload: CreateProductPayload,
  ) -> Result<product::Model, TransactionError<DbErr>> {
    let product = db
      .transaction::<_, product::Model, DbErr>(move |trx| {
        Box::pin(async move {
          let product_template = product_template::ActiveModel {
            name: Set(payload.name),
            uom_id: Set(payload.uom_id),
            category_id: Set(payload.category_id),
            is_track_inventory: Set(payload.is_track_inventory),
            product_type: Set(payload.product_type),
            product_subtype: Set(payload.product_subtype),
            ..Default::default()
          };
          let product_template = product_template.insert(trx).await?;

          let product = product::ActiveModel {
            product_template_id: Set(product_template.id),
            price: Set(payload.price),
            ..Default::default()
          };
          let product = product.insert(trx).await?;
          Ok(product)
        })
      })
      .await?;
    Ok(product)
  }
}
