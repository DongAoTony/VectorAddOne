use std::thread;
use std::collections::{HashMap};
use std::sync::{Arc};
use std::sync::RwLock;
fn vector_add_1(mut vector: Vec<i64>) -> Vec<i64> {
    const SPLIT_LENGTH: usize = 2;
    let mut handles = Vec::new();
    let vector_to_return2: HashMap<u32, Vec<i64>> = HashMap::new();
    let lock_1 = Arc::new(RwLock::new(vector_to_return2));

    // keep the order of the elements after handling
    let mut index1 = vector.len() / SPLIT_LENGTH;
    if vector.len() % SPLIT_LENGTH != 0 {
        index1 += 1;
    }
    while vector.len() > 0 {
        let lock = lock_1.clone();
        
        
        println!("index1: {}", index1);
        if vector.len() > SPLIT_LENGTH {
            let vector2 = vector.split_off(vector.len() - SPLIT_LENGTH);
            
            // spawn new thread to handle the sub-vector
            let handle = thread::spawn(move || {
                let mut vector_to_return: Vec<i64> = Vec::new();
                for i in vector2.iter() {
                    vector_to_return.push(i + 1);
                }
                let mut _w = lock.write().unwrap();
                (*_w).insert(index1.try_into().unwrap(), vector_to_return);
                println!("w1:{:?} {:?}", index1, *_w);
            });
            handles.push(handle);
        } else { // handle the last remaining sub-vector
            let vector2 = vector.split_off(0);
            // 启动线程来做相应的处理
            let handle = thread::spawn(move || {
                let mut vector_to_return: Vec<i64> = Vec::new();
                for i in vector2.iter() {
                    vector_to_return.push(i + 1);
                }
                let mut _w = lock.write().unwrap();
                (*_w).insert(index1.try_into().unwrap(), vector_to_return);
            });
            handles.push(handle);
        }
        index1 -= 1;
    }

    
    for handle in handles {
        handle.join().unwrap();
    }

    // keep the vetor elements sequence 
    let r = lock_1.read().unwrap();
    let mut index: u32 = 1;
    while index <= r.len().try_into().unwrap() {
        println!("index: {}", index);
        let vector4 = r.get(&index).unwrap();
        for value in vector4 {
            vector.push(*value);
        }
        
        index += 1;
    }

    vector
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let vec_return = vector_add_1(vec);
    assert_eq!(vec_return, [2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
}
