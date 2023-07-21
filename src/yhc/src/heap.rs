use std::sync::Arc;

use crate::{
    node::{CodePtr, Node},
    platform::platform::{UInt, Word},
    types::types::Bool,
};

/* a frame is the part of the stack that records successive applications,
it is stored as a structure making additions and the like very easy */
pub struct Frame<'a> {
    fp: Arc<&'a Frame<'a>>,
    ip: CodePtr<'a>,
    vapptr: &'a Node,
}

/* 'global variable's are used to hold pointers from C into the Haskell heap.
specifically these are organised as a stack. Generally used to hold values inside
primitive functions */
pub struct Global<'a> {
    next: Arc<&'a Global<'a>>,
    global: &'a &'a Node,
}

// heap.c

static mut G_markTableSize: UInt = 0;
static mut G_markTable: Option<Vec<Word>> = None;

static mut G_gcEnabled: Bool = false;
static mut G_hpSize: UInt = 0;
static mut G_hp: Option<Vec<Word>> = None;
static mut G_hpStart: Option<&Word> = None;
static mut G_hpEnd: Option<&Word> = None;

static mut G_reserve: UInt = 0;
static mut G_sp: Option<&&Node> = None;
static mut G_spBase: Option<&&Node> = None;
static mut G_spLimit: Option<&&Node> = None;
static mut G_fp: Option<&Frame> = None;

pub fn heap_init(heapSize: UInt){
  let wSize  = heapSize / sizeof(Word);

  unsafe { 
    G_hpSize = wSize;
    G_markTableSize = (wSize / WORD_BITS) + match (wSize % WORD_BITS) == 0 {
        true => 0,
        false => 1
      };

      G_markTable = Some(Vec::with_capacity(unsafe { G_markTableSize.try_into().unwrap() }));

      G_hp = Some(Vec::with_capacity(wSize.try_into().unwrap()));

      G_hpStart = G_hp.;
      G_hpEnd = G_hpStart + wSize;
    
    // G_spBase = G_spLimit = G_sp = G_fp = NULL;
    
    }

}
