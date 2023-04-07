use anyhow::{anyhow, Context, Result};
use rand::Rng;
use std::{
    fs::{File, OpenOptions},
    os::unix::prelude::AsRawFd,
};

#[derive(Debug)]
pub struct HybridVsockConfig {
    /// Unique identifier of the device
    pub id: String,

    /// A 32-bit Context Identifier (CID) used to identify the guest.
    pub guest_cid: u32,

    /// unix domain socket path
    pub uds_path: String,
}

#[derive(Debug)]
pub struct VsockConfig {
    /// Unique identifier of the device
    pub id: String,

    /// A 32-bit Context Identifier (CID) used to identify the guest.
    pub guest_cid: u32,

    /// Vhost vsock fd. Hold to ensure CID is not used by other VM.
    pub vhost_fd: File,
}

pub const VHOST_VSOCK_DEVICE: &str = "/dev/vhost-vsock";

// From <linux/vhost.h>
// Generate a wrapper function for VHOST_VSOCK_SET_GUEST_CID ioctl.
// It set guest CID for vsock fd, and return error if CID is already
// in use.
const VHOST_VIRTIO_IOCTL: u8 = 0xAF;
const VHOST_VSOCK_SET_GUEST_CID: u8 = 0x60;
nix::ioctl_write_ptr!(
    vhost_vsock_set_guest_cid,
    VHOST_VIRTIO_IOCTL,
    VHOST_VSOCK_SET_GUEST_CID,
    u64
);

const CID_RETRY_COUNT: u32 = 50;

impl VsockConfig {
    pub fn new(id: String) -> Result<Self> {
        let vhost_fd = OpenOptions::new()
            .read(true)
            .write(true)
            .open(VHOST_VSOCK_DEVICE)
            .context(format!(
                "failed to open {}, try to run modprobe vhost_vsock.",
                VHOST_VSOCK_DEVICE
            ))?;
        let mut rng = rand::thread_rng();

        // Try 50 times to find a context ID that is not in use.
        for _ in 0..CID_RETRY_COUNT {
            // First usable CID above VMADDR_CID_HOST (see vsock(7))
            let first_usable_cid = 3;
            let rand_cid = rng.gen_range(first_usable_cid..=(u32::MAX));
            let guest_cid =
                unsafe { vhost_vsock_set_guest_cid(vhost_fd.as_raw_fd(), &(rand_cid as u64)) };
            match guest_cid {
                Ok(_) => {
                    return Ok(VsockConfig {
                        id,
                        guest_cid: rand_cid,
                        vhost_fd,
                    });
                }
                Err(nix::Error::EADDRINUSE) => {
                    // The CID is already in use. Try another one.
                }
                Err(err) => {
                    return Err(err).context("failed to set guest CID");
                }
            }
        }

        anyhow::bail!(
            "failed to find a free vsock context ID after {} attempts",
            CID_RETRY_COUNT
        );
    }
}

pub fn set_guest_cid_by_diffent_fd(cid: u64) -> Result<()> {
    let vhost_fd = OpenOptions::new()
        .read(true)
        .write(true)
        .open(VHOST_VSOCK_DEVICE)
        .context(format!(
            "failed to open {}, try to run modprobe vhost_vsock.",
            VHOST_VSOCK_DEVICE
        ))?;

    // reuse the VsockConfig instance's vhost_fd
    let guest_cid = unsafe { vhost_vsock_set_guest_cid(vhost_fd.as_raw_fd(), &(cid as u64)) };

    match guest_cid {
        Ok(_) => {
            println!("set guest cid: {} successful", guest_cid.unwrap());
            return Ok(());
        }
        Err(nix::Error::EADDRINUSE) => {
            // The CID is already in use. Try another one.
            // return Err(anyhow::Error("The CID is already in use. Try another one."));
            return Err(anyhow!("The CID is already in use. Try another one."));
        }
        Err(err) => {
            return Err(err).context("failed to set guest CID");
        }
    }
}

pub fn set_guest_cid_by_same_vhost_fd(v: &VsockConfig) -> Result<()> {
    // reuse the VsockConfig instance's vhost_fd
    let guest_cid =
        unsafe { vhost_vsock_set_guest_cid(v.vhost_fd.as_raw_fd(), &(v.guest_cid as u64)) };
    match guest_cid {
        Ok(_) => {
            println!("set guest cid: {} successful", guest_cid.unwrap());
            return Ok(());
        }
        Err(nix::Error::EADDRINUSE) => {
            // The CID is already in use. Try another one.
            // return Err(anyhow::Error("The CID is already in use. Try another one."));
            return Err(anyhow!("The CID is already in use. Try another one."));
        }
        Err(err) => {
            return Err(err).context("failed to set guest CID");
        }
    }
}
