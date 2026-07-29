use headless_chrome::Tab;
use image::codecs::gif::{GifEncoder, Repeat};
use image::{Delay, Frame, ImageFormat};
use std::fs::File;
use std::path::Path;
use std::time::Duration;

pub struct FrameRecorder {
    frames: Vec<Vec<u8>>,
}

impl FrameRecorder {
    pub fn new() -> Self {
        Self { frames: Vec::new() }
    }

    pub fn capture_frame(&mut self, tab: &Tab) -> Result<(), Box<dyn std::error::Error>> {
        let png_bytes = tab.capture_screenshot(
            headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
            None,
            None,
            true,
        )?;
        self.frames.push(png_bytes);
        Ok(())
    }

    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    pub fn save_animated_gif<P: AsRef<Path>>(
        &self,
        path: P,
        frame_delay_ms: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.frames.is_empty() {
            println!("⚠️ [Video Recorder] No frames captured for video encoding.");
            return Ok(());
        }

        println!(
            "🎬 [Video Recorder] Encoding {} captured frames into GIF...",
            self.frames.len()
        );

        let file = File::create(path.as_ref())?;
        let mut encoder = GifEncoder::new_with_speed(file, 10);
        encoder.set_repeat(Repeat::Infinite)?;

        let delay = Delay::from_saturating_duration(Duration::from_millis(frame_delay_ms));

        for (idx, png_bytes) in self.frames.iter().enumerate() {
            let dyn_img = image::load_from_memory_with_format(png_bytes, ImageFormat::Png)?;
            let resized = dyn_img.resize_exact(1280, 720, image::imageops::FilterType::Triangle);
            let frame = Frame::from_parts(resized.to_rgba8(), 0, 0, delay);
            encoder.encode_frame(frame)?;
            if (idx + 1) % 5 == 0 || idx + 1 == self.frames.len() {
                println!(
                    "   🎥 Encoded video frame {}/{}",
                    idx + 1,
                    self.frames.len()
                );
            }
        }

        println!("✨ Video recording saved to: {}", path.as_ref().display());
        Ok(())
    }
}
