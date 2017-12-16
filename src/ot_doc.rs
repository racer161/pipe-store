use ot_core::{doc_state, operation};
use std::fs::File;
use std::path::Path;
use std::io;
use ot_user::OTUser;


pub struct Document <'t>
{
    //TODO: PUT AN ARC ON THIS BITCH FOR SOCKET THREADING!
    ot_document : doc_state::DocState, 
    file_buffer : File,
    users : Vec<&'t OTUser<'t>>
    //TODO: PERMISSIONS! SHARING! 
}

impl<'t> Document <'t>
{
    //this path doesn't exist and 
    //we want to create a new doc for it
    fn new(p : &Path) -> io::Result<Document<'t>>
    {
        match File::create(p)
        {
            Ok (buf) => 
            {
                Ok(Document
                { 
                    ot_document : doc_state::DocState::new_empty(),
                    file_buffer : buf,
                    users : Vec::new() 
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
    
    pub fn create_new_OTUser(&mut self) -> usize
    {
        let new_user = OTUser::new(&self.ot_document);
        self.users.push(new_user);
        return self.user.len();
    }
    
    pub fn add_operation_by_peer_index(&mut self, op : operation::Operation::Operation, peer_index : usize)
    {
        self.users[peer_index].merge_op(self.doc, op);
    }
}