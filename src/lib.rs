//! JAM SDK empty service

#![cfg_attr(any(target_arch = "riscv32", target_arch = "riscv64"), no_std)]
#![allow(clippy::unwrap_used)]

extern crate alloc;

use alloc::string::ToString;
use jam_pvm_common::*;
use jam_types::*;

pub struct Service;
jam_pvm_common::declare_service!(Service);

impl jam_pvm_common::Service for Service {
    fn refine(
        _core_index: CoreIndex,
        _item_index: usize,
        service_id: ServiceId,
        _payload: WorkPayload,
        _package_hash: WorkPackageHash,
    ) -> WorkOutput {
        info!("Empty Service Refine, {service_id:x}h");
        alloc::vec![].into()
    }

    fn accumulate(now: Slot, id: ServiceId, _item_count: usize) -> Option<Hash> {
        info!(
            "Empty Service Accumulate, {id:x}h @{now} ${}",
            accumulate::my_info().balance
        );
        for item in accumulate::accumulate_items() {
            match item {
                AccumulateItem::WorkItem(r) => on_work_item(r, now, id),
                AccumulateItem::Transfer(t) => on_transfer(t),
            }
        }
        None
    }
}

fn on_work_item(item: WorkItemRecord, now: Slot, service_id: ServiceId) {
    match item.result {
        Ok(data) => {
            info!(
                "Got a work item {service_id}@{now}: {}",
                alloc::string::String::from_utf8(data.to_vec())
                    .unwrap_or_else(|_| "???".to_string())
            );
        }
        Err(err) => {
            info!("Invalid work item: {err:?}")
        }
    }
}

fn on_transfer(item: TransferRecord) {
    let TransferRecord {
        source,
        amount,
        memo,
        ..
    } = item;
    info!(
        "Received transfer from {source} of {amount} with memo {}",
        alloc::string::String::from_utf8(memo.to_vec()).unwrap_or_else(|_| "???".to_string())
    );
}

pub const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
