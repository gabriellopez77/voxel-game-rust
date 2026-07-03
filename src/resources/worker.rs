use std::{sync::{Arc, Condvar, Mutex, atomic::{AtomicBool, Ordering}}, thread::JoinHandle};


pub struct Worker<T: Send + 'static> {
    finalized_tasks: Arc<Mutex<Vec<T>>>,
    pending_tasks: Arc<Mutex<Vec<Box<dyn FnOnce() -> T + Send + 'static>>>>,

    pair: Arc<(Mutex<bool>, Condvar)>,

    need_working_flag: bool,
    working_flag: Arc<AtomicBool>,
    stop_flag: Arc<AtomicBool>,
    clear_flag: Arc<AtomicBool>,

    join_handler: Option<JoinHandle<()>>,
}

impl<T: Send + 'static> Worker<T> {
    pub fn new() -> Self {
        Self {
            finalized_tasks: Arc::new(Mutex::new(Vec::new())),
            pending_tasks: Arc::new(Mutex::new(Vec::new())),

            pair: Arc::new((Mutex::new(false), Condvar::new())),

            need_working_flag: false,
            working_flag: Arc::new(AtomicBool::new(false)),
            stop_flag: Arc::new(AtomicBool::new(false)),
            clear_flag: Arc::new(AtomicBool::new(false)),

            join_handler: None,
        }
    }

    pub fn stop(&mut self) {
        self.stop_flag.store(true, Ordering::Relaxed);

        self.notify();
        self.join_handler.take().unwrap().join().expect("Error to join worker thread!");
    }

    pub fn clear(&mut self) {
        self.clear_flag.store(true, Ordering::Relaxed);

        self.notify();

        while self.clear_flag.load(Ordering::Relaxed) {}
    }

    pub fn start(&mut self) {
        self.join_handler = Some(self.thread_loop());
    }

    pub fn get_finalized_task(&mut self) -> Option<T> {
        return self.finalized_tasks.lock().unwrap().pop();
    }

    pub fn add_task<T2>(&mut self, task: T2)
    where
        T2: FnOnce() -> T + Send + 'static,
    {
        self.need_working_flag = true;

        self.pending_tasks.lock().unwrap().push(Box::new(task));
    }

    pub fn process_tasks(&mut self) {
        if self.working_flag.load(Ordering::Relaxed) || !self.need_working_flag {
            return;
        }

        self.working_flag.store(true, Ordering::Relaxed);
        self.need_working_flag = false;

        self.notify();
    }

    pub fn notify(&mut self) {
        let (lock, cvar) = &*self.pair;

        {
            *lock.lock().unwrap() = true;
        }

        cvar.notify_one();
    }

    fn thread_loop(&mut self) -> JoinHandle<()> {
        let pair = self.pair.clone();
        let working_flag = self.working_flag.clone();
        let stop_flag = self.stop_flag.clone();
        let clear_flag = self.clear_flag.clone();
        let pending_tasks = self.pending_tasks.clone();
        let finalized_tasks = self.finalized_tasks.clone();

        return std::thread::spawn(move || {
            let mut process_list = Vec::new();
            //let mut processed_list = Vec::new();

            loop {
                let (lock, cvar) = &*pair;
                let mut mutex = lock.lock().unwrap();

                while !*mutex {
                    mutex = cvar.wait(mutex).unwrap();
                }

                if clear_flag.load(Ordering::Relaxed) {
                    process_list.clear();
                    pending_tasks.lock().unwrap().clear();
                    finalized_tasks.lock().unwrap().clear();
                    clear_flag.store(false, Ordering::Relaxed);
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
                    let result = task();

                    //processed_list.push(result);
                    finalized_tasks.lock().unwrap().push(result);

                }

                //{
                //    let pl = &mut *finalized_tasks.lock().unwrap();

                //    while let Some(result) = processed_list.pop() {
                //        pl.push(result);
                //    }
                //}

                *mutex = false;
                working_flag.store(false, Ordering::Relaxed);
            }
        });
    }
}
