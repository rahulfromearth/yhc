use std::sync::Arc;

use crate::{
    module::Module,
    platform::platform::{HUInt, Int, QUInt, UByte, UInt, UInt8, Word},
};
pub type CodePtr<'a> = &'a UInt8;

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
    args: Vec<Arc<Node>>,
}

/* an exception handler */
pub struct ExceptionHandlerNode<'a> {
    node: NodeHeader,
    next: Arc<ExceptionHandlerNode<'a>>, /* next exception handler in the stack */
    vapptr: Arc<Node>,                   /* vapptr of the handler code */
    ip: CodePtr<'a>,                     /* ip to jump to for the handler code */
    spOffs: UInt, /* offset of sp from G_spBase, offset is easier than ptr here because of GC */
    fpOffs: UInt, /* offset of fp from G_spBase, again offsets easier */
}

/* basic field shared by all information structures */
pub struct Info {
    tag: HUInt,
}

/* Partial application information, records how many arguments we have and how many remain
to be satured. */
struct PInfo {
    info: Info,
    size: QUInt,
    need: QUInt,
}

type ConstItem = Word;

/* function info,  must always be preceeded by the pap table for the function of the
appropriate size. */
pub struct FInfo<'a> {
    info: Info,
    papTable: &'a PInfo,    /* partial application table */
    link: Arc<FInfo<'a>>,   /* for garbage collecting CAFs */
    arity: HUInt,           /* function arity */
    stack: HUInt,           /* function stack usage - UNUSED */
    flags: HUInt,           /* function flags */
    module: &'a Module<'a>, /* the module this finfo was loaded from */
    name: String,           /* function name */

    //C   #ifdef HAT
    //C     HNode            hatNode,
    //C     HInfo            hatInfo,
    //C   #endif

    /* FInfo specific */
    codeSize: Int,
    code: CodePtr<'a>,         /* pointer to byte code */
    numConsts: HUInt,          /* number of constants */
    constTypes: &'a UByte,     /* type of each constant */
    constTable: &'a ConstItem, /* the constants themselves */
}
