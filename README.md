# gcloud-env

Read [Google Cloud Run](https://cloud.google.com/run) runtime environment as a Rust struct.

```rust
use gcloud_env::GCloudRunEnv;

fn main() {
    let genv = GCloudRunEnv::from_env().expect("Not running from Cloud Run");
    println!("Starting with config: {:?}", genv);
    start_server(genv.port);
}
```
