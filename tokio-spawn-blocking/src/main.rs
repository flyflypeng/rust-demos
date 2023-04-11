use anyhow::anyhow;
use anyhow::Result;
use std::thread::sleep;
use std::time;

#[tokio::main]
async fn main() -> Result<()> {
    tokio::task::spawn_blocking(move || -> Result<()> {
        let mut cmd = std::process::Command::new("sleep");
        cmd.args(&["30"]);
        let mut child = cmd
            .spawn()
            .map_err(|e| anyhow!("failed to spawn sleep command: {}", e))?;
        println!("new sleep process starting, process id: {}", child.id());
        match child.wait() {
            Ok(r) => {
                if !r.success() {
                    return Err(
                        anyhow!("sleep command execute failed, exit with status: {:?}", r).into(),
                    );
                }
                Ok(())
            }
            Err(e) => Err(anyhow!("failed to execute sleep command with error {}", e).into()),
        }
    })
    .await // block until the closure in the `spawn_blocking` is executed
    .map_err(|e| anyhow!("failed to join the sleep process thread {}", e))??;

    println!("start sleep in the main() func");
    let sleep_time = time::Duration::from_secs(10);
    sleep(sleep_time);
    Ok(())
}
