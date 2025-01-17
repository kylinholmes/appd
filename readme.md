# Webd

`webd` is the main server, for basic function, like config log, prepare runtime, listern port, reboot appd-core, etc.

`appd-core` is the core implement of the application, it's a library(static or code) for webd.

## Run
```bash
# build in debug mode, will enable openapi with scalar
cargo run
```


## Interface
For any other module or app, it should provide a function to get the router, like:
```rust
use axum::Router;

pub fn get_api(cfg: PathBuf) -> Router 
```