use std::sync::Arc;
//use std::sync::Mutex;
use std::thread;
use std::sync::mpsc::channel;
//use std::rc::Rc;

const N: usize = 10;

fn main() {

//    let movido = String::from("Serei movido");
//    let _para_ca = movido;
//    let _erro = movido;

//    let contador_ptr = Arc::new(Mutex::new(0));
    let contador_ptr = Arc::new(0);
//    let contador_ptr = Rc::new(Mutex::new(0));

    let (transmissor, receptor) = channel();

    let mut handlers = Vec::with_capacity(N);

    for id in 0..N {
//        let (contador_mutex, tx) = (contador_ptr.clone(), transmissor.clone());
        let (mut contador_ptr, tx) = (contador_ptr.clone(), transmissor.clone());

        handlers.push(thread::spawn(move || {

//            let mut contador_guard = contador_mutex.lock().unwrap();
//            *contador_guard += 1;
            *contador_ptr += 1;
            println!("Thread {} incrementou", id);

            tx.send(()).unwrap();

        }));
    }
    for _ in 0..N {
        receptor.recv().unwrap();
    }

    for handler in handlers.into_iter() {
        handler.join();
    }

//    let contador_guard = contador_ptr.lock().unwrap();
//    println!("Contador = {}", *contador_guard);
    println!("Contador = {}", *contador_ptr);

}
