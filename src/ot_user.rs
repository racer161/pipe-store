use ot_core::peer::Peer;
use ot_doc::Document;

pub struct OTUser<'u>
{
    doc : &'u Document<'u>,
    peer : Peer 
}

impl<'u> OTUser <'u>
{
    pub fn new( doc : &'u Document) -> OTUser<'u>
    {
        return OTUser
        {
            doc : doc,
            peer : Peer::new()
        
        }
    }
    
    
}