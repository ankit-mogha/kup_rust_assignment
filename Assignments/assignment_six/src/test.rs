pub mod tests {
    use crate::main_func;
    use crate::ques1::{ques1_fn, Quadrants};
    use crate::ques2::{ques2_fn, Ip};

    #[test]
    fn check_for_fourth_quadrant() {
        let points: (i32, i32) = (2, -2);
        assert_eq!(
            ques1_fn(points),
            Quadrants::Fourth {
                abscissa: 2,
                ordinate: -2
            }
        );
    }

    #[test]
    fn check_for_first_quadrant() {
        let points: (i32, i32) = (2, 2);
        assert_eq!(
            ques1_fn(points),
            Quadrants::First {
                abscissa: 2,
                ordinate: 2
            }
        );
    }

    #[test]
    fn check_for_second_quadrant() {
        let points: (i32, i32) = (-2, 2);
        assert_eq!(
            ques1_fn(points),
            Quadrants::Second {
                abscissa: -2,
                ordinate: 2
            }
        );
    }

    #[test]
    fn check_for_third_quadrant() {
        let points: (i32, i32) = (-2, -2);
        assert_eq!(
            ques1_fn(points),
            Quadrants::Third {
                abscissa: -2,
                ordinate: -2
            }
        );
    }

    #[test]
    fn check_for_origin() {
        let points: (i32, i32) = (0, 0);
        assert_eq!(
            ques1_fn(points),
            Quadrants::Origin {
                abscissa: 0,
                ordinate: 0
            }
        );
    }

    #[test]
    fn point_on_axis() {
        let points: (i32, i32) = (0, 1);
        assert_eq!(
            ques1_fn(points),
            Quadrants::Axis {
                abscissa: 0,
                ordinate: 1
            }
        );
    }

    #[test]
    fn check_address_class_a() {
        let add: (i32, i32, i32, i32) = (0, 0, 1, 1);
        let ip: String = "0.0.1.1".to_string();
        assert_eq!(ques2_fn(add), Ip::ClassA(ip));
    }

    #[test]
    fn check_address_class_b() {
        let add: (i32, i32, i32, i32) = (128, 0, 1, 1);
        let ip: String = "128.0.1.1".to_string();
        assert_eq!(ques2_fn(add), Ip::ClassB(ip));
    }

    #[test]
    fn check_address_class_c() {
        let add: (i32, i32, i32, i32) = (192, 0, 1, 1);
        let ip: String = "192.0.1.1".to_string();
        assert_eq!(ques2_fn(add), Ip::ClassC(ip));
    }

    #[test]
    fn check_address_class_d() {
        let add: (i32, i32, i32, i32) = (224, 0, 1, 1);
        let ip: String = "224.0.1.1".to_string();
        assert_eq!(ques2_fn(add), Ip::ClassD(ip));
    }

    #[test]
    fn check_if_invalid() {
        let add: (i32, i32, i32, i32) = (-1, 0, 1, 1);
        assert_eq!(ques2_fn(add), Ip::Error("Invalid Ip Address".to_string()));
    }

    #[test]
    fn check_func_test() {
        assert!(main_func());
    }
}
