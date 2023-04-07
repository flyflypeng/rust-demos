mod vsock;
use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let vsock_instance = vsock::VsockConfig::new("vsock-111".to_string())?;
    println!("vsock instance info: {:?}", vsock_instance);

    // reuse the same vsock instance vhost_fd to set guest id by ioctl
    println!("set_guest_cid_by_same_vhost_fd result: ");
    vsock::set_guest_cid_by_same_vhost_fd(&vsock_instance)?;

    // use the different vhost_fd to set the same guest id by ioctl
    println!("set_guest_cid_by_diffent_fd result: ");
    vsock::set_guest_cid_by_diffent_fd(vsock_instance.guest_cid as u64)?;

    Ok(())
}
