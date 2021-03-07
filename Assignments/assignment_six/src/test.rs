pub mod tests {

    use crate::func_test;
    use crate::ques1::ques1_fn;
    use crate::ques1::Quadrants::{First, Fourth, Origin, Second, Third};
    use crate::ques2::ques2_fn;
    use crate::ques2::Ip::{ClassA, ClassB, ClassC, ClassD, Error};

    #[test]
    fn check_points() {
        let points: (i32, i32) = (2, -2);
        assert_eq!(
            ques1_fn(points),
            Fourth {
                abscissa: 2,
                ordinate: -2
            }
        );
    }

    #[test]
    fn check_points_1() {
        let points: (i32, i32) = (2, 2);
        assert_eq!(
            ques1_fn(points),
            First {
                abscissa: 2,
                ordinate: 2
            }
        );
    }

    #[test]
    fn check_points_2() {
        let points: (i32, i32) = (-2, 2);
        assert_eq!(
            ques1_fn(points),
            Second {
                abscissa: -2,
                ordinate: 2
            }
        );
    }

    #[test]
    fn check_points_3() {
        let points: (i32, i32) = (-2, -2);
        assert_eq!(
            ques1_fn(points),
            Third {
                abscissa: -2,
                ordinate: -2
            }
        );
    }

    #[test]
    fn check_points_4() {
        let points: (i32, i32) = (0, 0);
        assert_eq!(
            ques1_fn(points),
            Origin {
                abscissa: 0,
                ordinate: 0
            }
        );
    }

    #[test]
    fn check_add() {
        let add: (i32, i32, i32, i32) = (0, 0, 1, 1);
        assert_eq!(ques2_fn(add), ClassA(0, 0, 1, 1));
    }

    #[test]
    fn check_add_1() {
        let add: (i32, i32, i32, i32) = (128, 0, 1, 1);
        assert_eq!(ques2_fn(add), ClassB(128, 0, 1, 1));
    }

    #[test]
    fn check_add_2() {
        let add: (i32, i32, i32, i32) = (192, 0, 1, 1);
        assert_eq!(ques2_fn(add), ClassC(192, 0, 1, 1));
    }

    #[test]
    fn check_add_3() {
        let add: (i32, i32, i32, i32) = (224, 0, 1, 1);
        assert_eq!(ques2_fn(add), ClassD(224, 0, 1, 1));
    }

    #[test]
    fn check_add_error() {
        let add: (i32, i32, i32, i32) = (-1, 0, 1, 1);
        assert_eq!(ques2_fn(add), Error);
    }

    #[test]
    fn check_func_test() {
        assert!(func_test());
    }
}
