#[macro_use]
extern crate rocket;
mod database;

use database::requests::AmdProcessorTypeRequest;
use database::responses::AmdProcessorTypeResponse;
use database::{
    get_amd_processor_types, get_amd_processor_types_by_id, upsert_amd_processor_types, DBResult,
};
use rocket::serde::json::Json;
use rocket::State;
use sqlx::{Pool, Sqlite, SqlitePool};

#[post(
    "/processors/amd/types/add",
    format = "json",
    data = "<amd_processor_type>"
)]
async fn add_amd_processor_type(
    amd_processor_type: Json<AmdProcessorTypeRequest>,
    pool: &State<Pool<Sqlite>>,
) -> DBResult<Json<AmdProcessorTypeResponse>> {
    let id: i64 = upsert_amd_processor_types(
        pool,
        &amd_processor_type.architecture,
        &amd_processor_type.fabrication_nm,
        &amd_processor_type.family,
        &amd_processor_type.release_date,
        &amd_processor_type.codename,
        &amd_processor_type.model_group,
        &amd_processor_type.cores,
        &amd_processor_type.smt,
        &amd_processor_type.clock_rate_mhz,
        &amd_processor_type.bus_speed_type,
        &amd_processor_type.cache_l1,
        &amd_processor_type.cache_l2,
        &amd_processor_type.cache_l3,
        &amd_processor_type.socket,
        &amd_processor_type.memory_controller,
        &amd_processor_type.simd,
        &amd_processor_type.speed_power,
        &amd_processor_type.other,
        &amd_processor_type.changes,
    )
    .await?;
    let processor_type = get_amd_processor_types_by_id(pool, id).await?;
    Ok(Json(processor_type))
}

#[get("/processors/amd/types/list")]
async fn list_amd_processor_type(
    pool: &State<Pool<Sqlite>>,
) -> DBResult<Json<Vec<AmdProcessorTypeResponse>>> {
    let type_list: Vec<AmdProcessorTypeResponse> = get_amd_processor_types(pool).await?;
    Ok(Json(type_list))
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let pool = SqlitePool::connect("sqlite://data.db")
        .await
        .expect("Couldn't connect to sqlite database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Couldn't migrate the database tables");

    let _rocket = rocket::build()
        .mount(
            "/",
            routes![add_amd_processor_type, list_amd_processor_type],
        )
        .manage(pool)
        .launch()
        .await?;
    Ok(())
}
