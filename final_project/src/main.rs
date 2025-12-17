
use std::collections::{HashMap, VecDeque}; // Using HashMap to track cancelled tasks by ID and VecDeque as a FIFO queue
use std::panic; // Using to catch panics so a worker thread does not crash the entire program
use std::sync::{Arc, Mutex}; // Using Arc for shared ownership, and Mutex for mutual exclusion
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering}; // Using Atomic types used for shared counters and flags without locking
use std::thread; // Using to create thread
use std::time::Duration; // Using to sleep the thread

// Alias for task ID's
type TaskId = usize;


// Priority levels for tasks
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Priority {
    High,
    Medium,
    Low,
}

// More information about when task was created
struct TaskMetadata {
    created_at: u64,
}

// Represents a successful task result
struct TaskResult {
    message: String,
}

// Represents a task failure
struct TaskError {
    message: String,
}

// A task will be created and sent to the work queue
struct Task {
    id: TaskId, // Unique task ID
    priority: Priority, // Task priority level
    payload: Box<dyn TaskPayload>, // Work to execute
    metadata: TaskMetadata, // Extra task information
}

// Defines what a task must be able to do
// Send + 'static' is required to safely move tasks across threads
trait TaskPayload: Send + 'static {
    fn execute(&self) -> Result<TaskResult, TaskError>; // Runs task
    fn cancel(&self) -> Result<(), TaskError>; // Allows task to be cancelled
}


// Work queue with three levels
struct WorkQueue {
    high: VecDeque<Task>, // High Priority Tasks
    medium: VecDeque<Task>, // Medium Priority Tasks
    low: VecDeque<Task>, // Low Priority Tasks
    cancelled: HashMap<TaskId, bool>, // Tracks cancelled task IDs 
}

impl WorkQueue {
    // Create a new empty queue
    fn new() -> Self {
        Self {
            high: VecDeque::new(),
            medium: VecDeque::new(),
            low: VecDeque::new(),
            cancelled: HashMap::new(),
        }
    }

    // Submit a task to correct priority queue
    fn submit(&mut self, task: Task) {
        match task.priority {
            Priority::High => self.high.push_back(task),
            Priority::Medium => self.medium.push_back(task),
            Priority::Low => self.low.push_back(task),
        }
    }

    // Marks task as cancelled
    fn cancel(&mut self, task_id: TaskId) {
        self.cancelled.insert(task_id, true);
    }

    // Get next available task
    fn pop_next(&mut self) -> Option<Task> {
        let task = self.high.pop_front()
        .or_else(|| self.medium.pop_front())
        .or_else(|| self.low.pop_front());

        // Skip current task if it was cancelled
        if let Some(ref t) = task {
            if self.cancelled.contains_key(&t.id) {
                return None;
            }
        }
        task
    }
}


// Tracks statistics from all workers
struct Metrics {
    submitted: AtomicUsize, // Number of tasks submitted
    completed: AtomicUsize, // Number of tasks completed successfully
    errors: AtomicUsize, // Number of failed or panicked tasks
}

impl Metrics {
    fn new() -> Self {
        Self {
            submitted: AtomicUsize::new(0),
            completed: AtomicUsize::new(0),
            errors: AtomicUsize::new(0),
        }
    }
}


// main loop run by each worker thread
fn worker_loop(id: usize, queue: Arc<Mutex<WorkQueue>>, running: Arc<AtomicBool>, metrics: Arc<Metrics>) {
    while running.load(Ordering::SeqCst) {
        // Pop task from queue
        let task_opt = {
            let mut q = queue.lock().unwrap();
            q.pop_next()
        };

        if let Some(task) = task_opt {
            // Catch panics so one task does not crash the worker
            let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
                task.payload.execute()
            }));

            match result {
                Ok(Ok(_task_result)) => {
                    metrics.completed.fetch_add(1, Ordering::SeqCst);
                }
                Ok(Err(_task_error)) => {
                    metrics.errors.fetch_add(1, Ordering::SeqCst);
                }
                Err(_) => {
                    metrics.errors.fetch_add(1, Ordering::SeqCst);
                }
            }
        } else {
            // No task available, sleep for a millisecond
            thread::sleep(Duration::from_millis(1));
        }
    }

    println!("Worker {} shutting down", id);
}

// Example Task to demonstrate system
struct ExampleTask {
    number: usize,
}

impl TaskPayload for ExampleTask {
    fn execute(&self) -> Result<TaskResult, TaskError> {
        println!("Executing task {}", self.number);
        thread::sleep(Duration::from_millis(5));
        Ok(TaskResult {
            message: format!("Task {} completed", self.number),
        })
    }

    fn cancel(&self) -> Result<(), TaskError> {
        Ok(())
    }
}


fn main() {
    let queue = Arc::new(Mutex::new(WorkQueue::new())); // Shared work queue
    let running = Arc::new(AtomicBool::new(true)); // Global running flag
    let metrics = Arc::new(Metrics::new()); // Shared metrics

    let worker_count = 4;
    let mut workers = Vec::new();

    // Spawn worker threads
    for id in 0..worker_count {
        let q = Arc::clone(&queue);
        let r = Arc::clone(&running);
        let m = Arc::clone(&metrics);

        let handle = thread::spawn(move || {
            worker_loop(id, q, r, m);
        });

        workers.push(handle);
    }

    // Submit example tasks
    for i in 0..20 {
        let task = Task {
            id: i,
            priority: if i % 3 == 0 {
                Priority::High
            }
            else if i % 3 == 1 {
                Priority::Medium
            }
            else {
                Priority::Low
            },
            payload: Box::new(ExampleTask {number: i}),
            metadata: TaskMetadata {created_at: i as u64},
        };

        metrics.submitted.fetch_add(1, Ordering::SeqCst);
        queue.lock().unwrap().submit(task);
    }

    // Sleep threads to let workers process their tasks
    thread::sleep(Duration::from_secs(2));

    // Make workers stop
    running.store(false, Ordering::SeqCst);

    // Wait for workers to finish
    for handle in workers {
        let _ = handle.join();
    }

    // Print the final metrics from tasks
    println!("Submitted: {}", metrics.submitted.load(Ordering::SeqCst));
    println!("Completed: {}", metrics.completed.load(Ordering::SeqCst));
    println!("Errors: {}", metrics.errors.load(Ordering::SeqCst));
}