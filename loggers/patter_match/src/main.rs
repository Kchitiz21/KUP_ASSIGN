use log::info;
///enum coordinate
///
enum coordinate {
    Abscissa(i32),
    Ordinate(i32),
}

///enum position which used to describe the position of Quadrant
///
enum position {
    First(coordinate, coordinate),
    Second(coordinate, coordinate),
    Third(coordinate, coordinate),
    Fourth(coordinate, coordinate),
    XAxis(coordinate, coordinate),
    YAxis(coordinate, coordinate),
    Origin(coordinate, coordinate),
}

/// Function 'check_coordinate' is used check the Quadrant of the given point
///
/// #Arguments
///
/// point: A point is tuple object of integer type
///
/// #Return
///
/// Returns the position lied point
fn check_coordinate((point_1, point_2): (i32, i32)) {
    match (point_1, point_2) {
        (point_1, point_2) if point_1 > 0 && point_2 > 0 => info!(
            "position::First(coordinate::Abscissa({}),coordinate::Ordinate({}))",
            point_1, point_2
        ),
        (point_1, point_2) if point_1 < 0 && point_2 > 0 => info!(
            "position::First(coordinate::Abscissa({}),coordinate::Ordinate({}))",
            point_1, point_2
        ),
        (point_1, point_2) if point_1 < 0 && point_2 < 0 => info!(
            "position::Third(coordinate::Abscissa({}),coordinate::Ordinate({}))",
            point_1, point_2
        ),
        (point_1, point_2) if point_1 > 0 && point_2 < 0 => info!(
            "position::Fourth(coordinate::Abscissa({}),coordinate::Ordinate({}))",
            point_1, point_2
        ),
        (point_1, point_2) if point_1 == 0 && point_2 != 0 => info!(
            "position::YAxis(coordinate::Abscissa({}),coordinate::Ordinate({}))",
            point_1, point_2
        ),
        (point_1, point_2) if point_1 != 0 && point_2 == 0 => info!(
            "position::XAxis(coordinate::Abscissa({}),coordinate::Ordinate({}))",
            point_1, point_2
        ),
        (point_1, point_2) if point_1 == 0 && point_2 == 0 => info!(
            "position::Origin(coordinate::Abscissa({}),coordinate::Ordinate({}))",
            point_1, point_2
        ),
        _ => info!("Wrong input"),
    }
}

/// Function main
fn main() {
    env_logger::init();
    let first_point = (-2, -2);
    let second_point = (0, 0);
    check_coordinate(first_point);
    check_coordinate(second_point);
}