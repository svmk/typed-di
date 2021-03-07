#![feature(test)]
extern crate test;

use md5::compute as compute_md5;

use test::bench::Bencher;
use typed_di::service::service_constructor::ServiceConstructor;
use typed_di::service::service::Service;
use typed_di::service::service_id::ServiceId;

const MD5_HAHSER_SERVICE: ServiceId<Md5Hasher> = ServiceId::typename();
struct Md5Hasher {
    count: usize,
}

impl Md5Hasher {
    pub fn new(count: usize) -> Md5Hasher {
        return Md5Hasher {
            count,
        }
    }

    fn run(&self) {
        let mut value: [u8; 16] = [1; 16];
        for _ in 0..self.count {
            let digest = compute_md5(&value);
            value = digest.0;
        }
    }
}

#[bench]
fn service_call_md5_hasher_0_fn(bencher: &mut Bencher) {
    let service = Md5Hasher::new(0);
    let _ = bencher.iter(|| {
        service.run()
    });
}

#[bench]
fn service_call_md5_hasher_0_service(bencher: &mut Bencher) {
    let service = Md5Hasher::new(0);
    let service = Service::new(MD5_HAHSER_SERVICE, service);
    let _ = bencher.iter(|| {
        service.run();
    });
}


#[bench]
fn service_call_md5_hasher_1000_fn(bencher: &mut Bencher) {
    let service = Md5Hasher::new(1000);
    let _ = bencher.iter(|| {
        service.run()
    });
}

#[bench]
fn service_call_md5_hasher_1000_service(bencher: &mut Bencher) {
    let service = Md5Hasher::new(1000);
    let service = Service::new(MD5_HAHSER_SERVICE, service);
    let _ = bencher.iter(|| {
        service.run();
    });
}

#[bench]
fn service_call_md5_hasher_100_000_fn(bencher: &mut Bencher) {
    let service = Md5Hasher::new(100_000);
    let _ = bencher.iter(|| {
        service.run()
    });
}

#[bench]
fn service_call_md5_hasher_100_000_service(bencher: &mut Bencher) {
    let service = Md5Hasher::new(100_000);
    let service = Service::new(MD5_HAHSER_SERVICE, service);
    let _ = bencher.iter(|| {
        service.run();
    });
}