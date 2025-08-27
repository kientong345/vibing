use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Row, SqlitePool};

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct TrackHeader {
    pub id: i64,
    pub path: String,
    pub vibes: Vec<Vibe>
}

impl PartialEq for TrackHeader {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Vibe {
    pub name: String,
    pub group_name: String
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct VibeGroup {
    pub name: String,
    pub vibes: Vec<Vibe>
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
    pub async fn get_track_header(&self, track_id: i64) -> Result<Option<TrackHeader>, sqlx::Error> {
        let result = sqlx::query!(
            "
            SELECT track_id AS id, path
            FROM track_pointers
            WHERE track_id = ?
            ", track_id)
            .fetch_optional(&self.pool)
            .await?;
            
        if result.is_none() {
            return Ok(None);
        }

        let mut track_header = TrackHeader {
            id: track_id,
            path: result.unwrap().path,
            vibes: Vec::new(),
        };

        let vibes = self.get_vibes_for_track(track_id).await?;

        for vibe in vibes {
            track_header.vibes.push(vibe);
        }

        Ok(Some(track_header))
    }

    // READ TRACKS
    pub async fn get_all_tracks(&self) -> Result<Vec<TrackHeader>, sqlx::Error> {
        let mut track_headers = Vec::new();

        let records = sqlx::query!(
            "
            SELECT track_id, path
            FROM track_pointers
            ORDER BY track_id ASC
            ")
            .fetch_all(&self.pool)
            .await?;
    
        for record in records {
            let vibes = self.get_vibes_for_track(record.track_id).await?;
            track_headers.push(
                TrackHeader { id: record.track_id, path: record.path, vibes }
            );
        }

        Ok(track_headers)
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
    pub async fn remove_track(&self, track_id: i64) -> Result<(), sqlx::Error> {
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
    pub async fn get_vibe_group(&self, name: &str) -> Result<VibeGroup, sqlx::Error> {
        let record = sqlx::query!(
            "
            SELECT name
            FROM vibe_groups
            WHERE name = ?
            ", name)
            .fetch_one(&self.pool)
            .await?;

        let mut group = VibeGroup {
            name: record.name,
            vibes: Vec::new()
        };

        let vibes = self.get_vibes_in_group(name).await?;

        group.vibes = vibes;

        Ok(group)
    }

    // READ GROUPS
    pub async fn get_all_vibe_groups(&self) -> Result<Vec<VibeGroup>, sqlx::Error> {
        let mut groups = Vec::new();
        let records = sqlx::query!(
            "
            SELECT name
            FROM vibe_groups
            ")
            .fetch_all(&self.pool)
            .await?;

        for record in records {
            let vibes = self.get_vibes_in_group(&record.name).await?;
            groups.push(
                VibeGroup { name: record.name, vibes }
            );
        }

        Ok(groups)
    }

    // UPDATE GROUP
    pub async fn change_vibe_group_name(&self, old_name: &str, new_name: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            UPDATE vibe_groups
            SET name = ?
            WHERE name = ?
            ", new_name, old_name)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // DELETE GROUP
    pub async fn remove_vibe_group(&self, name: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            DELETE FROM vibe_groups
            WHERE name = ?
            ", name)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // CREATE VIBE
    pub async fn add_vibe(&self, name: &str, group_name: &str) -> Result<i64, sqlx::Error> {
        let group_id_result = sqlx::query!(
            "
            SELECT vibe_group_id AS id
            FROM vibe_groups
            WHERE name = ?
            ", group_name)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(group_id_record) = group_id_result {
            let group_id = group_id_record.id;
            let record = sqlx::query!(
                "
                INSERT INTO vibes (name, vibe_group_id)
                VALUES (?, ?)
                RETURNING vibe_id
                ", name, group_id)
                .fetch_one(&self.pool)
                .await?;
            Ok(record.vibe_id.unwrap())
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }

    // READ VIBE
    pub async fn get_vibe(&self, name: &str) -> Result<Vibe, sqlx::Error> {
        sqlx::query_as!(Vibe,
            "
            SELECT vb.name AS name, vg.name AS group_name
            FROM vibes AS vb
            JOIN vibe_groups AS vg ON vb.vibe_group_id = vg.vibe_group_id
            WHERE vb.name = ?
            ", name)
            .fetch_one(&self.pool)
            .await
    }

    // UPDATE VIBE
    pub async fn change_vibe_name(&self, old_name: &str, new_name: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            UPDATE vibes
            SET name = ?
            WHERE name = ?
            ", new_name, old_name)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // DELETE VIBE
    pub async fn remove_vibe(&self, name: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            DELETE FROM vibes
            WHERE name = ?
            ", name)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn associate_vibe_with_track(&self, track_id: i64, vibe_name: &str) -> Result<(), sqlx::Error> {
        let vibe_id = sqlx::query!(
            "
            SELECT vibe_id
            FROM vibes
            WHERE name = ?
            ", vibe_name)
            .fetch_one(&self.pool)
            .await?
            .vibe_id;

        if vibe_id.is_none() { return Ok(()); } // it's not OK!
        let vibe_id = vibe_id.unwrap();

        sqlx::query!(
            "
            INSERT OR IGNORE INTO track_vibes (track_id, vibe_id)
            VALUES (?, ?)
            ", track_id, vibe_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn disassociate_vibe_with_track(&self, track_id: i64, vibe_name: &str) -> Result<(), sqlx::Error> {
        let vibe_id = sqlx::query!(
            "
            SELECT vibe_id
            FROM vibes
            WHERE name = ?
            ", vibe_name)
            .fetch_one(&self.pool)
            .await?
            .vibe_id;

        if vibe_id.is_none() { return Ok(()); } // it's not OK!
        let vibe_id = vibe_id.unwrap();

        sqlx::query!(
            "
            DELETE FROM track_vibes
            WHERE track_id = ? AND vibe_id = ?
            ", track_id, vibe_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_vibes_for_track(&self, track_id: i64) -> Result<Vec<Vibe>, sqlx::Error> {
        Ok(sqlx::query_as!(Vibe,
            "
            SELECT vb.name AS name, vg.name AS group_name
            FROM track_vibes AS tv
            JOIN vibes AS vb ON vb.vibe_id = tv.vibe_id
            JOIN vibe_groups AS vg ON vb.vibe_group_id = vg.vibe_group_id
            WHERE tv.track_id = ?
            ", track_id)
            .fetch_all(&self.pool)
            .await?)
    }

    pub async fn get_vibes_in_group(&self, name: &str) -> Result<Vec<Vibe>, sqlx::Error> {
        sqlx::query_as!(Vibe,
            "
            SELECT vb.name AS name, vg.name AS group_name
            FROM vibes AS vb
            JOIN vibe_groups AS vg ON vb.vibe_group_id = vg.vibe_group_id
            WHERE vg.name = ?
            ", name)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_tracks_by_vibes(&self, vibe_names: &[&str]) -> Result<Vec<TrackHeader>, sqlx::Error> {
        if vibe_names.is_empty() {
            return Ok(Vec::new());
        }

        let mut tracks = Vec::new();

        let query_str = format!(
            "
            SELECT tp.track_id AS id, tp.path AS path
            FROM track_pointers AS tp
            INNER JOIN track_vibes AS tv ON tp.track_id = tv.track_id
            INNER JOIN vibes AS vb on tv.vibe_id = vb.vibe_id
            WHERE vb.name IN ({})
            GROUP BY tp.track_id, tp.path
            HAVING COUNT(DISTINCT vb.name) = {}
            ",
            vibe_names.iter().map(|_| "?").collect::<Vec<_>>().join(","),
            vibe_names.len()
        );

        let mut query = sqlx::query(&query_str);
        for name in vibe_names {
            query = query.bind(name);
        }

        let tracks_db = query.fetch_all(&self.pool).await?;

        for track in tracks_db {
            let vibes = self.get_vibes_for_track(track.get("id")).await?;
            tracks.push(
                TrackHeader { id: track.get("id"), path: track.get("path"), vibes }
            );
        }

        Ok(tracks)
    }

}
