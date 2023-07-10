use platform::platform::{UInt, UInt8, Word};

pub type CodePtr = &UInt8;

pub struct NodeHeader {
    _hidden: Word,
    // #ifdef HAT
    //      hatNode: HNode;
    // #endif
}

/* A basic heap node format. _hidden is the combined info pointer and flags,
and is designed to be accessed via the macros only. args acts as apointer to further
arguments allocated after the node header. */
pub struct Node {
    _hidden: Word,
    //   #ifdef HAT
    //      hatNode: HNode;
    //   #endif
    args: Vec<&Node>,
}

/* an exception handler */
pub struct ExceptionHandlerNode {
    node: NodeHeader,
    next: &ExceptionHandlerNode, /* next exception handler in the stack */
    vapptr: &Node, /* vapptr of the handler code */
    ip: CodePtr, /* ip to jump to for the handler code */
    spOffs: UInt, /* offset of sp from G_spBase, offset is easier than ptr here because of GC */
    fpOffs: UInt, /* offset of fp from G_spBase, again offsets easier */
}
