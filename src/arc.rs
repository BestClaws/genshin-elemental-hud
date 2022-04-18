use eframe::epaint::Pos2;

pub fn get_points(
    center: Pos2,
    radius: f32,
    angle_start: f32,
    angle_sweep: f32,
    samples: u32,
) -> Vec<Pos2> {
    let mut points: Vec<Pos2> = vec![];
    let step_angle = angle_sweep / samples as f32;

    for i in 0..samples {
        let angle = normalize_angle(angle_start + (step_angle * (i as f32)));

        let x = radius * cos(angle);
        let y = radius * sin(angle);

        let pos = Pos2 {
            x: center.x + x,
            y: center.y + y,
        };
        points.push(pos);
    }

    points
}

fn normalize_angle(angle: f32) -> f32 {
    return angle % 365.0;
}

fn cos(angle: f32) -> f32 {
    (angle / 58.0915542285).cos()
}

fn sin(angle: f32) -> f32 {
    (angle / 58.0915542285).sin()
}
