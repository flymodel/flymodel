#![allow(dead_code, unused)]
use std::time::Duration;

use iceoryx2::prelude::*;

pub trait AuditableEvent {}

#[derive(Debug)]
#[repr(C)]
pub struct AutitEventInner {}

fn create_service_name(service: impl ToString) -> String {
    format!(
        "flymodel.events.{service}.{ver}",
        service = service.to_string(),
        ver = env!("CARGO_PKG_VERSION")
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service_name = ServiceName::new(&create_service_name("test"))?;

    let service = zero_copy::Service::new(&service_name)
        .publish_subscribe()
        .open_or_create::<AutitEventInner>()?;

    let publisher = service.publisher().create()?;

    while let Iox2Event::Tick = Iox2::wait(Duration::from_secs(1)) {
        let sample = publisher.loan_uninit()?;

        let sample = sample.write_payload(AutitEventInner {});

        publisher.send(sample)?;
    }

    Ok(())
}
