use std::process::{Command, Stdio};
use std::io::{Result, Write};

pub struct VideoWriter {
    child: std::process::Child,
    stdin: std::process::ChildStdin,
    width: usize,
    height: usize,
}

impl VideoWriter {
    /// Spawns ffmpeg in "rawvideo" mode reading from stdin.
    ///
    /// - `width` and `height`: The frame size in pixels (e.g. 800x600).
    /// - `fps`: Frames per second for the output.
    /// - `filename`: The output filename (e.g. "wave_simulation.mp4").
    pub fn new(width: usize, height: usize, fps: u32, filename: &str) -> Result<Self> {
        let mut child = Command::new("ffmpeg")
            .args(&[
                "-y",  // overwrite if file exists
                "-f", "rawvideo",  // reading raw frames from pipe
                "-pixel_format", "rgb24", // 24-bit RGB
                "-video_size", &format!("{}x{}", width, height),
                "-framerate", &format!("{}", fps),
                "-i", "pipe:0",  // read from stdin
                // Encode to H.264 MP4:
                "-vcodec", "libx264",
                "-pix_fmt", "yuv420p",
                "-loglevel", "error", // Suppresses all output except errors
                "-nostats",           // Prevents status/progress messages
                filename,
            ])
            .stdin(Stdio::piped())
            .spawn()?;

        let stdin = child.stdin.take().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::BrokenPipe, "Failed to open ffmpeg stdin")
        })?;

        Ok(Self {
            child,
            stdin,
            width,
            height,
        })
    }

    /// Write one raw RGB24 frame to ffmpeg.
    /// - `frame_data` must be exactly `width * height * 3` bytes of RGB.
    pub fn write_frame(&mut self, frame_data: &[u8]) -> Result<()> {
        assert_eq!(frame_data.len(), self.width * self.height * 3);
        self.stdin.write_all(frame_data)
    }

    /// Finish the video by closing stdin and waiting for ffmpeg to exit.
    pub fn finish(mut self) -> Result<()> {
        drop(self.stdin);      // close the pipe
        let status = self.child.wait()?;
        if status.success() {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("ffmpeg exited with status {}", status),
            ))
        }
    }
}
