use serde::Serialize;
use sqlx::FromRow;
use chrono::{DateTime,Utc};

#[derive(Serialize,FromRow,Debug)]
pub struct Song {
    create_by:Option<String>,
    created_at:DateTime<Utc>,
    song_type:Option<String>,
    update_by:Option<String>,
    song_cover:Option<String>,
    song_name:Option<String>,
    recommend:Option<bool>,
    song_singer_id:Option<String>,
    is_new:Option<bool>,
    ranking_list_id:Option<String>,
    lyrics_id:Option<String>,
    song_description:Option<String>,
    deleted_at:Option<DateTime<Utc>>,
    play_count:Option<u64>,
    updated_at:Option<DateTime<Utc>>,
}