use dlt_log::init;
use regex::Regex;
use std::process::Command;
use std::sync::{Arc, Barrier};
use std::thread;

const THREADS: usize = 8;
const MESSAGES_PER_THREAD: usize = 50;
const PAYLOAD_LEN: usize = 64;

#[test]
fn log_threads_test() {
    let app_id = "TTHR";
    let context_id = "INT";
    assert!(init(app_id, "Rust tests", context_id, "Integration tests").is_ok());

    // log from several threads at once, starting them together
    let barrier = Arc::new(Barrier::new(THREADS));
    let log_line_number = line!() + 10;
    let handles: Vec<_> = (0..THREADS)
        .map(|thread_index| {
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                // a payload unique to the thread reveals interleaved messages
                let payload = char::from(b'a' + thread_index as u8).to_string();
                let payload = payload.repeat(PAYLOAD_LEN);
                barrier.wait();
                for message_index in 0..MESSAGES_PER_THREAD {
                    log::info!("thread {thread_index} message {message_index} {payload}");
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().expect("logging thread panicked");
    }

    // get data
    let dlt_receive = Command::new("timeout")
        .arg("1")
        .arg("dlt-receive")
        .arg("-a")
        .arg("localhost")
        .output()
        .expect("failed to execute dlt-receive");

    // dlt running?
    if !String::from_utf8_lossy(&dlt_receive.stderr).contains("failed to connect") {
        let stdout = String::from_utf8_lossy(&dlt_receive.stdout);

        // 2025/12/11 17:50:20.681606   43991828 000 ECU1 TTHR INT- log info V 1 [[log_threads.rs:29] thread 3 message 7 ddd…]
        let pattern = format!(
            r"{} {}.*info.*\[\[log_threads\.rs:{}\] thread (\d+) message (\d+) ([a-z]+)\]",
            app_id, context_id, log_line_number
        );
        let regex = Regex::new(&pattern).unwrap();

        let mut received = vec![vec![0; MESSAGES_PER_THREAD]; THREADS];
        for captures in regex.captures_iter(&stdout) {
            let thread_index: usize = captures[1].parse().unwrap();
            let message_index: usize = captures[2].parse().unwrap();
            let expected_payload = char::from(b'a' + thread_index as u8)
                .to_string()
                .repeat(PAYLOAD_LEN);
            assert_eq!(
                &captures[3], expected_payload,
                "corrupted payload in message {message_index} of thread {thread_index}"
            );
            received[thread_index][message_index] += 1;
        }

        for (thread_index, counts) in received.iter().enumerate() {
            for (message_index, count) in counts.iter().enumerate() {
                assert_eq!(
                    *count, 1,
                    "message {message_index} of thread {thread_index} received {count} times"
                );
            }
        }
    }
}
