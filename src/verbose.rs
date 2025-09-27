
use std::sync::atomic::{AtomicBool, Ordering};

// global flag
pub static IS_VERBOSE: AtomicBool = AtomicBool::new(false);

// set from anywhere
pub fn set_verbose(on: bool)
{
   IS_VERBOSE.store(on, Ordering::Relaxed);
}

// read from anywhere
pub fn is_verbose() -> bool
{
   IS_VERBOSE.load(Ordering::Relaxed)
}



pub fn verbose_msg(msg: &str)
{
   if!is_verbose()
   {
      return;
   }

   println!("[VERBOSE] {}", msg);
}



pub fn verbose_value<T: std::fmt::Debug>(label: &str, val: T)
{
   if!is_verbose()
   {
      return;
   }

   println!("[VERBOSE] {} = {:?}", label, val);
}