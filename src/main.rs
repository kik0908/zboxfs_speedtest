use env_logger::{Builder, Target};
use log::{self, info};
use std::{
    fs,
    io::Write,
    time::{Duration, Instant},
};
use zbox::{init_env, OpenOptions, Repo, RepoOpener};

fn file_open_and_write_bench(
    repo: &mut Repo,
    data: &[u8],
    count: usize,
    open: &mut [Duration],
    write: &mut [Duration],
    finish: &mut [Duration],
) {
    assert!(count == open.len());
    assert!(count == write.len());
    assert!(count == finish.len());

    let file_path = "/file.txt";
    repo.create_file(file_path).unwrap();
    for i in 0..count {
        let start = Instant::now();
        let mut file = OpenOptions::new()
            .write(true)
            .open(repo, file_path)
            .unwrap();
        let end = start.elapsed();
        open[i] = end;

        let start = Instant::now();
        file.write_all(data).unwrap();
        let end = start.elapsed();
        write[i] = end;

        let start = Instant::now();
        file.finish().unwrap();
        let end = start.elapsed();
        finish[i] = end;

        file.set_len(0).unwrap();
        drop(file);
        // repo.remove_file(file_path).unwrap();
        info!("Iter {} end", i + 1);
    }
    repo.remove_file(file_path).unwrap();

}

fn main() {
    // initialise zbox environment, called first
    let mut logger = Builder::from_default_env();
    logger.target(Target::Stdout);
    logger.filter_level(log::LevelFilter::Info);
    logger.init();

    init_env();
    // create and open a repository
    let mut repo = RepoOpener::new()
        .create(true)
        .force(true)
        .open("file://./my_repo", "1234")
        .unwrap();

    let count = 4000;
    let mut open_time = vec![Duration::from_nanos(0); count];
    let mut write_time = vec![Duration::from_nanos(0); count];
    let mut finish_time = vec![Duration::from_nanos(0); count];

    let start = Instant::now();
    file_open_and_write_bench(
        &mut repo,
        &[62; 128],
        count,
        &mut open_time,
        &mut write_time,
        &mut finish_time,
    );
    let end = start.elapsed();
    println!("All 2: {}", end.as_secs_f32());
    let avg_open_time = open_time.iter().sum::<Duration>().as_nanos() / count as u128;
    println!("Open time: {avg_open_time:?}");

    let avg_write_time = write_time.iter().sum::<Duration>().as_nanos() / count as u128;
    println!("Write time: {avg_write_time}");

    let avg_finish_time = finish_time.iter().sum::<Duration>().as_nanos() / count as u128;
    println!("Finish time: {avg_finish_time}");
    println!();
    fs::write("./open_times.txt", vec_duration_to_string(&open_time)).unwrap();
    fs::write("./write_times.txt", vec_duration_to_string(&write_time)).unwrap();
    fs::write("./finish_times.txt", vec_duration_to_string(&finish_time)).unwrap();

    // std::fs::remove_dir_all("./my_repo").unwrap();
}

fn vec_duration_to_string(arr: &[Duration]) -> String {
    arr.iter()
        .map(|el| format!("{}", el.as_nanos()))
        .collect::<Vec<_>>()
        .join(" ")
}
