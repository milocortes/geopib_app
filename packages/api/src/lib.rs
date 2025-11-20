//! This crate contains all shared fullstack server functions.
use chrono::NaiveDateTime;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Serie {
    time: NaiveDateTime,
    value: f64,
}

#[cfg(feature = "server")]
thread_local! {
    static DB: std::sync::LazyLock<rusqlite::Connection> = std::sync::LazyLock::new(|| {
        let conn = rusqlite::Connection::open("geopib.db").expect("Failed to open database");
        /*
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS dogs (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL
            );",
        )
        .unwrap();
        */
        conn
    });
}

#[get("/api/precip")]
pub async fn list_precip() -> Result<Vec<f64>> {
    DB.with(|db| {
        Ok(db
            .prepare("SELECT * FROM co2")?
            .query_map([], |row| Ok(row.get(1)?))?
            .collect::<Result<Vec<f64>, rusqlite::Error>>()?)
    })
}

#[get("/api/serie")]
pub async fn list_serie() -> Result<Vec<(NaiveDateTime, f64)>> {
    DB.with(|db| {
        Ok(db
            .prepare("SELECT * FROM co2 ORDER BY time")?
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
            .collect::<Result<Vec<(NaiveDateTime, f64)>, rusqlite::Error>>()?)
    })
}
/*
#[get("/api/dogs")]
pub async fn list_dogs() -> Result<Vec<(usize, String)>> {
    DB.with(|db| {
        Ok(db
            .prepare("SELECT id, url FROM dogs ORDER BY id DESC LIMIT 10")?
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
            .collect::<Result<Vec<(usize, String)>, rusqlite::Error>>()?)
    })
}

#[delete("/api/dogs/{id}")]
pub async fn remove_dog(id: usize) -> Result<()> {
    DB.with(|db| db.execute("DELETE FROM dogs WHERE id = ?1", [id]))?;
    Ok(())
}

#[post("/api/dogs")]
pub async fn save_dog(image: String) -> Result<()> {
    DB.with(|db| db.execute("INSERT INTO dogs (url) VALUES (?1)", [&image]))?;
    Ok(())
}
*/
