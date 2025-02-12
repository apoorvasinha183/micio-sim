use ndarray::{Array1, Array2};

/// Solves the 1D wave equation using the finite difference method.
///
/// ∂²u/∂t² = c² ∂²u/∂x²
///
/// - `u_prev`: Previous time step values.
/// - `u_curr`: Current time step values.
/// - `c`: Wave speed.
/// - `dx`: Space step.
/// - `dt`: Time step.
/// - `timesteps`: Number of timesteps to simulate.
///
/// Returns the final state of the system.
pub fn solve_wave_1d(
    mut u_prev: Array1<f64>,
    mut u_curr: Array1<f64>,
    c: f64,
    dx: f64,
    dt: f64,
    timesteps: usize,
) -> Array1<f64> {
    let c2 = (c * dt / dx).powi(2);
    let mut u_next = u_curr.clone();
    let nx = u_curr.len();
    for _ in 0..timesteps {
        for i in 1..u_curr.len() - 1 {
            u_next[i] = 2.0 * u_curr[i] - u_prev[i] + c2 * (u_curr[i - 1] - 2.0 * u_curr[i] + u_curr[i + 1]);
        }
        // **Apply Absorbing Boundary Conditions (Prevents Reflection for test)**
        u_next[0] = u_next[1];          // Left boundary absorbs
        u_next[nx - 1] = u_next[nx - 2]; // Right boundary absorbs
        // Reflecting
        //u_next[0] = 0.0;        // Left boundary fixed at 0
        //u_next[nx - 1] = 0.0;   // Right boundary fixed at 0
        // Shift time steps
        u_prev = u_curr.clone();
        u_curr = u_next.clone();
    }

    u_curr
}
