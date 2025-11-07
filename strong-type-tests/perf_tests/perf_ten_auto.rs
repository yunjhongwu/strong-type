// Performance benchmark: Ten types with auto_operators
// Measures impact of many types on compilation time

use strong_type::StrongType;

#[derive(StrongType)]
#[strong_type(auto_operators)]
struct Type1(i32);

#[derive(StrongType)]
#[strong_type(auto_operators)]
struct Type2(i32);

#[derive(StrongType)]
#[strong_type(auto_operators)]
struct Type3(i64);

#[derive(StrongType)]
#[strong_type(auto_operators)]
struct Type4(i64);

#[derive(StrongType)]
#[strong_type(auto_operators)]
struct Type5(f32);

#[derive(StrongType)]
#[strong_type(auto_operators)]
struct Type6(f32);

#[derive(StrongType)]
#[strong_type(auto_operators)]
struct Type7(f64);

#[derive(StrongType)]
#[strong_type(auto_operators)]
struct Type8(f64);

#[derive(StrongType)]
#[strong_type(auto_operators)]
struct Type9(u32);

#[derive(StrongType)]
#[strong_type(auto_operators)]
struct Type10(u64);

fn main() {
    let t1 = Type1::new(1_i32);
    let t2 = Type2::new(2_i32);
    let t3 = Type3::new(3_i64);
    let t4 = Type4::new(4_i64);
    let t5 = Type5::new(5.0_f32);
    let t6 = Type6::new(6.0_f32);
    let t7 = Type7::new(7.0_f64);
    let t8 = Type8::new(8.0_f64);
    let t9 = Type9::new(9_u32);
    let t10 = Type10::new(10_u64);
    println!(
        "{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
        t1, t2, t3, t4, t5, t6, t7, t8, t9, t10
    );
}
