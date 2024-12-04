
mod utils;

mod header;

mod values_bool;

mod values_of_type;
//mod ijk_values_of_type;




//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod for_learning {
    use super::*;

    #[test]
    fn learn() {
        let src = "a b\tc  d \t e";
        println!("{}", src);
        let arr = src.split_ascii_whitespace();
        for (i,item) in arr.enumerate() {
            println!("#{i}:<{}>",item);
        }
    }
}
