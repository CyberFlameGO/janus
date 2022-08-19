//! Provides a testcontainer image to allow container-based testing of Janus.

use crate::load_zstd_compressed_docker_image;
use std::sync::Mutex;
use testcontainers::{core::WaitFor, Image};

// interop_aggregator.tar.zst is created by this package's build.rs.
const INTEROP_AGGREGATOR_IMAGE_BYTES: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/interop_aggregator.tar.zst"));
static INTEROP_AGGREGATOR_IMAGE_HASH: Mutex<Option<String>> = Mutex::new(None);

/// Represents a Janus Aggregator as a testcontainer image.
#[non_exhaustive]
pub struct Aggregator {}

impl Default for Aggregator {
    fn default() -> Self {
        // One-time initialization step: load compiled image into docker, recording its image tag,
        // so that we can launch it later.
        let mut image_hash = INTEROP_AGGREGATOR_IMAGE_HASH.lock().unwrap();
        if image_hash.is_none() {
            *image_hash = Some(load_zstd_compressed_docker_image(
                INTEROP_AGGREGATOR_IMAGE_BYTES,
            ));
        }

        Self {}
    }
}

impl Image for Aggregator {
    type Args = ();

    fn name(&self) -> String {
        // This works around a quirk in testcontainers: it will always generate the image name
        // it passes to Docker as "$NAME:$TAG". We want a string of the form "sha256:$HASH". So we
        // hardcode the name to be "sha256" and the tag to be the hash we want.
        "sha256".to_string()
    }

    fn tag(&self) -> String {
        INTEROP_AGGREGATOR_IMAGE_HASH
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .clone()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        Vec::new()
    }
}