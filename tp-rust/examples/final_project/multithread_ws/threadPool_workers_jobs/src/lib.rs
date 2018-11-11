use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

// THREADPOOL
pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

// Handy Trick: take ownership of the value inside the Box<T> 
// (Rust Team are working to fix this issue)
trait FnBox {
  fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
  fn call_box(self: Box<F>) {
    (*self)()
  }
}


//struct Job;

// Job type alias for a Box that holds each closure and then sending 
// the job down the channel
type Job = Box<dyn FnBox + Send + 'static>;


impl ThreadPool {
  
 pub fn new(size: usize) -> ThreadPool {
  assert!(size > 0);

  let (sender, receiver) = mpsc::channel();

  // share ownership across multiple threads
  // Arc type will let multiple workers own the receiver
  // Mutex will ensure that only one worker gets a job from the receiver at a time
  let receiver = Arc::new(Mutex::new(receiver));

  let mut workers = Vec::with_capacity(size);
  
  for id in 0..size {
     // workers.push(Worker::new(id,receiver));
    workers.push(Worker::new(id, Arc::clone(&receiver)));
  }

  ThreadPool {
     workers,
     sender,
    }
  }

  pub fn execute<F>(&self, f: F)
    where
    F: FnOnce() + Send + 'static
  {
    let job = Box::new(f);

    self.sender.send(job).unwrap();
  }
}

// WORKERS

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
  // pass the receiving end of the channel into Worker::new, 
  // and then we use it inside the closure.
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {

    let thread = thread::spawn(move|| {
      loop{
        let job = receiver.lock().unwrap().recv().unwrap();

         println!("Worker {} got a job; executing.", id);

        //(*job)(); 
        job.call_box();
      }
    });

    Worker {
      id,
      thread,
    }
  }
}
