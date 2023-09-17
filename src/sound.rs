use rodio::{Decoder, OutputStreamHandle, Source};
use std::{fs::File, io::BufReader};

pub type SoundFile = Decoder<BufReader<File>>;

pub struct Sound {
    #[allow(dead_code)] // stream is unused but it has to stay in memory
    stream: rodio::OutputStream,
    stream_handle: OutputStreamHandle,
}
impl Sound {
    pub(crate) fn new() -> Self {
        let (stream, stream_handle) =
            rodio::OutputStream::try_default().expect("can't find output device");
        Self {
            stream,
            stream_handle,
        }
    }
    pub fn play_sound<S>(&self, source: S) -> Result<(), rodio::PlayError>
    where
        S: Source<Item = f32> + Send + 'static,
    {
        self.stream_handle.play_raw(source)?;
        Ok(())
    }
}
