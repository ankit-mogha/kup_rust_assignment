#[derive(PartialEq, Debug)]
///Quadrants Enum
///
/// this enum have six variants
///
/// First,Second,Third,Fourth,Origin,Error
pub enum Quadrants {
    First { abscissa: i32, ordinate: i32 },
    Second { abscissa: i32, ordinate: i32 },
    Third { abscissa: i32, ordinate: i32 },
    Fourth { abscissa: i32, ordinate: i32 },
    Origin { abscissa: i32, ordinate: i32 },
    Error,
}
/// This function matches the pattern.
///
/// #Arguments
///
/// tuple
///
/// #Return
///
/// Returns Quadrants type
pub fn ques1_fn(pts: (i32, i32)) -> Quadrants {
    match pts {
        (2, 2) => Quadrants::First {
            abscissa: pts.0,
            ordinate: pts.1,
        },
        (-2, 2) => Quadrants::Second {
            abscissa: pts.0,
            ordinate: pts.1,
        },
        (-2, -2) => Quadrants::Third {
            abscissa: pts.0,
            ordinate: pts.1,
        },
        (2, -2) => Quadrants::Fourth {
            abscissa: pts.0,
            ordinate: pts.1,
        },
        (0, 0) => Quadrants::Origin {
            abscissa: pts.0,
            ordinate: pts.1,
        },
        _ => Quadrants::Error,
    }
}
