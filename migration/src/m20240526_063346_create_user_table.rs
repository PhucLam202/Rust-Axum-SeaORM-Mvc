use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Username).string().null())
                    .col(ColumnDef::new(Users::Email).string().null())
                    .col(ColumnDef::new(Users::Phone).integer().null())
                    .col(ColumnDef::new(Users::CreatedAt).timestamp().null())
                    .col(ColumnDef::new(Users::UpdatedAt).timestamp().null())
                    .col(ColumnDef::new(Users::Password).string().null())
                    .col(ColumnDef::new(Users::SoflDelete).boolean().null())
                    .col(ColumnDef::new(Users::IsActive).boolean().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Email,
    Phone,
    CreatedAt,
    UpdatedAt,
    Password,
    SoflDelete,
    IsActive,
}
