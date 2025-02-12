mod core;

use core::wave_solver::solve_wave_1d;
use ndarray::Array1;
use plotters::prelude::*;
use std::{fs, process::Command, thread, time};
use indicatif::ProgressBar;
use std::io::{self, Write};
//use std::{thread, time};

fn plot_wave_live(wave: &Array1<f64>, frame_num: usize, temp_dir: &str) {
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
fn generate_video() {
    println!("🔄 Generating video using FFmpeg...");

    let output = Command::new("ffmpeg")
        .args([
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

fn cleanup_frames(temp_dir: &str) {
    println!("🧹 Cleaning up PNG frames...");

    println!("🧹 Cleaning up temporary directory: {}", temp_dir);
    fs::remove_dir_all(temp_dir).unwrap();
    //println!("✅ Cleanup complete.");

    println!("✅ Cleanup complete.");
}


fn main() {
    let nx = 1000;
    let timesteps = 500;
    let c = 1.0;
    let dx = 1.0 / nx as f64;
    let dt = 0.2 * dx / c;
    // Garbage library
    let temp_dir = "temp_frames";
    fs::create_dir_all(temp_dir).unwrap();
    let mut u_prev = Array1::zeros(nx);
    let mut u_curr = Array1::zeros(nx);
    for i in 1..nx - 1 {
        let x = (i as f64 - nx as f64 / 2.0) / (nx as f64 / 10.0);
        u_curr[i] = (-x * x).exp();
    }

    for i in 1..nx - 1 {
        u_prev[i] = u_curr[i] - (c * dt / dx) * (u_curr[i + 1] - u_curr[i - 1]);
    }
    let pb = ProgressBar::new(timesteps as u64);
    for t in 0..timesteps {
        let u_next = solve_wave_1d(u_prev.clone(), u_curr.clone(), c, dx, dt, 1);
        plot_wave_live(&u_next, t, temp_dir);

        // Pause for visualization (adjust speed as needed)
        thread::sleep(time::Duration::from_millis(50));

        // Shift time steps
        u_prev = u_curr.clone();
        u_curr = u_next;
        //println!("\rRendering frame {}/{}", t + 1, timesteps);
        pb.inc(1);

    }
    pb.finish_with_message("✅ Simulation complete!");
    //println!("Simulation complete. Check the frames.");
    // Generate video from frames
    generate_video();

    // Clean up PNG files
    cleanup_frames(temp_dir);
}