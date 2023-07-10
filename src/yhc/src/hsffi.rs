use platform::platform::{UInt64};
use once_cell::sync::OnceCell;

const FFI_MAX_ARGS: u32 = 64;

/* a particular call to a FFI function */
// FFIContext<T>
pub struct FFIContext {
    next: &FFIContext, /* next ffi context in chain, used in free-lists */
    func: &FFIFunction, /* the ffi function we are calling */
    parent: &Process, /* the process that called this ffi function */
    waiters: &Process, /* list of processes waiting on this FFI call */
    argPtrs: [&UInt64; FFI_MAX_ARGS], /* pointers to all the arguments */
    argSpaces: [UInt64; FFI_MAX_ARGS], /* the space for the arguments */
    retSpace: UInt64, /* the space for returning */
}


/* the list of all free contexts */
static G_freeContexts: OnceCell<&FFIContext> = OnceCell::new();

/* the list of all waiting contexts */
static G_waitingContexts: OnceCell<&FFIContext> = OnceCell::new();

/* the FFIContext mutex, used to lock the waiting list */
static G_ffiLock: OnceCell<Mutex> = OnceCell::new();

/* the FFI waiting semaphore, used for waiting the main thread */
static G_ffiWait: OnceCell<Semaphore> = OnceCell::new();

/* the FFI context used for all 'fast' calls */
static G_fastContext: OnceCell<FFIContext> = OnceCell::new();;
