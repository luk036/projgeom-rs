//! SVG visualization utilities
//!
//! This module provides utilities for visualizing geometric objects
//! using SVG format.

use crate::pg_object::{PgPoint, PgLine};
use std::fmt::Write;

/// SVG renderer for geometric objects
pub struct SvgRenderer {
    /// Width of the SVG canvas
    width: usize,
    /// Height of the SVG canvas
    height: usize,
    /// Scale factor for coordinates
    scale: f64,
    /// X offset for centering
    offset_x: f64,
    /// Y offset for centering
    offset_y: f64,
}

impl SvgRenderer {
    /// Create a new SVG renderer
    ///
    /// # Arguments
    ///
    /// * `width` - Width of the SVG canvas
    /// * `height` - Height of the SVG canvas
    /// * `scale` - Scale factor for coordinates
    pub fn new(width: usize, height: usize, scale: f64) -> Self {
        let offset_x = width as f64 / 2.0;
        let offset_y = height as f64 / 2.0;

        SvgRenderer {
            width,
            height,
            scale,
            offset_x,
            offset_y,
        }
    }

    /// Convert homogeneous coordinates to SVG coordinates
    fn to_svg_coords(&self, point: &PgPoint) -> Option<(f64, f64)> {
        if point.coord[2] == 0 {
            return None; // Point at infinity
        }

        let x = point.coord[0] as f64 / point.coord[2] as f64;
        let y = point.coord[1] as f64 / point.coord[2] as f64;

        // Convert to SVG coordinates (y is flipped)
        let svg_x = self.offset_x + x * self.scale;
        let svg_y = self.offset_y - y * self.scale;

        Some((svg_x, svg_y))
    }

    /// Start an SVG document
    pub fn start(&self) -> String {
        format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#,
            self.width, self.height, self.width, self.height
        )
    }

    /// End an SVG document
    pub fn end(&self) -> String {
        "</svg>".to_string()
    }

    /// Draw a point
    ///
    /// # Arguments
    ///
    /// * `point` - The point to draw
    /// * `color` - Color of the point
    /// * `radius` - Radius of the point
    pub fn draw_point(&self, point: &PgPoint, color: &str, radius: f64) -> String {
        if let Some((x, y)) = self.to_svg_coords(point) {
            format!(
                r#"<circle cx="{:.2}" cy="{:.2}" r="{:.2}" fill="{}" />"#,
                x, y, radius, color
            )
        } else {
            String::new()
        }
    }

    /// Draw a line
    ///
    /// # Arguments
    ///
    /// * `line` - The line to draw
    /// * `color` - Color of the line
    /// * `stroke_width` - Width of the line
    pub fn draw_line(&self, line: &PgLine, color: &str, stroke_width: f64) -> String {
        // Find two points on the line
        let a = line.coord[0] as f64;
        let b = line.coord[1] as f64;
        let c = line.coord[2] as f64;

        if a.abs() > b.abs() {
            // Line is more vertical: solve for x
            let y1 = -c / b;
            let y2 = -(c + a * 1000.0) / b;
            let x1 = 0.0;
            let x2 = 1000.0;

            if let Some((svg_x1, svg_y1)) = self.to_svg_coords(&PgPoint::new([x1 as i64, y1 as i64, 1])) {
                if let Some((svg_x2, svg_y2)) = self.to_svg_coords(&PgPoint::new([x2 as i64, y2 as i64, 1])) {
                    return format!(
                        r#"<line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" stroke="{}" stroke-width="{:.2}" />"#,
                        svg_x1, svg_y1, svg_x2, svg_y2, color, stroke_width
                    );
                }
            }
        } else {
            // Line is more horizontal: solve for y
            let x1 = -c / a;
            let x2 = -(c + b * 1000.0) / a;
            let y1 = 0.0;
            let y2 = 1000.0;

            if let Some((svg_x1, svg_y1)) = self.to_svg_coords(&PgPoint::new([x1 as i64, y1 as i64, 1])) {
                if let Some((svg_x2, svg_y2)) = self.to_svg_coords(&PgPoint::new([x2 as i64, y2 as i64, 1])) {
                    return format!(
                        r#"<line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" stroke="{}" stroke-width="{:.2}" />"#,
                        svg_x1, svg_y1, svg_x2, svg_y2, color, stroke_width
                    );
                }
            }
        }

        String::new()
    }

    /// Draw a segment between two points
    ///
    /// # Arguments
    ///
    /// * `p1` - First point
    /// * `p2` - Second point
    /// * `color` - Color of the segment
    /// * `stroke_width` - Width of the segment
    pub fn draw_segment(&self, p1: &PgPoint, p2: &PgPoint, color: &str, stroke_width: f64) -> String {
        if let Some((x1, y1)) = self.to_svg_coords(p1) {
            if let Some((x2, y2)) = self.to_svg_coords(p2) {
                return format!(
                    r#"<line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" stroke="{}" stroke-width="{:.2}" />"#,
                    x1, y1, x2, y2, color, stroke_width
                );
            }
        }
        String::new()
    }

    /// Draw a triangle
    ///
    /// # Arguments
    ///
    /// * `vertices` - Array of three vertices
    /// * `fill_color` - Fill color (use "none" for no fill)
    /// * `stroke_color` - Stroke color
    /// * `stroke_width` - Width of the stroke
    pub fn draw_triangle(
        &self,
        vertices: &[PgPoint; 3],
        fill_color: &str,
        stroke_color: &str,
        stroke_width: f64,
    ) -> String {
        let mut points = String::new();

        for vertex in vertices {
            if let Some((x, y)) = self.to_svg_coords(vertex) {
                write!(&mut points, "{:.2},{:.2} ", x, y).unwrap();
            } else {
                return String::new();
            }
        }

        format!(
            r#"<polygon points="{}" fill="{}" stroke="{}" stroke-width="{:.2}" />"#,
            points, fill_color, stroke_color, stroke_width
        )
    }

    /// Draw a circle
    ///
    /// # Arguments
    ///
    /// * `center` - Center of the circle
    /// * `radius` - Radius of the circle
    /// * `fill_color` - Fill color (use "none" for no fill)
    /// * `stroke_color` - Stroke color
    /// * `stroke_width` - Width of the stroke
    pub fn draw_circle(
        &self,
        center: &PgPoint,
        radius: f64,
        fill_color: &str,
        stroke_color: &str,
        stroke_width: f64,
    ) -> String {
        if let Some((cx, cy)) = self.to_svg_coords(center) {
            let svg_radius = radius * self.scale;
            format!(
                r#"<circle cx="{:.2}" cy="{:.2}" r="{:.2}" fill="{}" stroke="{}" stroke-width="{:.2}" />"#,
                cx, cy, svg_radius, fill_color, stroke_color, stroke_width
            )
        } else {
            String::new()
        }
    }

    /// Draw text at a point
    ///
    /// # Arguments
    ///
    /// * `point` - Position for the text
    /// * `text` - Text to display
    /// * `color` - Color of the text
    /// * `font_size` - Font size
    pub fn draw_text(&self, point: &PgPoint, text: &str, color: &str, font_size: f64) -> String {
        if let Some((x, y)) = self.to_svg_coords(point) {
            format!(
                r#"<text x="{:.2}" y="{:.2}" fill="{}" font-size="{:.2}">{}</text>"#,
                x, y, color, font_size, text
            )
        } else {
            String::new()
        }
    }

    /// Draw coordinate axes
    ///
    /// # Arguments
    ///
    /// * `color` - Color of the axes
    /// * `stroke_width` - Width of the axes
    pub fn draw_axes(&self, color: &str, stroke_width: f64) -> String {
        let origin = PgPoint::new([0, 0, 1]);
        let x_axis_end = PgPoint::new([1000, 0, 1]);
        let y_axis_end = PgPoint::new([0, 1000, 1]);

        let mut svg = String::new();
        svg.push_str(&self.draw_segment(&origin, &x_axis_end, color, stroke_width));
        svg.push_str(&self.draw_segment(&origin, &y_axis_end, color, stroke_width));

        svg
    }

    /// Draw a grid
    ///
    /// # Arguments
    ///
    /// * `spacing` - Spacing between grid lines
    /// * `color` - Color of the grid
    /// * `stroke_width` - Width of the grid lines
    pub fn draw_grid(&self, spacing: i64, color: &str, stroke_width: f64) -> String {
        let mut svg = String::new();

        // Vertical lines
        for i in -100..=100 {
            let x = i * spacing;
            let p1 = PgPoint::new([x, -1000, 1]);
            let p2 = PgPoint::new([x, 1000, 1]);
            svg.push_str(&self.draw_segment(&p1, &p2, color, stroke_width));
        }

        // Horizontal lines
        for i in -100..=100 {
            let y = i * spacing;
            let p1 = PgPoint::new([-1000, y, 1]);
            let p2 = PgPoint::new([1000, y, 1]);
            svg.push_str(&self.draw_segment(&p1, &p2, color, stroke_width));
        }

        svg
    }
}

impl Default for SvgRenderer {
    fn default() -> Self {
        Self::new(800, 600, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_renderer() {
        let renderer = SvgRenderer::new(800, 600, 50.0);

        let svg = format!(
            "{}{}{}{}",
            renderer.start(),
            renderer.draw_axes("black", 1.0),
            renderer.draw_point(&PgPoint::new([1, 1, 1]), "red", 5.0),
            renderer.end()
        );

        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("circle"));
        assert!(svg.contains("line"));
    }

    #[test]
    fn test_draw_triangle() {
        let renderer = SvgRenderer::new(800, 600, 50.0);

        let triangle = [
            PgPoint::new([0, 0, 1]),
            PgPoint::new([2, 0, 1]),
            PgPoint::new([1, 2, 1]),
        ];

        let svg = renderer.draw_triangle(&triangle, "lightblue", "blue", 2.0);

        assert!(svg.contains("<polygon"));
        assert!(svg.contains("fill=\"lightblue\""));
    }
}
