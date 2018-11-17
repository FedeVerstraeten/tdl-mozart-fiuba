use std::thread;

pub struct ThreadPool {
  // ThreadPool: hold a vector of thread::JoinHandle<()>
  threads: Vec<thread::JoinHandle<()>>,
}


impl ThreadPool {
  
  /// # Constructor  
  /// We chose usize as the type, because we know that a ThreadPool
  /// negative number of threads doesn’t make any sense. 
  ///
   pub fn new(size: usize) -> ThreadPool {
  // # Panics: The `new` function will panic if the size is zero.
  //
    assert!(size > 0);


  //  initialized the vector with a capacity of size
    let mut threads = Vec::with_capacity(size);
  
  // set up a for loop that will run some code to create the threads,
    for _ in 0..size {
      // create some threads and store them in the vector
    }

  // and returned a ThreadPool instance containing them.
    ThreadPool {
      threads
    }
  }

  /// define the execute method on ThreadPool to take a closure as a parameter
  /// similar to the standard library thread::spawn implementation
  ///
  pub fn execute<F>(&self, f: F)
    where
    
    F: FnOnce() + Send + 'static
  // we need Send, to transfer the closure from one thread to another and 
  // 'static because we don’t know how long the thread will take to execute
  {
    // nothing
  }
}
