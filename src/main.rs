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
    append: bool,
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

        if append == false {
            file.set_len(0).unwrap();
        }
        drop(file);
        if i % 50 == 0 {
            info!("Iter {} end", i + 1);
        }
    }
    repo.remove_file(file_path).unwrap();
}

fn vec_duration_to_string(arr: &[Duration]) -> String {
    arr.iter()
        .map(|el| format!("{}", el.as_nanos()))
        .collect::<Vec<_>>()
        .join(" ")
}

fn write_result(
    file_id: usize,
    open: &[Duration],
    write: &[Duration],
    finish: &[Duration],
    all_time: Duration,
    open_avg: u128,
    write_avg: u128,
    finish_avg: u128,
    suffix: &str,
) {
    let all_time = all_time.as_secs_f32();

    fs::write(
        format!("./input/open_times_{}_{}.txt", suffix, file_id),
        vec_duration_to_string(&open),
    )
    .unwrap();
    fs::write(
        format!("./input/write_times_{}_{}.txt", suffix, file_id),
        vec_duration_to_string(&write),
    )
    .unwrap();
    fs::write(
        format!("./input/finish_times_{}_{}.txt", suffix, file_id),
        vec_duration_to_string(&finish),
    )
    .unwrap();
    fs::write(
        format!("./input/meta_{}_{}.txt", suffix, file_id),
        format!(
            "original\nFull time: {}\nAvg open: {}\nAvg write: {}\nAvg finish: {}\n",
            all_time, open_avg, write_avg, finish_avg
        ),
    )
    .unwrap();
}

fn benchmark(count_inner: usize, append: bool, start_idx: usize, end_idx: usize) {
    let mut open_time = vec![Duration::from_nanos(0); count_inner];
    let mut write_time = vec![Duration::from_nanos(0); count_inner];
    let mut finish_time = vec![Duration::from_nanos(0); count_inner];
    for i in start_idx..end_idx {
        let mut repo = RepoOpener::new()
            .create(true)
            .force(true)
            .open("file://./my_repo", "1234")
            .unwrap();

        let start = Instant::now();
        file_open_and_write_bench(
            &mut repo,
            &['<' as u8; 128],
            count_inner,
            &mut open_time,
            &mut write_time,
            &mut finish_time,
            false,
        );
        let end = start.elapsed();

        let open_avg = open_time.iter().sum::<Duration>().as_nanos() / count_inner as u128;
        let write_avg = write_time.iter().sum::<Duration>().as_nanos() / count_inner as u128;
        let finish_avg = finish_time.iter().sum::<Duration>().as_nanos() / count_inner as u128;

        write_result(
            i,
            &open_time,
            &write_time,
            &finish_time,
            end,
            open_avg,
            write_avg,
            finish_avg,
            if append { "append" } else { "write" },
        );

        fs::remove_dir_all("./my_repo").unwrap();
    }
}

fn main() {
    let mut logger = Builder::from_default_env();
    logger.target(Target::Stdout);
    logger.filter_level(log::LevelFilter::Info);
    logger.init();

    init_env();
    let count_inner = 15000;
    let count_outer = 8;

    match fs::remove_dir_all("./my_repo") {
        Ok(_) => {}
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {}
            _ => panic!("{}", err),
        },
    };

    match fs::create_dir("./input") {
        Ok(_) => {}
        Err(err) => match err.kind() {
            std::io::ErrorKind::AlreadyExists => {}
            _ => panic!("{}", err),
        },
    };

    let step = 4;
    for i in (0..count_outer).step_by(step) {
        benchmark(count_inner, false, i, i + 4);
        benchmark(count_inner, true, i, i + 4);
    }
}
