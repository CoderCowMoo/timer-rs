// A regex to match a valid token of time expression in the input is this.
// [0-9]+[ ]*[hms]*(inutes|econds|ours|our|econd|inute|ins|ecs)*
// This regex matches the following strings:
// 10m20s (splits up into 10m and 20s)
// 10minutes is allowed.

fn main() {
    println!("Hello, world!");
}
