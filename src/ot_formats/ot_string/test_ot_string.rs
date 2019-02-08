#[cfg(test)]
#[allow(dead_code)]

use super::test_string_utils::*;
use ot_core::doc_state::DocState;
use ot_formats::ot_string::OTString;
use op_formats::op_char::OPChar;

pub const TEST_SIZE : usize = 100;

#[test]
//tests the doc_state constructor and string()
fn doc_state_get_string() 
{
    let test_string = rand_string(TEST_SIZE);
    let test_state : DocState<char> = DocState::from_string(test_string.clone());
    assert_eq!(test_string, test_state.string(),
        "

        Starting String : {}

        Get String : {}

        ",
        test_string,
        test_state.string());
}

#[test]
//tests insertion for doc_state
fn doc_state_add_insertion()
{   
    let test_operation = rand_insert_op();
    
    //generate the first half of the doc
    let first_half = rand_readable_string(test_operation.get_index().clone() as usize);
    
    //generate the latter portion of the doc after the insert
    let second_half = rand_readable_string((TEST_SIZE - test_operation.get_index().clone()) as usize);
    
    //generate the before string thats fed to the doc_state constructor
    let mut starting_doc_state_string : String = first_half.clone();
    starting_doc_state_string.push_str(&second_half.clone());
    
    
    let mut test_state = DocState::from_string(starting_doc_state_string);
    //println!("{} \n\n\n\n\n\n",test_operation.get_readable_representation());

    test_state.add(test_operation.clone());

    let after_string = test_state.string();
    
    let mut expected_string : String = first_half.clone();
    
    expected_string.push(test_operation.get_object());
    expected_string.push_str(&second_half.clone());
    
    assert_eq!(expected_string, after_string,"Expected : '{}' \n\nAfter  : '{}'\n\n", expected_string, after_string);
}

#[test]
fn doc_state_add_remove()
{
    let test_operation = rand_remove_op();
    //println!("{} \n\n\n\n\n\n",test_operation.get_readable_representation());
    
    //generate the first half of the doc
    let first_half = rand_readable_string(test_operation.get_index().clone() as usize);
    
    //generate the latter portion of the doc after the insert
    let second_half = rand_readable_string((TEST_SIZE - test_operation.get_index().clone()) as usize);
    
    //generate the before string thats fed to the doc_state constructor
    let mut starting_doc_state_string : String = first_half.clone();
    starting_doc_state_string.push(test_operation.get_object());
    starting_doc_state_string.push_str(&second_half.clone());
    
    let mut test_state = DocState::from_string(starting_doc_state_string.clone());
    
    test_state.add(test_operation.clone());
    
    let after_string = test_state.string();
    
    let mut expected_string : String = first_half.clone();
    expected_string.push_str(&second_half.clone());
    
    //println!("Starting: '{}'\nResult  : '{}' ",starting_doc_state_string,after_string);
    
    assert_eq!(expected_string, after_string,"Expected : '{}' \n\nAfter  : '{}'\n\n", expected_string, after_string);
}

#[test]
fn remove_insertion_operation()
{
    let test_operation = rand_insert_op();
    
    //generate an arbitrary test string
    let test_string = rand_readable_string(test_operation.get_index().clone() as usize);
    
    let mut test_state = DocState::from_string(test_string.clone());
    
    test_state.add(test_operation.clone());
    
    test_state.remove(test_operation.clone());
    
    let after_string = test_state.string();
    
    assert_eq!(test_string, after_string,"Expected : '{}' \n\nAfter  : '{}'\n\n", test_string, after_string);
    
}

#[test]
fn remove_remove_operation()
{
    let test_operation = rand_remove_op();
    println!("{} \n\n",test_operation.to_JSON());
    
    //generate the first half of the doc
    let first_half = rand_readable_string(test_operation.get_index().clone() as usize);
    
    //generate the latter portion of the doc after the insert
    let second_half = rand_readable_string((TEST_SIZE - test_operation.get_index().clone()) as usize);
    
    //generate the before string thats fed to the doc_state constructor
    let mut starting_doc_state_string : String = first_half.clone();
    starting_doc_state_string.push(test_operation.get_object());
    starting_doc_state_string.push_str(&second_half.clone());
    
    let mut test_state = DocState::from_string(starting_doc_state_string.clone());
    
    test_state.add(test_operation.clone());
    
    test_state.remove(test_operation.clone());
    
    let after_string = test_state.string();
    
    //println!("Starting: '{}'\nResult  : '{}' ",starting_doc_state_string,after_string);
    
    assert_eq!(starting_doc_state_string.clone(), after_string,"Expected : '{}' \n\nAfter  : '{}'\n\n", starting_doc_state_string.clone(), after_string);
}