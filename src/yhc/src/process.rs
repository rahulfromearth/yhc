use node::{ExceptionHandlerNode, Node, NodeHeader};
use platform::platform::{HUInt, UInt, Word};
use types::types::Bool;
use hsffi::FFIContext;

/* the id of a process */
type ProcessID = UInt;

/* default process stack size, in words */
const DEFAULT_STACK_SIZE: u32 = 64;

/* the number of instructions to run per thread */
const INS_PER_THREAD: u32 = 10000;

/* process mode
    DEAD     - process is dead, can be recycled as a new one
    RUNNING  - process is currently running
    READY    - process is ready to run but is currently not scheduled
    BLOCKED  - process is unable to run because it's waiting on blackhole/mvar
    WAITING  - process is unable to run because it's waiting on an FFI action
    THROWING - process is unable to run because it's waiting to throw an exception
*/
enum PMode {
    PM_DEAD,
    PM_RUNNING,
    PM_READY,
    PM_BLOCKED,
    PM_WAITING,
    PM_THROWING,
}

/* blocked mode
BM_HOLE - waiting on a blackhole
BM_MVAR - waiting on an mvar
BM_EXCEP - waiting for exceptions to unblock */
enum BMode {
    BM_HOLE,
    BM_MVAR,
    BM_EXCEP,
}

/* a process stack node */
struct ProcStackNode {
    node: NodeHeader,
    exceptionStack: &ExceptionHandlerNode, /* exception stack for this process */
    parent: &Process,                      /* the process that owns us */
    size: UInt,                            /* the size of the stack */
    data: Vec<Word>,                       /* the stack data */
}

/* a process, is also a TInfo (used in blackhole-blocking) */
struct Process {
    info: Info,         /* because it's also a TInfo */
    pmode: HUInt,       /* current process mode */
    linkNext: &Process, /* next process in the list of all processes */
    linkPrev: &Process, /* prev process in the list of all processes */

    next: &Process,        /* next in chain, use varies depending on process status */
    id: ProcessID,         /* the unique id of the process */
    saveFP: &Frame,        /* the saved frame pointer */
    stack: &ProcStackNode, /* pointer to the stack node for the process */
    blockedOn: &Node,      /* node this process is waiting on */
    blockedMode: BMode,    /* what type of thing 'blockedOn' is */
    waitContext: &FFIContext, /* ffi context that we are waiting on */
    isInterruptible: Bool, /* if true then this thread will accept throwTos even when blocked */
}



/* list of free processes */
static mut G_freeProcList: &Process = unsafe {mem::uninitialized()};

/* list of all processes */
G_procList: &Process = NULL;

/* the current process */
G_proc: &Process = NULL;

/* the current process ID */
static mut G_nextID: ProcessID = 0;

/* the number of instructions before a swith */
G_insBeforeSwitch: Int = 0;

/* whether process switching is disabled */
G_procSwitchDisabled: Bool = false;

/* the list of processes that are ready */
static G_firstReady: &Process = NULL;
static G_lastReady: &Process = NULL;

/* the list of processes blocked waiting for exceptions to become available */
static G_excepBlockList: &Process = NULL;
G_excepBlocked: Bool = false;

/* return whether this is the only process */
// fn proc_isOnlyProcess() -> Bool {
//   assert(G_procList != NULL);
//   return G_procList.linkNext == NULL;
// }


/* allocate a new process */
// pub fn proc_alloc() -> &Process{
//    let mut proc: &Process;
//   let mut stack: &ProcStackNode;

//   /* get a new process */
//   if (G_freeProcList != NULL){
//     proc = G_freeProcList;
//     G_freeProcList = proc->next;
//   }else{
//     proc = (Process*)malloc(sizeof(Process));
//   }
//   /* allocate a stack node */
//   stack = (ProcStackNode*)heap_alloc(wordsof(ProcStackNode) + DEFAULT_STACK_SIZE);
//   MAKE_NODE(stack, &G_infoProcStack, N_NORMAL);
//   stack->size = DEFAULT_STACK_SIZE;
//   stack->parent = proc;
//   stack->exceptionStack = (ExceptionHandlerNode*)G_nodeUnit; /* keeps the GC much happier than using NULL */
//   /* initialize it */
//   proc->info.tag = I_TINFO;
//   proc->id = G_nextID++;
//   proc->pmode = PM_READY;
//   proc->stack = stack;
//   proc->blockedOn = G_nodeUnit;
//   proc->waitContext = NULL;
//   proc->saveFP = NULL;
//   proc->isInterruptible = false;

//   /* add to process list */
//   proc->linkPrev = NULL;
//   proc->linkNext = G_procList;
//   if (G_procList){
//     G_procList->linkPrev = proc;
//   }
//   G_procList = proc;

//   /* setup counter */
//   return proc;
// }
