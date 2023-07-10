use node::CodePtr;

/* a frame is the part of the stack that records successive applications,
it is stored as a structure making additions and the like very easy */
pub struct Frame {
    fp: &Frame,
    ip: CodePtr,
    vapptr: &Node,
}
