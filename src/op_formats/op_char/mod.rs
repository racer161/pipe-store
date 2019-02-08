use serde_json::Value;
//TODO: add reading from JSON
use ot_core::operation::Operation;

pub trait OPChar
{
    fn from_JSON(json : Value);
    fn to_JSON(&self) -> String;
}

impl OPChar for Operation<char>
{
    
    fn from_JSON(json : Value)
    {
        
    }
    
    fn to_JSON(&self) -> String
    {
        format!(" is_insert: {}, object: {}, index: {}, id: {}, timestamp: {}, user_id: {} ", self.is_insert, self.object, self.index, self.id, self.time_stamp, self.user_id)
    }
    
    
}