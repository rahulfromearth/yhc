
use process::proc_alloc;

/* initialize the mutator */
fn mut_init(){
  G_proc = proc_alloc();

  G_spBase = (Node**)&G_proc.stack.data[G_proc.stack.size];
  G_spLimit = (Node**)G_proc.stack.data;
  G_sp = G_spBase;
  G_insBeforeSwitch = INS_PER_THREAD;
}
