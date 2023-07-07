#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


mod platform;
mod types;

use std::{process::exit, ptr::null_mut, env::Args};
use platform::platform::{UInt, Int64};
use types::types::Bool;


// main.h

static DEFAULT_HEAP_SIZE: u32 = 16 * 1024 * 1024;
static DEFAULT_MAX_STACK_SIZE: u32 = 1 * 1024 * 1024;

struct Options {
  heapSize: UInt,
  maxStackSize: UInt,
  stats: Bool,
  pretty: Bool,
}

/* global variable storing the program name and arguments */
static mut G_argc: i32 = 0;
static mut G_argv: Args = null_mut();
static mut G_progName: &str;
static mut G_options: Options;
static mut G_yhcBasePath: &str; // Char*

static mut G_insCount: Int64;

//C #ifndef VERSION
//C # define VERSION "Unversioned"
//C #endif

static VERSION: &str = "UNVERSIONED";

fn version() {
  eprintln!("yhi: The York Haskell Interpreter version {}\n", VERSION);
}

/* give usage information and exit */
fn usage(){
  version();
//C #ifdef HAT
//C   eprintln!("\t\tCompiled with Hat support\n");
//C #endif
  eprintln!("\nusage: {} [options] classfile [args]\n", G_progName);
  eprintln!("\n");
  eprintln!("  classfile            - name of haskell class file to execute\n");
  eprintln!("  args                 - arguments to pass to haskell program\n");
  eprintln!("\n");
  eprintln!("options:\n");
  eprintln!("  -h --heap size       - set the heap size in bytes, e.g -h 10M\n");
  eprintln!("  -s --stack size      - set the maximum stack size in bytes, e.g. -s 1M\n");
  eprintln!("  -stats               - print statistics on the execution\n");
  eprintln!("  -v --version         - print the Yhi version then exit\n");
  eprintln!("  -dump                - don't execute bytecode, just print it and exit\n");
  eprintln!("\n");

  exit(1);
}

/* parse all the arguments */
fn parseArgs() -> &'static str {
  let mainMod: &str = "";
  //C int i;

  /* initialize options */
  // std::env::args().len()
  // G_progName = ;
  G_options.heapSize = DEFAULT_HEAP_SIZE;
  G_options.maxStackSize = DEFAULT_MAX_STACK_SIZE;
  G_options.stats = false;
  G_options.pretty = false;

  /* parse arguments 
  for i in 1..argc(i = 1; i < argc; i++){
    char* arg = argv[i];

    char* next = (i+1 < argc) ? argv[i+1] : NULL;

    if (*arg != '-'){
      mainMod = arg;
      i++;
      break;
    }

    if (!strcmp(arg, "-h") || !strcmp(arg, "--heap")){
      if !next {
        eprint!("ERROR: expected heap size after option {}\n", arg);
        usage();
      }
      G_options.heapSize = parseSize(arg, next);
      i++;
    } else if (!strcmp(arg, "-s") || !strcmp(arg, "--stack")){
      if (!next){
        eprint!("ERROR: expected stack size after option {}\n", arg);
        usage();
      }
      G_options.maxStackSize = parseSize(arg, next);
      i++;
    } else if (!strcmp(arg, "-stats")){
      G_options.stats = true;
    } else if (!strcmp(arg, "-dump")){
      G_options.pretty = true;
    } else if !strcmp(arg, "-v") ||
     !strcmp(arg, "-version") ||
      !strcmp(arg, "--version"){
      version();
      exit(0)

    } else {
      eprint!("WARNING: ignored unknown flag '{}'\n", arg);
    }
  } */

  /* check we have a main module */
  if !mainMod {
    usage();
  }

  /* store argument information */
  // G_argc = argc - i;
  // G_argv = &argv[i];

  /* get yhc base path */
  // G_yhcBasePath = basepath_get();

  return mainMod;
}


// main.c
fn main() {

    /* parse program arguments */
    let mainMod: &str = parseArgs();

    parseArgs()
}
