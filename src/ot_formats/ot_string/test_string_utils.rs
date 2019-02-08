#![allow(dead_code)]

use rand::{thread_rng, Rng};
use ot_core::operation::Operation;
use ot_formats::ot_string::test_ot_string;
use std::char;  

const MAX_UNICODE_CODE_POINT : u32 = 1114111;

//generates a new random insert operation
pub fn rand_insert_op() -> Operation<char>
{
    //initialize random
    let mut rng = thread_rng();
    
    //op values
    let test_char = rand_readable_char();

    let test_ix : usize = rng.gen_range(0, test_ot_string::TEST_SIZE-1);

    println!("Index : {}", test_ix);

    let test_op_id : usize = rng.gen_range(0, usize::max_value());
    let test_user_id : usize = rng.gen_range(0, usize::max_value());

    Operation::new(true, test_char, test_ix, test_op_id, test_user_id,0)
}

//generates a new random insert operation
pub fn rand_remove_op() -> Operation<char>
{
    //initialize random
    let mut rng = thread_rng();

    //op values
    let test_char = rand_readable_char();

    let test_ix : usize = rng.gen_range(0, test_ot_string::TEST_SIZE-1);

    let test_op_id : usize = rng.gen_range(0, usize::max_value());
    let test_user_id : usize = rng.gen_range(0, usize::max_value());
    
    Operation::new(false, test_char, test_ix, test_op_id, test_user_id,0)
}

//Generates a char over the full range of Unicode support (support for the full 32 bits)
pub fn rand_string(size : usize) -> String 
{
    (0..size).map(|_| rand_char()).collect()
}

//Generates a readable char for testing sanity
fn rand_readable_char() -> char
{
    //initialize random
    let mut rng = thread_rng();

    let test_char : u32 = rng.gen_range(0, 254/*Magic number MAX ASCII*/);
    match char::from_u32(test_char)
    {
        Some(ch) => return ch,
        None => panic!("Couldn't parse randomly generated char!")
    }
}

//Generates a char over the full range of Unicode support (support for the full 32 bits)
fn rand_char() -> char
{
    //initialize random
    let mut rng = thread_rng();

    let test_char : u32 = rng.gen_range(0, MAX_UNICODE_CODE_POINT/*Magic number MAX UNICODE POINT*/);
    match char::from_u32(test_char)
    {
        Some(ch) => return ch,
        None => panic!("Couldn't parse randomly generated char!")
    }
}

//Generates a readable String for testing sanity
pub fn rand_readable_string(size : usize) -> String
{
    (0..size).map(|_| rand_readable_char()).collect()
}