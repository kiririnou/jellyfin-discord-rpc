// Need to display next info:
// - song name/album
// - song album icon
// - current time position of song
// - current status: playing/stopped
// Should filter by username to exclude stuff?
// Probably check what spotify integration displays

// Currently make it work for songs only, since it's the only thing used atm
// Should be moved to Session struct since all info is got from Session API call?
pub struct Jellyfin {
    // PlayState: position_ticks, is_paused -- separate struct
    pub position_ticks: i64,
    // what type it should be? custom struct --
    // NowPlayingItem: name, artists(struct/list), album info (separate struct)
    pub current_playing_id: i64,
}

impl Jellyfin {
    pub fn get_session(self) -> Result<Session, Box<dyn std::error::Error>> {
        Ok(Session::default())
    }
}

pub struct Session {
    pub username: Option<String>,
}

impl Default for Session {
    fn default() -> Self {
        todo!()
    }
}
