use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, SqlitePool};

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct VibeGroup {
    pub vibe_group_id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Vibe {
    pub vibe_id: i64,
    pub name: String,
    pub vibe_group_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct VibeDetail {
    pub vibe_name: String,
    pub group_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct TrackPointer {
    pub track_id: i64,
    pub path: String,
}

pub struct Mp3Database {
    pool: SqlitePool,
}

impl Mp3Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok( Self { pool } )
    }

    // CREATE TRACK
    pub async fn add_track(&self, path: &str) -> Result<i64, sqlx::Error> {
        let id = sqlx::query!(
            "
            INSERT INTO track_pointers (path)
            VALUES (?)
            RETURNING track_id
            ", path)
            .fetch_one(&self.pool)
            .await?
            .track_id;

        Ok(id)
    }

    // READ TRACK
    pub async fn get_track(&self, track_id: i64) -> Result<Option<TrackPointer>, sqlx::Error> {
        sqlx::query_as!(TrackPointer,
            "
            SELECT track_id, path
            FROM track_pointers
            WHERE track_id = ?
            ", track_id)
            .fetch_optional(&self.pool)
            .await
    }

    // READ TRACKS
    pub async fn get_all_tracks(&self) -> Result<Vec<TrackPointer>, sqlx::Error> {
        sqlx::query_as!(TrackPointer,
            "
            SELECT track_id, path
            FROM track_pointers
            ORDER BY track_id ASC
            ")
            .fetch_all(&self.pool)
            .await
    }

    // UPDATE TRACK
    pub async fn update_track_path(&self, track_id: i64, path: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            UPDATE track_pointers
            SET path = ?
            WHERE track_id = ?
            ", path, track_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // DELETE TARCK
    pub async fn remove_track(self, track_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            DELETE FROM track_pointers
            WHERE track_id = ?
            ", track_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // CREATE GROUP
    pub async fn add_vibe_group(&self, name: &str) -> Result<i64, sqlx::Error> {
        let id = sqlx::query!(
            "
            INSERT INTO vibe_groups (name)
            VALUES (?)
            RETURNING vibe_group_id
            ", name)
            .fetch_one(&self.pool)
            .await?
            .vibe_group_id;

        Ok(id)
    }

    // READ GROUP
    pub async fn get_vibe_group(&self, vibe_group_id: i64) -> Result<VibeGroup, sqlx::Error> {
        sqlx::query_as!(VibeGroup,
            "
            SELECT vibe_group_id, name
            FROM vibe_groups
            WHERE vibe_group_id = ?
            ", vibe_group_id)
            .fetch_one(&self.pool)
            .await
    }

    // READ GROUPS
    pub async fn get_all_vibe_groups(&self) -> Result<Vec<VibeGroup>, sqlx::Error> {
        sqlx::query_as!(VibeGroup,
            "
            SELECT vibe_group_id, name
            FROM vibe_groups
            ")
            .fetch_all(&self.pool)
            .await
    }

    // UPDATE GROUP
    pub async fn change_vibe_group_name(&self, vibe_group_id: i64, name: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            UPDATE vibe_groups
            SET name = ?
            WHERE vibe_group_id = ?
            ", name, vibe_group_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // DELETE GROUP
    pub async fn remove_vibe_group(&self, vibe_group_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            DELETE FROM vibe_groups
            WHERE vibe_group_id = ?
            ", vibe_group_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // CREATE VIBE
    pub async fn add_vibe(&self, name: &str, vibe_group_id: i64) -> Result<i64, sqlx::Error> {
        let id = sqlx::query!(
            "
            INSERT INTO vibes (name, vibe_group_id)
            VALUES (?, ?)
            RETURNING vibe_id
            ", name, vibe_group_id)
            .fetch_one(&self.pool)
            .await?
            .vibe_id
            .unwrap();
        
        Ok(id)
    }

    // READ VIBE
    pub async fn get_vibe(&self, vibe_id: i64) -> Result<Vibe, sqlx::Error> {
        sqlx::query_as!(Vibe,
            "
            SELECT vibe_id, name, vibe_group_id
            FROM vibes
            WHERE vibe_id = ?
            ", vibe_id)
            .fetch_one(&self.pool)
            .await
    }

    // UPDATE VIBE
    pub async fn change_vibe_name(&self, vibe_id: i64, name: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            UPDATE vibes
            SET name = ?
            WHERE vibe_id = ?
            ", name, vibe_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // DELETE VIBE
    pub async fn remove_vibe(&self, vibe_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            DELETE FROM vibes
            WHERE vibe_id = ?
            ", vibe_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn associate_vibe_with_track(&self, track_id: i64, vibe_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            INSERT OR IGNORE INTO track_vibes (track_id, vibe_id)
            VALUES (?, ?)
            ", track_id, vibe_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn disassociate_vibe_with_track(&self, track_id: i64, vibe_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            DELETE FROM track_vibes
            WHERE track_id = ? AND vibe_id = ?
            ", track_id, vibe_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_vibe_details_for_track(&self, track_id: i64) -> Result<Vec<VibeDetail>, sqlx::Error> {
        sqlx::query_as!(VibeDetail,
            "
            SELECT vb.name AS vibe_name, vg.name AS group_name
            FROM track_vibes AS tv
            JOIN vibes AS vb ON tv.vibe_id = vb.vibe_id
            JOIN vibe_groups AS vg ON vb.vibe_group_id = vg.vibe_group_id
            WHERE tv.track_id = ?
            ", track_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_tracks_by_vibe(&self, vibe_id: &str) -> Result<Vec<TrackPointer>, sqlx::Error> {
        sqlx::query_as!(TrackPointer,
            "
            SELECT tv.track_id AS track_id, t.path AS path
            FROM track_pointers AS t
            JOIN track_vibes AS tv ON t.track_id = tv.track_id
            WHERE tv.vibe_id = ?
            ", vibe_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_vibes_in_group(&self, vibe_group_id: i64) -> Result<Vec<Vibe>, sqlx::Error> {
        sqlx::query_as!(Vibe,
            "
            SELECT vb.vibe_id AS vibe_id, vb.name AS name, vb.vibe_group_id AS vibe_group_id
            FROM vibes AS vb
            JOIN vibe_groups AS vg ON vb.vibe_group_id = vg.vibe_group_id
            WHERE vg.vibe_group_id = ?
            ", vibe_group_id)
            .fetch_all(&self.pool)
            .await
    }

}
