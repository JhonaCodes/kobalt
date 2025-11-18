//! Test to verify layout imports work correctly

use kobalt_core::layout::{MainAxisAlignment, CrossAxisAlignment, EdgeInsets};
use kobalt_core::types::Color;

fn main() {
    println!("✅ Testing layout imports...");

    let main_align = MainAxisAlignment::Center;
    let cross_align = CrossAxisAlignment::Start;
    let padding = EdgeInsets::all(10.0);

    println!("✅ MainAxisAlignment::Center: {:?}", main_align);
    println!("✅ CrossAxisAlignment::Start: {:?}", cross_align);
    println!("✅ EdgeInsets::all(10.0): {:?}", padding);

    println!("\n✅ All layout imports working correctly!");
}
