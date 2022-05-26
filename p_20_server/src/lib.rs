use std::thread;
use std::sync::mpsc;
use std::sync::{Arc,Mutex};
pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

// struct Job;
type Job = Box<dyn FnBox + Send + 'static>;
enum Message {
    NewJob(Job),
    Terminate,
}


impl ThreadPool {
    pub fn new(size:usize) -> ThreadPool{
        // size 는 풀 안의 스레드 개수입니다.
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        // let r = Arc::clone(&receiver);

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // 스레드들을 생성하고 벡터 내에 보관합니다
            // workers[id] = Worker::new(id,Arc::clone(&receiver)); 
            // 왜 이거 컴파일 오류X, 실행시 오류...
            // thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0'
            workers.push(Worker::new(id,Arc::clone(&receiver)));

        }

        ThreadPool {
            workers,
            sender
        }

    }
        
    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static
    // 스레드가 언제 파괴될지 모르기 때문에 'static' 이 필요합
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();

    }
}

type Receiver = Arc<Mutex<mpsc::Receiver<Message>>>;

struct Worker {
    id: usize,
    thread:Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id:usize, receiver:Receiver)->Worker{
        let thread = Option::Some(thread::spawn(move || {
            loop {
                let message  = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        //(*job)();
                        job.call_box();
                        //러스트 컴파일러는 아직 완벽하지 않다...네?
                        //fnbox 트레잇을 만드오 해결
                        //이 트릭은 매우 복잡하고 교활합니다
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    },

                }

                

            }
        }));

        Worker { 
            id, 
            thread,
        }
    }
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}


impl Drop for ThreadPool {
    fn drop(&mut self) {

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers.");


        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            
            //종료 메세지 전송

            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            } 
        }
    }
}