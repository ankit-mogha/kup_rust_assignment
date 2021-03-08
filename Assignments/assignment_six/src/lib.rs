mod ques1;
mod ques2;
mod test;

pub fn main_func() -> bool {
    let points: (i32, i32) = (2, -2);
    ques1::ques1_fn(points);

    let add: (i32, i32, i32, i32) = (192, 0, 1, 1);
    ques2::ques2_fn(add);

    true
}
