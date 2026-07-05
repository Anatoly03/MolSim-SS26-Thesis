//! Simple two-dimensional console graph plotter.

use std::ops::Range;

#[derive(Debug, Default, Clone)]
pub struct ConsoleGraph {
    points: Vec<(f64, f64)>,
    lines: Vec<(f64, f64, f64, f64)>, // (x1, y1, x2, y2)

    /// The console range of the x-axis, in terms of the number of columns to print.
    pub console_width: usize,

    /// The console range of the y-axis, in terms of the number of rows to print.
    pub console_height: usize,

    /// The range of the x-axis.
    x_axis: Range<f64>,

    /// The range of the y-axis.
    y_axis: Range<f64>,
}

impl ConsoleGraph {
    pub fn add_point(&mut self, x: f64, y: f64) {
        self.points.push((x, y));

        // Update the x-axis range if necessary
        if x < self.x_axis.start {
            self.x_axis.start = x;
        }

        if x > self.x_axis.end {
            self.x_axis.end = x;
        }

        // Update the y-axis range if necessary
        if y < self.y_axis.start {
            self.y_axis.start = y;
        }

        if y > self.y_axis.end {
            self.y_axis.end = y;
        }
    }

    pub fn add_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        self.lines.push((x1, y1, x2, y2));

        // Update the x-axis range if necessary
        if x1 < self.x_axis.start {
            self.x_axis.start = x1;
        }

        if x1 > self.x_axis.end {
            self.x_axis.end = x1;
        }

        if x2 < self.x_axis.start {
            self.x_axis.start = x2;
        }

        if x2 > self.x_axis.end {
            self.x_axis.end = x2;
        }

        // Update the y-axis range if necessary
        if y1 < self.y_axis.start {
            self.y_axis.start = y1;
        }

        if y1 > self.y_axis.end {
            self.y_axis.end = y1;
        }

        if y2 < self.y_axis.start {
            self.y_axis.start = y2;
        }

        if y2 > self.y_axis.end {
            self.y_axis.end = y2;
        }
    }

    pub fn plot(&self) {
        println!("┌{}┐", "─".repeat(self.console_width + 2));
        for y in 0..self.console_height {
            print!("│ ");
            for x in 0..self.console_width {
                self.plot_pixel(x, y);
            }
            println!(" │");
        }
        println!("└{}┘", "─".repeat(self.console_width + 2));
    }

    fn plot_pixel(&self, x: usize, y: usize) {
        // Check if any point is close enough to this pixel to be plotted
        // Account for the console scaling (width and height).
        let x_range = self.x_axis.end - self.x_axis.start;
        let y_range = self.y_axis.end - self.y_axis.start;

        for &(px, py) in &self.points {
            let scaled_x = ((px - self.x_axis.start) / x_range * self.console_width as f64) as usize;
            let scaled_y = ((py - self.y_axis.start) / y_range * self.console_height as f64) as usize;

            if scaled_x == x && scaled_y == (self.console_height - 1 - y) {
                print!("*");
                return;
            }
        }

        // If no element is close enough, print a space.
        print!(" ");
    }
}
