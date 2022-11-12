use env_logger::Env;

use prover::shared_state::SharedState;



#[tokio::main]
async fn main() {

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let shared_state = SharedState::new(SharedState::random_worker_id(), Option::Some("127.0.0.1:8888".to_string()));

    let ctx_bak = shared_state.clone();
    let mut times:u32=0;
    loop{
        times=times+1;
        let _result=ctx_bak.circuit_check().await.unwrap();
        if times>=10 {
            break;
        }
    }
    
}
