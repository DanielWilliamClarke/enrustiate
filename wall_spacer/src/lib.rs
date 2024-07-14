pub fn calc_frame_center_points(wall_width: f32, frame_widths: Vec<f32>) -> Vec<f32> {
    let full_frame_width = frame_widths.iter().sum::<f32>();
    let wall_gaps = (frame_widths.len() + 1) as f32;
    let left_over_wall_width = wall_width - full_frame_width;
    let wall_gap_width = left_over_wall_width / wall_gaps;

    frame_widths
        .iter()
        .enumerate()
        .map(|(index, width)| {
            (wall_gap_width * (index + 1) as f32) + // sum of wall gaps so far
                frame_widths[0..index].iter().sum::<f32>() + // sum of all frames so far
                (width / 2.0f32) // halfway point on current frame
        }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_center_points_23cm_frames() {
        let result = calc_frame_center_points(212f32, vec![23f32, 23f32, 23f32, 23f32]);
        println!("{:?}", result);

        assert_eq!(result, vec![35.5f32, 82.5f32, 129.5f32, 176.5f32])
    }
}
