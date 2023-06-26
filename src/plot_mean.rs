use plotters::coord::Shift;
use plotters::prelude::*;
use crate::generate_unique_id::generate_unique_id;

fn root(fielname: &str) -> DrawingArea<BitMapBackend<'_>, Shift> {
    let root_drawing_area = BitMapBackend::new(fielname, (1920, 600));
    let drawing_area = root_drawing_area.into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    drawing_area
}

pub fn plot(data: Vec<f64>, final_result: f64) {
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let binding = format!("{}.png", generate_unique_id());
    let root_drawing_area = root(&binding);
    let mut ctx = ChartBuilder::on(&root_drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .caption("Mean Plot", ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(
            min as f64 - (min as f64 * 0.1 as f64)..max as f64 + (max as f64 * 0.1 as f64),
            0.0f64..12.0f64,
        )
        .unwrap();

    ctx.configure_mesh().draw().unwrap();
    let line_width = 2;
    ctx.draw_series(LineSeries::new(data.iter().map(|y| (*y, 0.0)),
    ShapeStyle::from(GREEN).stroke_width(4),
    )).unwrap();

    let single_point = [(final_result, 0.0)];

    ctx.draw_series(
        single_point
            .iter()
            .map(|(x, y)| Circle::new((*x, *y), 5, ShapeStyle::from(&RED).filled())),
    )
    .expect("Cannot create mean point");
}
