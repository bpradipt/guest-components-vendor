// Copyright (C) 2023 Alibaba Cloud. All rights reserved.
//
// SPDX-License-Identifier: Apache-2.0

const VIRTIO_F_VERSION_1: u32 = 32;
const QUEUE_SIZE: usize = 1024;
const NUM_QUEUES: usize = 2;

type VhostUserBackendResult<T> = std::io::Result<T>;

struct VhostUserFsBackend {
    event_idx: bool,
    kill_evt: EventFd,
    mem: Option<GuestMemoryAtomic<GuestMemoryMmap>>,
    //<<<<<<<<<<<<<<<<<<<<<<<<<
    server: Arc<Server<Arc<Vfs>>>,
    //>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>.
}

impl VhostUserFsBackend {
    // There's no way to recover if error happens during processing a virtq, let the caller
    // to handle it.
    fn process_queue(&mut self, vring_state: &mut MutexGuard<VringState>) -> std::io::Result<bool> {
        let mut used_any = false;
        let guest_mem = match &self.mem {
            Some(m) => m,
            None => return Err(Error::QueueMemoryUnset.into()),
        };

        let avail_chains: Vec<DescriptorChain<GuestMemoryLoadGuard<GuestMemoryMmap>>> = vring_state
            .get_queue_mut()
            .iter(guest_mem.memory())
            .map_err(|_| Error::IterateQueue)?
            .collect();

        for chain in avail_chains {
            used_any = true;

            let head_index = chain.head_index();
            let mem = chain.memory();

            /*
            let reader = Reader::from_descriptor_chain(mem, chain.clone())
                .map_err(Error::InvalidDescriptorChain)?;
            let writer = VirtioFsWriter::new(mem, chain.clone())
                .map(|w| w.into())
                .map_err(Error::InvalidDescriptorChain)?;

            self.server
                .handle_message(
                    reader,
                    writer,
                    self.vu_req
                        .as_mut()
                        .map(|x| x as &mut dyn FsCacheReqHandler),
                    None,
                )
                .map_err(Error::ProcessQueue)?;
             */

            if self.event_idx {
                if vring_state.add_used(head_index, 0).is_err() {
                    warn!("Couldn't return used descriptors to the ring");
                }

                match vring_state.needs_notification() {
                    Err(_) => {
                        warn!("Couldn't check if queue needs to be notified");
                        vring_state.signal_used_queue().unwrap();
                    }
                    Ok(needs_notification) => {
                        if needs_notification {
                            vring_state.signal_used_queue().unwrap();
                        }
                    }
                }
            } else {
                if vring_state.add_used(head_index, 0).is_err() {
                    warn!("Couldn't return used descriptors to the ring");
                }
                vring_state.signal_used_queue().unwrap();
            }
        }

        Ok(used_any)
    }
}

