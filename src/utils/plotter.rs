use plotters::prelude::*;
use plotters_bitmap::BitMapBackend;
use std::{fs, process::Command, thread, time};
use ndarray::{Array1};
/// Renders `wave` into an in-memory RGB buffer of size (width x height).
pub fn render_wave_in_memory(wave: &ndarray::Array1<f64>, width: usize, height: usize) -> Vec<u8> {
    let mut pixel_data = vec![0u8; width * height * 3]; // for RGB24

    {
        // Create a Plotters drawing area that writes into `pixel_data`
        let drawing_area = BitMapBackend::with_buffer(&mut pixel_data, (width as u32, height as u32))
            .into_drawing_area();

        drawing_area.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&drawing_area)
            .margin(20)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0..wave.len(), -1.2..1.2)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(
                (0..wave.len()).map(|i| (i, wave[i])),
                &BLUE,
            ))
            .unwrap();
    } // the drawing_area is dropped here, ensuring the data was drawn

    pixel_data
}
pub fn plot_wave_live(wave: &Array1<f64>, frame_num: usize, temp_dir: &str) {
    let filename = format!("{}/wave_frame_{:04}.png", temp_dir, frame_num);
    let root = BitMapBackend::new(&filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Wave Evolution", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..wave.len(), -1.2..1.2)
        .unwrap();
    
    chart.configure_mesh().draw().unwrap();
    chart.draw_series(LineSeries::new((0..wave.len()).map(|i| (i, wave[i])), &BLUE)).unwrap();

    root.present().unwrap();
}
pub fn generate_video() {
    println!("🔄 Generating video using FFmpeg...");

    let output = Command::new("ffmpeg")
        .args([
            "-y",
            "-framerate", "20",  // Frame rate (adjustable)
            "-i", "temp_frames/wave_frame_%04d.png",  // Input images
            "-vcodec", "libx264",
            "-crf", "25",
            "-pix_fmt", "yuv420p",
            "wave_simulation.mp4",
        ])
        .output()
        .expect("Failed to run FFmpeg");

    if output.status.success() {
        println!("✅ Video generated: wave_simulation.mp4");
    } else {
        println!("❌ FFmpeg error: {}", String::from_utf8_lossy(&output.stderr));
    }
}

pub fn cleanup_frames(temp_dir: &str) {
    println!("🧹 Cleaning up PNG frames...");

    println!("🧹 Cleaning up temporary directory: {}", temp_dir);
    fs::remove_dir_all(temp_dir).unwrap();
    //println!("✅ Cleanup complete.");

    println!("✅ Cleanup complete.");
}