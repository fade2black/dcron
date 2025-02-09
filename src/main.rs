use dcron::get_args;
use tracing::info;

fn main() {
    tracing_subscriber::fmt::init();

    let command = get_args();
    
    info!("{:?}", command);
}
