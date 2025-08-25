use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use std::fs::File;
use std::path::PathBuf;
use std::time::{Duration, Instant};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum State {
    Playing,
    Paused,
    Stopped,
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: Option<Duration>,
}

pub struct Audio {
    path: PathBuf,
    sink: Sink,
    _stream_handle: OutputStream,
    state: State,
    playback_start_time: Option<Instant>,
    paused_duration: Duration,
}

impl Audio {
    pub fn new(audio_path: &str) -> Self {
        let path = PathBuf::from(audio_path);

        let _stream_handle = OutputStreamBuilder::open_default_stream()
            .expect("cannot open stream");

        let sink = Sink::connect_new(&_stream_handle.mixer());

        let file = File::open(&path)
            .expect("cannot open file");

        let source = Decoder::try_from(file)
            .expect("cannot load song");

        sink.append(source);
        sink.pause();

        Audio {
            path,
            sink,
            _stream_handle,
            state: State::Paused,
            playback_start_time: None,
            paused_duration: Duration::from_secs(0),
        }
    }

    pub fn get_metadata(&self) -> Metadata {
        let tag = audiotags::Tag::new().read_from_path(&self.path)
            .expect("cannot get metadata");

        Metadata {
            title: tag.title().map(String::from),
            artist: tag.artist().map(String::from),
            album: tag.album_title().map(String::from),
            duration: tag.duration().map(Duration::from_secs_f64),
        }
    }

    pub fn get_current_state(&self) -> State {
        if self.sink.is_paused() {
            State::Paused
        } else if self.sink.empty() {
            State::Stopped
        } else {
            self.state.clone()
        }
    }

    pub fn play(&mut self) {
        if self.sink.is_paused() {
            println!("cp1");
            self.sink.play();
            println!("cp2");
            self.state = State::Playing;
            if self.playback_start_time.is_none() {
                self.playback_start_time = Some(Instant::now());
            }
        }
    }

    pub fn pause(&mut self) {
        if !self.sink.is_paused() {
            self.sink.pause();
            self.state = State::Paused;
            if let Some(start_time) = self.playback_start_time.take() {
                self.paused_duration += start_time.elapsed();
            }
        }
    }

    pub fn get_volume(&self) -> f32 {
        self.sink.volume()
    }

    pub fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume.max(0.0)); // Volume can't be negative
    }

    pub fn get_elapsed_time(&self) -> Duration {
        match self.playback_start_time {
            Some(start_time) if self.state == State::Playing => {
                self.paused_duration + start_time.elapsed()
            }
            _ => self.paused_duration,
        }
    }

    // pub fn set_elapsed_time(&mut self, time: Duration) {
    //     // This requires recreating the source, which is inefficient but the only way in rodio
    //     self.sink.clear(); // Remove the old source
    //     let file = File::open(&self.path)
    //         .expect("cannot open file");
    //     let source = Decoder::new(BufReader::new(file))
    //         .unwrap().skip_duration(time);

    //     self.sink.append(source);

    //     // Reset time tracking
    //     self.paused_duration = time;
    //     if self.state == State::Playing {
    //         self.playback_start_time = Some(Instant::now());
    //     } else {
    //         self.playback_start_time = None;
    //     }
    // }

    pub fn is_end(&self) -> bool {
        self.sink.empty() && self.state != State::Paused
    }
}
