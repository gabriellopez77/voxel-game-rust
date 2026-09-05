use std::{array, sync::{Arc, Condvar, Mutex, atomic::{AtomicBool, Ordering}}, thread::JoinHandle};


pub struct ThreadWorker<const COUNT: usize> {
    pending_tasks: Arc<Mutex<Vec<Box<dyn FnOnce() + Send + 'static>>>>,

    pair: Arc<(Mutex<bool>, Condvar)>,
    working_flag: Arc<AtomicBool>,
    stop_flag: Arc<AtomicBool>,
    clear_flag: Arc<AtomicBool>,

    join_handler: Option<[JoinHandle<()>; COUNT]>,
}

impl<const COUNT: usize> ThreadWorker<COUNT> {
    pub fn new() -> Self {
        Self {
            pending_tasks: Arc::new(Mutex::new(Vec::new())),

            pair: Arc::new((Mutex::new(false), Condvar::new())),

            working_flag: Arc::new(AtomicBool::new(false)),
            stop_flag: Arc::new(AtomicBool::new(false)),
            clear_flag: Arc::new(AtomicBool::new(false)),

            join_handler: None,
        }
    }

    pub fn stop(&mut self) {
        self.stop_flag.store(true, Ordering::Relaxed);

        self.notify();

        for join in self.join_handler.take().unwrap() {
            join.join().expect("Error to join worker thread!");
        }
    }

    pub fn clear(&mut self) {
        self.clear_flag.store(true, Ordering::Relaxed);

        self.notify();

        while self.clear_flag.load(Ordering::Relaxed) {}
    }

    pub fn start(&mut self) {
        self.join_handler = Some(self.thread_loop());
    }

    pub fn add_task<T>(&mut self, task: T)
    where
        T: FnOnce() + Send + 'static,
    {
        self.pending_tasks.lock().unwrap().push(Box::new(task));

        self.working_flag.store(true, Ordering::Relaxed);
        self.notify();
    }

    pub fn notify(&mut self) {
        let (lock, cvar) = &*self.pair;

        {
            *lock.lock().unwrap() = true;
        }

        cvar.notify_all();
    }

    fn thread_loop(&mut self) -> [JoinHandle<()>; COUNT] {
        // SAFETY: init on loop below
        let mut joins: [Option<JoinHandle<()>>; COUNT] = array::from_fn(|_| None);

        for i in 0..COUNT {
            let pair = self.pair.clone();
            let working_flag = self.working_flag.clone();
            let stop_flag = self.stop_flag.clone();
            let clear_flag = self.clear_flag.clone();
            let pending_tasks = self.pending_tasks.clone();


            joins[i] = Some(std::thread::spawn(move || {
                let mut process_list = Vec::new();

                'main_loop: loop {
                    let (lock, cvar) = &*pair;
                    let mut mutex = lock.lock().unwrap();

                    while !*mutex {
                        mutex = cvar.wait(mutex).unwrap();
                    }

                    if clear_flag.load(Ordering::Relaxed) {
                        process_list.clear();
                        pending_tasks.lock().unwrap().clear();
                        clear_flag.store(false, Ordering::Relaxed);
                        *mutex = false;
                        working_flag.store(false, Ordering::Relaxed);
                    }

                    if stop_flag.load(Ordering::Relaxed) {
                        break;
                    }


                    {
                        let pt = &mut *pending_tasks.lock().unwrap();

                        while let Some(task) = pt.pop() {
                            process_list.push(task);
                        }
                    }

                    while let Some(task) = process_list.pop() {
                        if stop_flag.load(Ordering::Relaxed) {
                            break 'main_loop;
                        }

                        if clear_flag.load(Ordering::Relaxed) {
                            continue 'main_loop;
                        }

                        task();
                    }


                    *mutex = false;
                    working_flag.store(false, Ordering::Relaxed);
                }
            }));
        }

        return joins.map(|handler| handler.unwrap());

    }
}
