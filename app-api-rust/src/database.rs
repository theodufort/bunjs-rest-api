use sqlx::{Pool, Sqlite};
pub mod requests;
pub mod responses;
use std::env;

use responses::AmdProcessorTypeResponse;

pub type DBResult<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

pub async fn upsert_amd_processor_types(
    pool: &Pool<Sqlite>,
    architecture: &String,
    fabrication_nm: &String,
    family: &String,
    release_date: &String,
    codename: &String,
    model_group: &String,
    cores: &String,
    smt: &String,
    clock_rate_mhz: &String,
    bus_speed_type: &String,
    cache_l1: &String,
    cache_l2: &String,
    cache_l3: &String,
    socket: &String,
    memory_controller: &String,
    simd: &String,
    speed_power: &String,
    other: &String,
    changes: &String,
) -> DBResult<i64> {
    let mut connection = pool.acquire().await?;
    let id = sqlx::query_as!(
        AmdProcessorTypeResponse,
        r#"
        INSERT OR REPLACE INTO AmdProcessorsTypes (architecture,fabrication_nm,family,release_date,codename,model_group,cores,smt,clock_rate_mhz,bus_speed_type,cache_l1,cache_l2,cache_l3,socket,memory_controller,simd,speed_power,other,changes) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);
        "#,
        architecture,
        fabrication_nm,
        family,
        release_date,
        codename,
        model_group,
        cores,
        smt,
        clock_rate_mhz,
        bus_speed_type,
        cache_l1,
        cache_l2,
        cache_l3,
        socket,
        memory_controller,
        simd,
        speed_power,
        other,
        changes,
    )
    .execute(&mut connection)
    .await?
    .last_insert_rowid();
    Ok(id)
}

pub async fn get_amd_processor_types(
    pool: &Pool<Sqlite>,
) -> DBResult<Vec<AmdProcessorTypeResponse>> {
    let mut connection = pool.acquire().await.unwrap();
    let task =
        sqlx::query_as::<_, AmdProcessorTypeResponse>(r#"SELECT * from AmdProcessorsTypes;"#)
            .fetch_all(&mut connection)
            .await?;
    Ok(task)
}
pub async fn get_amd_processor_types_by_id(
    pool: &Pool<Sqlite>,
    id: i64,
) -> DBResult<AmdProcessorTypeResponse> {
    let mut connection = pool.acquire().await.unwrap();

    let task = sqlx::query_file!(
        env!("CARGO_MANIFEST_DIR")
        "/sql_queries/get_amd_processor_type_by_id.sql",
        id
    )
    .fetch_one(&mut connection)
    .await?;
    Ok(task)
}
