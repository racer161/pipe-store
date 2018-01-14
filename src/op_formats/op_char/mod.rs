use serde_json::Value;

trait OPChar
{
    pub fn from_JSON(json : Value);
}

impl OPChar for Operation<char>
{
    
    fn from_JSON(json : Value)
    {
        
    }
}