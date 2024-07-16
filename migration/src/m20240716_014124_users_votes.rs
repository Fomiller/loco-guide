use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(UsersVotes::Table)
                    .primary_key(
                        Index::create()
                            .name("idx-users_votes-refs-pk")
                            .table(UsersVotes::Table)
                            .col(UsersVotes::UserId)
                            .col(UsersVotes::MovieId)
                            ,
                    )
                    .col(integer(UsersVotes::UserId))
                    .col(integer(UsersVotes::MovieId))
                    .col(integer_null(UsersVotes::Vote))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-users_votes-users")
                            .from(UsersVotes::Table, UsersVotes::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-users_votes-movies")
                            .from(UsersVotes::Table, UsersVotes::MovieId)
                            .to(Movies::Table, Movies::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UsersVotes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UsersVotes {
    Table,
    UserId,
    MovieId,
    Vote,
    
}


#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Movies {
    Table,
    Id,
}
