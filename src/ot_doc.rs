use ot_core::peer::Peer;
use std::fs::File;
use std::path::Path;
use std::io;


pub struct Document
{
    ot_document : Peer, 
    fileBuffer : File
}

impl Document
{
    //this path doesn't exist and 
    //we want to create a new doc for it
    fn new(p : &Path) -> io::Result<Document>
    {
        match File::create(p)
        {
            Ok (buf) => 
            {
                Ok(Document
                { 
                    ot_document : Peer::new(),
                    fileBuffer : buf
                })
            },
            Err(e) => Err(e) 
        }
        
        
    }
    
    //the ot_doc exists we need to read it into memory
    //TODO: Optimize this read
    //2 Ways:
    //1. Optimize for the right block size for storage type
    //2. only read the necessary operations for this documents merge
    //i.e. if we know what version this peer has we only need to read 
    //from there on
    fn from_file(p : &Path)
    {
        
    
    }
    
    
}