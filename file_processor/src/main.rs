use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct FileStats {
    word_count: usize,
    line_count: usize,
    char_frequencies: HashMap<char, usize>,
    size_bytes: u64,
}

#[derive(Debug, Clone)]
struct FileAnalysis {
    filename: String,
    stats: FileStats,
    processing_time: Duration,
}

// Thread Pool
type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::new();

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        self.sender.send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(_id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            while let Ok(job) = receiver.lock().unwrap().recv() {
                job();
            }
        });
        Worker { thread: Some(thread) }
    }
}

// File processing
fn find_files(dir: &PathBuf) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |e| e == "txt") {
                files.push(path);
            } else if path.is_dir() {
                files.extend(find_files(&path));
            }
        }
    }
    files
}

fn analyze_file(path: &PathBuf) -> FileAnalysis {
    let start = Instant::now();
    let filename = path.to_string_lossy().to_string();
    
    let stats = fs::File::open(path)
        .and_then(|mut file| {
            let size_bytes = file.metadata()?.len();
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            
            let line_count = contents.lines().count();
            let word_count = contents.split_whitespace().count();
            let mut char_frequencies = HashMap::new();
            for ch in contents.chars() {
                *char_frequencies.entry(ch).or_insert(0) += 1;
            }
            
            Ok(FileStats { word_count, line_count, char_frequencies, size_bytes })
        })
        .unwrap_or(FileStats {
            word_count: 0,
            line_count: 0,
            char_frequencies: HashMap::new(),
            size_bytes: 0,
        });

    FileAnalysis { filename, stats, processing_time: start.elapsed() }
}

fn main() {
    println!("\n");
    println!("------Parallel File Processor------");
    println!("\n");

    let num_threads = 4;
    let books_dir = PathBuf::from("./books");

    println!("Using {} threads", num_threads);
    println!("Directory: {}\n", books_dir.display());

    let files = find_files(&books_dir);
    let total = files.len();
    
    if total == 0 {
        println!("No .txt files found in ./books/");
        return;
    }
    
    println!("Found {} files\n", total);

    let pool = ThreadPool::new(num_threads);
    let results = Arc::new(Mutex::new(Vec::new()));
    let progress = Arc::new(Mutex::new(0));
    let start_time = Instant::now();

    println!("Processing...\n");

    for file in files {
        let results = Arc::clone(&results);
        let progress = Arc::clone(&progress);
        
        pool.execute(move || {
            let analysis = analyze_file(&file);
            results.lock().unwrap().push(analysis);
            
            let mut p = progress.lock().unwrap();
            *p += 1;
            if *p % 10 == 0 || *p == total {
                println!("Progress: {}/{}", *p, total);
            }
        });
    }

    // Wait for completion
    loop {
        if *progress.lock().unwrap() >= total {
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }

    let total_time = start_time.elapsed();
    let results = results.lock().unwrap();

    // Print results
    println!("\n========================================");
    println!("ALL DONE!");
    println!("========================================\n");

    let total_words: usize = results.iter().map(|r| r.stats.word_count).sum();
    let total_lines: usize = results.iter().map(|r| r.stats.line_count).sum();
    let total_bytes: u64 = results.iter().map(|r| r.stats.size_bytes).sum();

    println!("Files processed: {}", total);
    println!("Total words: {}", total_words);
    println!("Total lines: {}", total_lines);
    println!("Total bytes: {}", total_bytes);
    println!("Time: {:.2?}", total_time);

    if !results.is_empty() {
        let avg = results.iter().map(|r| r.processing_time.as_nanos()).sum::<u128>() 
                  / results.len() as u128;
        println!("Avg per file: {:.2?}", Duration::from_nanos(avg as u64));
    }

    // Top 10 characters
    println!("\nTop 10 Characters:");
    let mut all_chars = HashMap::new();
    for r in results.iter() {
        for (ch, count) in &r.stats.char_frequencies {
            *all_chars.entry(*ch).or_insert(0) += count;
        }
    }

    let mut chars: Vec<_> = all_chars.iter().collect();
    chars.sort_by(|a, b| b.1.cmp(a.1));

    for (i, (ch, count)) in chars.iter().take(10).enumerate() {
        let display = match ch {
            ' ' => "space".to_string(),
            '\n' => "newline".to_string(),
            '\t' => "tab".to_string(),
            _ => format!("'{}'", ch),
        };
        println!("  {}. {}: {}", i + 1, display, count);
    }

    println!("\n========================================\n");
}
//might still have soem error handling issues

/*had issues
// Compute statistics 
fn compute_stats(path: &PathBuf) -> std::io::Result<FileStats> {
    let mut file = fs::File::open(path)?;
    let metadata = file.metadata()?;
    let size_bytes = metadata.len();
    let mut contents = String::new();
    file.read_to_string(&mut contents)?
    let line_count = contents.lines().count();
    let word_count = contents.count();

    let mut char_frequencies = HashMap::new();
    for ch in contents.chars() {
        *char_frequencies.entry(ch).or_insert(0) += 1;
    }
    Ok(FileStats {
        word_count,
        line_count,
        char_frequencies,
        size_bytes,
    })
}
*/