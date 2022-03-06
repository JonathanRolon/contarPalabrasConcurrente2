pub mod file_handler {

    use std::{
        collections::HashMap,
        fs::File,
        io::BufRead,
        io::BufReader,
        path::Path,
        sync::{
            mpsc::{self, Receiver, Sender},
            Arc, Mutex,
        },
        thread::{self, JoinHandle},
    };

    pub fn leer_archivo(
        filename: impl AsRef<Path>,
        children: &mut Vec<JoinHandle<()>>,
        tx: Sender<HashMap<String, u16>>,
    ) {
        let file = File::open(filename).expect("no such file");
        let reader = BufReader::new(file);
        let child = thread::spawn(move || {
            let mut ocurrencias: HashMap<String, u16> = HashMap::new();
            for line in reader.lines() {
                let line2 = line.unwrap();
                let palabras: Vec<&str> = line2.split(" ").collect();
                for palabra in palabras {
                    let pa2 = palabra.clone().to_lowercase().to_string();
                    match ocurrencias.get(&pa2) {
                        Some(cantidad) => {
                            ocurrencias.insert(pa2.clone(), *cantidad + 1);
                        }
                        None => {
                            ocurrencias.insert(pa2.clone(), 1);
                        }
                    }
                }
            }

            tx.send(ocurrencias.clone()).unwrap();
        });

        children.push(child);
    }

    pub fn contar_palabras(filenames: Vec<String>) {
        let mut ocurrencias: HashMap<String, u16> = HashMap::new();
        let mut children: Vec<JoinHandle<()>> = Vec::new();
        let (tx, rx): (Sender<HashMap<String, u16>>, Receiver<HashMap<String, u16>>) =
            mpsc::channel();

        for file in filenames.clone() {
            leer_archivo(file, &mut children, tx.clone());
        }
        for i in 0..filenames.len() {
            match rx.recv() {
                Ok(results) => {
                    for (key, value) in results {
                        match ocurrencias.get(&key) {
                            Some(cantidad) => {
                                ocurrencias.insert(key.clone(), value + cantidad);
                            }
                            None => {
                                ocurrencias.insert(key.clone(), value);
                            }
                        }
                    }
                }
                Error => {
                    println!("err");
                }
            }
        }

        // Wait for the threads to complete any remaining work
        for child in children {
            child.join().expect("oops! the child thread panicked");
        }

        println!("{:?}", ocurrencias);
    }
}
