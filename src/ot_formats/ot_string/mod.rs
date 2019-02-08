mod test_ot_string;
mod test_string_utils;

use std::collections::BTreeSet;
use ot_core::doc_state::DocState; 

trait OTString
{
    fn string(&self ) -> String;
    fn from_string(start_string : String ) -> DocState<char>;
}

impl OTString for DocState<char>
{
    //custom constructor for charDoc
    fn from_string(start_string : String ) -> DocState<char>
    {
        let doc : DocState<char> = DocState 
        { 
            operations : Vec::new(),
            deletions : BTreeSet::new(),
            doc_str : doc_str_from_string(start_string)
        };
        doc
    }
    
    fn string(&self ) -> String
    {
        let mut return_string = String::new();
        for ch in self.doc_str()
        {
            return_string.push(ch.clone());
        }
        
        return return_string;
    }
}

pub fn doc_str_from_string(start_string : String) -> Vec<char>
{
    let mut final_vec : Vec<char> = Vec::new();
    for ch in start_string.chars()
    {
        final_vec.push(ch.clone());
    }
    return final_vec;
}