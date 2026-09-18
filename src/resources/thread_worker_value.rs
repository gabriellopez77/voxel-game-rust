use std::{array, collections::VecDeque, sync::{Arc, atomic::{AtomicBool, AtomicI32, Ordering}}, thread::JoinHandle};
use parking_lot::{Mutex, Condvar};


struct SharedState<T: Send + 'static> {
    finalized_tasks: Mutex<Vec<T>>,
    pending_tasks: Mutex<VecDeque<Box<dyn FnOnce() -> T + Send + 'static>>>,

    pair: (Mutex<()>, Condvar),
    tasks_count: AtomicI32,
    working_count: AtomicI32,
    stop_flag: AtomicBool,
}

impl<T: Send + 'static> SharedState<T> {
    pub fn new() -> Self {
        Self {
            finalized_tasks: Mutex::new(Vec::new()),
            pending_tasks: Mutex::new(VecDeque::new()),

            pair: (Mutex::new(()), Condvar::new()),
            tasks_count: AtomicI32::new(0),
            working_count: AtomicI32::new(0),
            stop_flag: AtomicBool::new(false),
        }
    }
}

pub struct ThreadWorkerValue<T: Send + 'static, const COUNT: usize> {
    state: Arc<SharedState<T>>,
    need_working_flag: bool,

    join_handlers: Option<[JoinHandle<()>; COUNT]>,
}

//static TESTE: AtomicI32 = AtomicI32::new(0);

impl<T: Send + 'static, const COUNT: usize> ThreadWorkerValue<T, COUNT> {
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
            join.join().expect("Error to join worker thread! {}");
        }
    }

    pub fn clear(&mut self) {
        self.state.pending_tasks.lock().clear();
        self.state.tasks_count.store(0, Ordering::Relaxed);

        self.notify();

        while self.state.working_count.load(Ordering::Relaxed) > 0 {}

        self.state.finalized_tasks.lock().clear();
        self.state.tasks_count.store(0, Ordering::Relaxed);
    }

    pub fn start(&mut self) {
        self.join_handlers = Some(self.thread_loop());
    }

    pub fn get_finalized_task(&mut self) -> Option<T> {
        return self.state.finalized_tasks.lock().pop();
    }

    pub fn add_task<T2: FnOnce() -> T + Send + 'static>(&mut self, task: T2) {
        self.need_working_flag = true;
        self.state.tasks_count.fetch_add(1, Ordering::Relaxed);

        self.state.pending_tasks.lock().push_back(Box::new(task));
    }

    pub fn process_tasks(&mut self) {
        if !self.need_working_flag {
            return;
        }

        // some thread has working, then we do not need notify it again
        if self.state.working_count.load(Ordering::Relaxed) > 0 {
            return;
        }

        self.need_working_flag = false;

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
                //let id = TESTE.fetch_add(1, Ordering::Relaxed);
                //let mut count = 0;

                'main_loop: loop {
                    //while !(state.tasks_count.load(Ordering::Relaxed) > 0 || state.stop_flag.load(Ordering::Relaxed)) {
                        let (lock, cvar) = &state.pair;
                        let mut cond = lock.lock();
                        cvar.wait(&mut cond);
                        std::mem::drop(cond);
                        //}

                    if state.stop_flag.load(Ordering::Relaxed) {
                        break;
                    }

                    state.working_count.fetch_add(1, Ordering::Relaxed);

                    //println!("Inicio: {}", id);


                    while let Some(task) = { state.pending_tasks.lock().pop_front() } {
                        if state.tasks_count.load(Ordering::Relaxed) > 0 {
                            state.tasks_count.fetch_add(-1, Ordering::Relaxed);
                        }

                        if state.stop_flag.load(Ordering::Relaxed) {
                            break 'main_loop;
                        }

                        //count += 1;
                        let result = task();
                        state.finalized_tasks.lock().push(result);

                    }

                    state.working_count.fetch_add(-1, Ordering::Relaxed);

                    //println!("Fim: {}, count: {}", id, count);
                    //count = 0;
                }
            }));
        }

        return joins.map(|handler| handler.unwrap());
    }
}
