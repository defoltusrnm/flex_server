mod app_logging;
mod networking;
mod server_errors;

use std::{os::windows::io::AsRawHandle, time::Duration};

use app_logging::logger_cfg::configure_logs;
use dotenv::dotenv;
use log::LevelFilter;

#[tokio::main]
async fn main() {
    configure_logs(LevelFilter::Trace).unwrap();

    dotenv().unwrap();
    log::trace!(".env loaded");
    test_1();
}

fn test_1() {
    let f1 = |inner_handler: Box<dyn Fn(&mut Vec<i32>)>| {
        let mut vec = vec![0; 5];

        vec.iter()
            .inspect(|x| log::info!("{x}"))
            .collect::<Vec<_>>();

        inner_handler(&mut vec);
        inner_handler(&mut vec);
        inner_handler(&mut vec);

        vec.iter()
            .inspect(|x| log::info!("{x}"))
            .collect::<Vec<_>>();
    };

    let f2 = |x: &mut Vec<i32>| {
        x.push(1);
        x.push(1);
        x.push(1);
        x.push(1);
    };

    f1(Box::new(f2));
}

async fn test_2() {
    parent_func(child_func).await;
}

async fn child_func(x: &mut Vec<i32>) {
    tokio::time::sleep(Duration::from_millis(100)).await;
    x.push(1);
    x.push(1);
    x.push(1);
    x.push(1);
    tokio::time::sleep(Duration::from_millis(100)).await;
}

async fn parent_func<F, FOut>(inner_handler: F)
where
    F: for<'a> Fn(&'a mut Vec<i32>) -> FOut,
    FOut: Future<Output = ()>,
{
    let mut vec = vec![0; 5];

    vec.iter()
        .inspect(|x| log::info!("{x}"))
        .collect::<Vec<_>>();

    inner_handler(&mut vec).await;
    inner_handler(&mut vec).await;
    inner_handler(&mut vec).await;

    vec.iter()
        .inspect(|x| log::info!("{x}"))
        .collect::<Vec<_>>();
}
