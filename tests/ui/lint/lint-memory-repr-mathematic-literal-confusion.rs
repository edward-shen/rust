// run-rustfix
fn main() {
    let _: i32 = 0b1000_0000_0000_0000_0000_0000_0000_0000i32;
    //~^ ERROR literal out of range
    //~| HELP literals in binary or hexidecimal
    let _: i32 = -0b1000_0000_0000_0000_0000_0000_0000_0000i32;
    let _: i32 = 0b1000_0000_0000_0000_0000_0000_0000_0000;
    //~^ ERROR literal out of range
    //~| HELP literals in binary or hexidecimal
    let _: i32 = -0b1000_0000_0000_0000_0000_0000_0000_0000;
}
