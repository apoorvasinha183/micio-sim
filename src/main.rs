mod core;
mod utils;
use core::wave_solver::solve_wave_1d;
use utils::video_writer::VideoWriter;
use utils::plotter::render_wave_in_memory;
use utils::plotter::cleanup_frames;
use ndarray::Array1;
//use plotters::prelude::*;
use std::{fs, process::Command, thread, time};
use indicatif::ProgressBar;
//use std::io::{self, Write};
//use std::{thread, time};




fn main() -> std::io::Result<()> {
    let nx = 1000;
    let timesteps = 5000;
    let c = 10000.0;
    let dx = 1.0 / nx as f64;
    let dt = 0.2 * dx / c;
    // Garbage library
    let temp_dir = "temp_frames";
    fs::create_dir_all(temp_dir).unwrap();
    let mut u_prev = Array1::zeros(nx);
    let mut u_curr = Array1::zeros(nx);
    let c2 = (c * dt / dx).powi(2);
    for i in 1..nx - 1 {
        let x = (i as f64 - nx as f64 / 2.0) / (nx as f64 / 100.0);
        u_curr[i] = (-x * x).exp();
    }
    // Suppose we have a Gaussian in u_curr[i].
    // We'll define partial_u/partial_x for each point
    // and use that as the initial velocity.

    let alpha = 1.0 * c; // pick some fraction of c, or c itself
    for i in 1..nx - 1 {
        let dudx = (u_curr[i+1] - u_curr[i-1]) / (2.0 * dx);
        // Right-moving wave: negative sign in front
        let v0 = -alpha * dudx;

        // Then compute u_prev using the second-order wave initialization
        let lap = u_curr[i + 1] - 2.0 * u_curr[i] + u_curr[i - 1];
        u_prev[i] = u_curr[i]
            - dt * v0
            + 0.5 * c2 * lap;
    }
    let width = 800;
    let height = 600;

    // Instead of writing PNGs, spawn the rawvideo pipeline to "wave_simulation.mp4"
    let fps = 20;
    let mut video = VideoWriter::new(width, height, fps, "wave_simulation.mp4")?;
    let pb = ProgressBar::new(timesteps as u64);
    for t in 0..timesteps {
        let u_next = solve_wave_1d(u_prev.clone(), u_curr.clone(), c, dx, dt, 1);
        //plot_wave_live(&u_next, t, temp_dir);
        let frame_data = render_wave_in_memory(&u_next, width, height);
        // Pause for visualization (adjust speed as needed)
        //thread::sleep(time::Duration::from_millis(50));
        // Write that frame to ffmpeg's stdin
        video.write_frame(&frame_data)?;

        // Shift time steps
        u_prev = u_curr.clone();
        u_curr = u_next;
        //println!("\rRendering frame {}/{}", t + 1, timesteps);
        pb.inc(1);

    }
    pb.finish_with_message("✅ Simulation complete!");
    //println!("Simulation complete. Check the frames.");
    // Generate video from frames
    video.finish()?;
    println!("✅ Simulation + encoding complete!");

    // Clean up PNG files
    cleanup_frames(temp_dir);
    Ok(())
}