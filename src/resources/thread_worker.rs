use std::{array, collections::VecDeque, sync::{Arc, atomic::{AtomicBool, AtomicI32, Ordering}}, thread::JoinHandle};
use parking_lot::{Mutex, Condvar};


struct SharedState {
    pending_tasks: Mutex<VecDeque<Box<dyn FnOnce() + Send + 'static>>>,

    pair: (Mutex<()>, Condvar),
    tasks_count: AtomicI32,
    working_count: AtomicI32,
    stop_flag: AtomicBool,
}

impl SharedState {
    pub fn new() -> Self {
        Self {
            pending_tasks: Mutex::new(VecDeque::new()),

            pair: (Mutex::new(()), Condvar::new()),
            tasks_count: AtomicI32::new(0),
            working_count: AtomicI32::new(0),
            stop_flag: AtomicBool::new(false),
        }
    }
}

pub struct ThreadWorker<const COUNT: usize> {
    state: Arc<SharedState>,
    need_working_flag: bool,

    join_handlers: Option<[JoinHandle<()>; COUNT]>,
}

impl<const COUNT: usize> ThreadWorker<COUNT> {
    pub fn new() -> Self {
        Self {
            state: Arc::new(SharedState::new()),
            need_working_flag: false,

            join_handlers: None,
        }
    }

    pub fn stop(&mut self) {
        self.state.stop_flag.store(true, Ordering::Relaxed);

        self.notify();

        for join in self.join_handlers.take().unwrap() {
            join.join().expect("Error to join worker thread!");
        }
    }

    pub fn clear(&mut self) {
        self.state.pending_tasks.lock().clear();
        self.state.tasks_count.store(0, Ordering::Relaxed);

        self.notify();

        while self.state.working_count.load(Ordering::Relaxed) > 0 {}

        self.state.tasks_count.store(0, Ordering::Relaxed);
    }

    pub fn start(&mut self) {
        self.join_handlers = Some(self.thread_loop());
    }

    pub fn add_task<T: FnOnce() + Send + 'static>(&mut self, task: T) {
        self.need_working_flag = true;
        self.state.tasks_count.fetch_add(1, Ordering::Relaxed);

        self.state.pending_tasks.lock().push_back(Box::new(task));

        self.notify();
    }

    pub fn notify(&mut self) {
        let (_, cvar) = &self.state.pair;

        cvar.notify_all();
    }

    fn thread_loop(&mut self) -> [JoinHandle<()>; COUNT] {
        let mut joins: [Option<JoinHandle<()>>; COUNT] = array::from_fn(|_| None);

        for i in 0..COUNT {
            let state = self.state.clone();

            joins[i] = Some(std::thread::spawn(move || {
                'main_loop: loop {
                    let &(ref lock, ref cvar) = &state.pair;
                    let mut cond = lock.lock();
                    cvar.wait(&mut cond);

                    std::mem::drop(cond);


                    if state.stop_flag.load(Ordering::Relaxed) {
                        break;
                    }

                    state.working_count.fetch_add(1, Ordering::Relaxed);

                    while let Some(task) = { state.pending_tasks.lock().pop_front() } {
                        if state.tasks_count.load(Ordering::Relaxed) > 0 {
                            state.tasks_count.fetch_add(-1, Ordering::Relaxed);
                        }

                        if state.stop_flag.load(Ordering::Relaxed) {
                            break 'main_loop;
                        }

                        task();
                    }

                    state.working_count.fetch_add(-1, Ordering::Relaxed);
                }
            }));
        }

        return joins.map(|handler| handler.unwrap());
    }
}
