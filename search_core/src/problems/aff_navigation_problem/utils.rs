use std::collections::HashMap;
use ordered_float::OrderedFloat;
use serde::{Serialize, Deserialize};

/// Represents the discrete elevation levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZLevel {
    Ground,
    Low,
    Medium,
    High,
    MaxAltitude,
}

/// Converts a ZLevel into its corresponding numerical value.
/// Ground = 0.0, Low = 1.0, Medium = 2.0, High = 3.0, MaxAltitude = 4.0.
pub fn zlevel_to_f64(z: ZLevel) -> f64 {
    match z {
        ZLevel::Ground      => 0.0,
        ZLevel::Low         => 1.0,
        ZLevel::Medium      => 2.0,
        ZLevel::High        => 3.0,
        ZLevel::MaxAltitude => 4.0,
    }
}

/// Uses Bresenham's algorithm to generate the grid cells along a straight line
/// between two integer points. This is useful for checking the path over a grid.
pub fn bresenham_line(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<(i32, i32)> {
    let mut cells = Vec::new();
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut x = x0;
    let mut y = y0;

    loop {
        cells.push((x, y));
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
    cells
}

/// Determines if there is a line-of-sight between two points that lie at the same
/// discrete elevation level (ZLevel). The points are given as continuous coordinates (f64, f64),
/// and the terrain is provided via a dtm, a HashMap that maps grid cells (i32, i32) to an elevation value.
///
/// The function works by converting the continuous coordinates to grid cells (using rounding),
/// then generating the line of cells using Bresenham's algorithm, and finally checking that every cell's
/// terrain elevation (defaulting to 0.0 if missing) is strictly lower than the numerical equivalent of the given ZLevel.
pub fn has_line_of_sight_same_zlevel(
    p1: (f64, f64),
    p2: (f64, f64),
    elevation: ZLevel,
    dtm: &HashMap<(i32, i32), OrderedFloat<f64>>,
) -> bool {
    let elev_value = zlevel_to_f64(elevation);

    // Convert continuous coordinates to grid cells (rounding to nearest integer)
    let (x0, y0) = (p1.0.round() as i32, p1.1.round() as i32);
    let (x1, y1) = (p2.0.round() as i32, p2.1.round() as i32);

    // Generate the grid cells along the line between the two points.
    let cells = bresenham_line(x0, y0, x1, y1);

    // Check each cell along the line.
    for cell in cells {
        // Look up the terrain elevation at the grid cell; default to 0.0 if missing.
        let terrain = dtm.get(&cell).map(|v| v.into_inner()).unwrap_or(0.0);
        if terrain >= elev_value {
            return false;
        }
    }

    true
}



/// Checks line-of-sight between two points that may be at different discrete elevation levels (ZLevel)
/// without dense sampling. Instead, we use Bresenham’s algorithm to iterate over only the grid cells
/// that the line passes through and compute the expected elevation at each cell.
///
/// p1 and p2 are given as (x, y, ZLevel) with x,y as continuous coordinates.
/// The dtm maps grid cells (i32, i32) to a terrain elevation (as an OrderedFloat<f64>),
/// with elevation values assumed to be integers from 0 to 4.
pub fn has_line_of_sight(
    p1: (f64, f64, ZLevel),
    p2: (f64, f64, ZLevel),
    dtm: &HashMap<(i32, i32), OrderedFloat<f64>>,
) -> bool {
    // Convert discrete ZLevels to numerical values (0.0 to 4.0)
    let z1 = zlevel_to_f64(p1.2);
    let z2 = zlevel_to_f64(p2.2);

    // Convert the continuous coordinates to grid cells by rounding.
    let (x0, y0) = (p1.0.round() as i32, p1.1.round() as i32);
    let (x1, y1) = (p2.0.round() as i32, p2.1.round() as i32);

    // Compute the Euclidean distance between the source and target grid cells.
    let total_distance = (((x1 - x0).pow(2) + (y1 - y0).pow(2)) as f64).sqrt();
    // If the points are in the same cell, we assume LOS exists.
    if total_distance == 0.0 {
        return true;
    }

    // Overall slope (rise over run) from source to target.
    let overall_slope = (z2 - z1) / total_distance;

    // Get the list of grid cells along the line from source to target.
    let cells = bresenham_line(x0, y0, x1, y1);

    // For each cell along the line, compute the expected elevation based on the linear interpolation.
    // We assume that the center of the source cell is our starting point.
    for (cell_x, cell_y) in cells {
        // Compute the Euclidean distance from the source cell (x0,y0) to the current cell.
        let d = (((cell_x - x0).pow(2) + (cell_y - y0).pow(2)) as f64).sqrt();
        // Skip the source cell itself.
        if d == 0.0 {
            continue;
        }
        let expected_elev = z1 + overall_slope * d;

        // Look up the terrain elevation for the cell; default to 0 if missing.
        let terrain = dtm.get(&(cell_x, cell_y)).map(|v| v.into_inner()).unwrap_or(0.0);

        // If the terrain elevation is higher than the expected elevation at this cell,
        // the line-of-sight is blocked.
        if terrain > expected_elev {
            return false;
        }
    }

    true
}

