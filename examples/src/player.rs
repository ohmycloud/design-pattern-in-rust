// A music track
pub struct Track {
    pub title: String,
    pub duration: u32,
    cursor: u32,
}

impl Track {
    pub fn new(title: &'static str, duration: u32) -> Self {
        Self {
            title: title.into(),
            duration,
            cursor: 0,
        }
    }
}

/// A music player holds a playlist and it can do basic operations over it
pub struct Player {
    playlist: Vec<Track>,
    current_track: usize,
    _volume: u8,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            playlist: vec![
                Track::new("My Kind", 207),
                Track::new("Inside The Lines", 204),
                Track::new("Vote", 277),
                Track::new("In The End", 330),
                Track::new("我记得", 329),
            ],
            current_track: 0,
            _volume: 25,
        }
    }
}

impl Player {
    pub fn next_track(&mut self) {
        self.current_track = (self.current_track + 1) % self.playlist.len();
    }
    pub fn prev_track(&mut self) {
        self.current_track = (self.playlist.len() + self.current_track - 1) % self.playlist.len();
    }
    pub fn play(&mut self) {
        // Playback imitation
        self.track_mut().cursor = 10;
    }
    pub fn pause(&mut self) {
        // Paused at some moment.
        self.track_mut().cursor = 43;
    }
    pub fn rewind(&mut self) {
        self.track_mut().cursor = 0;
    }
    pub fn track(&self) -> &Track {
        &self.playlist[self.current_track]
    }
    fn track_mut(&mut self) -> &mut Track {
        &mut self.playlist[self.current_track]
    }
}
