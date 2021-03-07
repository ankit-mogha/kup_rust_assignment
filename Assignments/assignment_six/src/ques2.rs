#[derive(PartialEq, Debug)]
///Ip Enum
///
/// this enum have five variants
///
/// ClassA,ClassB,ClassC,ClassD,Error
pub enum Ip {
    ClassA(i32, i32, i32, i32),
    ClassB(i32, i32, i32, i32),
    ClassC(i32, i32, i32, i32),
    ClassD(i32, i32, i32, i32),
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
/// Returns Ip Type
pub fn ques2_fn(adder: (i32, i32, i32, i32)) -> Ip {
    match adder {
        (0, a, b, c) => Ip::ClassA(0, a, b, c),
        (128, a, b, c) => Ip::ClassB(128, a, b, c),
        (192, a, b, c) => Ip::ClassC(192, a, b, c),
        (224, a, b, c) => Ip::ClassD(224, a, b, c),
        _ => Ip::Error,
    }
}
