use std::env;

fn main() {
    env::set_var("TEST_FOO", "%time:~0,2%%time:~3,2%%time:~6,2%");
      
    if let Ok(pass_env) = env::var("PASS") {
        if pass_env == "true" {
            println!("cargo:rustc-cfg=feature=\"pass\"");
        }
    }
}
