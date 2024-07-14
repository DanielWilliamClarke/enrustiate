use std::fmt::{Display, Formatter};

/**
 * I recently needed to calculate the center points
 * of some picture frame where each frame should
 * be equidistant from the sides of the walls and each next frame
 * this function calculate the center points of each frame
 * it can even position frames of different wides equally along a wall
 */

struct WallSpacer {
    wall_width: f32,
    frame_widths: Vec<f32>,
}

impl From<(f32, Vec<f32>)> for WallSpacer {
    fn from((wall_width, frame_widths): (f32, Vec<f32>)) -> Self {
        WallSpacer {
            wall_width,
            frame_widths,
        }
    }
}

impl Display for WallSpacer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let display_length = 150f32;
        let wall_gap_width = self.calc_gap_width();
        let wall_gap_percentage = wall_gap_width / self.wall_width;
        let gap_chars = (display_length * wall_gap_percentage).ceil() as usize;

        let center_points = self.calc_frame_center_points();

        let mut previous = 0f32;
        for center in center_points.iter() {
            let center_text_length = format!("{:.1}", center);
            let center_text_center = center_text_length.len() / 2;

            let distance = center - previous;
            previous = center + center_text_length.len() as f32;

            let pos_percentage = distance / self.wall_width;
            let pos_chars = (display_length * pos_percentage).ceil() as usize;

            // add padding
            write!(f, "{: <1$}", "", pos_chars - center_text_center)?;
            // display center point
            write!(f, "{:.1}", center)?;
        }
        writeln!(f)?;

        // draw initial gap
        write!(f, "{:-<1$}", "", gap_chars)?;
        for width in &self.frame_widths {
            let frame_gap_percentage = width / self.wall_width;
            let frame_chars = (display_length * frame_gap_percentage).floor() as usize;

            let mut frame_str = String::with_capacity(frame_chars);

            if frame_chars > 0 {
                if frame_chars % 2 == 0 {
                    let center_left = frame_chars / 2 - 1;
                    for i in 0..frame_chars {
                        if i == center_left || i == center_left + 1 {
                            frame_str.push('^');
                        } else {
                            frame_str.push('+');
                        }
                    }
                } else {
                    let center = frame_chars / 2;
                    for i in 0..frame_chars {
                        if i == center {
                            frame_str.push('^');
                        } else {
                            frame_str.push('+');
                        }
                    }
                }

                // draw frame
                write!(f, "{}", frame_str)?;
                // draw gap
                write!(f, "{:-<1$}", "", gap_chars)?;
            }
        }

        Ok(())
    }
}

impl WallSpacer {
    fn calc_gap_width(&self) -> f32 {
        let full_frame_width = self.frame_widths.iter().sum::<f32>();
        let wall_gaps = (self.frame_widths.len() + 1) as f32;
        let leftover_wall_width = self.wall_width - full_frame_width;

        leftover_wall_width / wall_gaps
    }

    pub fn calc_frame_center_points(&self) -> Vec<f32> {
        let wall_gap_width = self.calc_gap_width();

        self.frame_widths
            .iter()
            .enumerate()
            .map(|(index, width)| {
                (wall_gap_width * (index + 1) as f32) + // sum of wall gaps so far
                    self.frame_widths[0..index].iter().sum::<f32>() + // sum of all frames so far
                    (width / 2.0f32) // halfway point on current frame
            }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_center_points_23cm_frames() {
        let spacer = WallSpacer::from((212f32, vec![23f32, 23f32, 23f32, 23f32]));
        let result = spacer.calc_frame_center_points();

        println!("{}", spacer);

        assert_eq!(result, vec![35.5f32, 82.5f32, 129.5f32, 176.5f32])
    }

    #[test]
    fn generates_center_points_irregular_frames() {
        let spacer = WallSpacer::from((250f32, vec![10f32, 15f32, 52f32, 23f32, 5f32]));
        let result = spacer.calc_frame_center_points();

        println!("{}", spacer);

        assert_eq!(result, vec![29.166666f32, 65.83333f32, 123.5f32, 185.16666f32, 223.33333f32])
    }

    #[test]
    fn generates_center_points_with_growing_frames() {
        let spacer = WallSpacer::from((1000f32, vec![8f32, 16f32, 32f32, 64f32, 128f32, 256f32]));
        // let result = spacer.calc_frame_center_points();

        println!("{}", spacer);

        // assert_eq!(result, vec![29.166666f32, 65.83333f32, 123.5f32, 185.16666f32, 223.33333f32])
    }
}
