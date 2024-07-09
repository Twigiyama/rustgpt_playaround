// MACRO CAPTURES

/* expr
    matches to a valid rust expression
    "hello".to_string(). vec![1,2,3], 1 + 2, 1
*/

/*stmt
    matches to a rust statement
    let x = 5, x.push(1), return Some(x)
*/

/* ident
    matches to a rust identifier
    variable_name, function_name, struct_name
*/

/* ty
    matches to a valid rust type
    i32, f32, String, Vec<i32>
*/

/* path
    matches to a valid rust path
    std::collections::HashMap, crate::module::function

*/

#[cfg(test)]

mod tests {
    use super::*;


    // macro_rules! mad_skills {
    //    /*  ($x: expr) => {
    //         format!("I have mad skills in {}", $x)
    //     }; */

    //     ($x: ty) => {
    //         match stringify!($x) {
    //             "i32" => "You sent an i32 integer".to_string(),
    //             _ => "You sent something else".to_string()
    //         }
    //     }
    // }

    macro_rules! my_vec {
        ($($x: expr),*) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        }
    }

    #[test]

    fn tests_declarative_macros() {
        println!("Hello 1");
        dbg!("Hello Again");
        let x:Vec<i32> = vec!(1, 2, 3, 4, 5);
        let formatted = format!("Hello 3 with format {:?}", x);
        dbg!(formatted);
        let y: Vec<i32> = my_vec!();
        dbg!(y);

        

        // let some_variable: String = mad_skills!(f32);
        // dbg!(some_variable);
    }
}