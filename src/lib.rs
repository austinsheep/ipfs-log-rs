#![allow(dead_code)]

mod log;
mod gset;
mod identity;
mod lamport_clock;
mod entry;

#[allow(unused_imports)]
use std::rc::Rc;

#[allow(unused_imports)]
use std::io::Cursor;
#[allow(unused_imports)]
use ipfs_api::IpfsClient;
#[allow(unused_imports)]
use hyper::rt::Future;

#[allow(unused_imports)]
use gset::GSet;
#[allow(unused_imports)]
use lamport_clock::LamportClock;
#[allow(unused_imports)]
use identity::Identity;
#[allow(unused_imports)]
use log::Log;
#[allow(unused_imports)]
use log::AdHocAccess;
#[allow(unused_imports)]
use entry::Entry;
#[allow(unused_imports)]
use entry::EntryOrHash;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_gset () {
		let mut x: GSet<i32> = GSet::new();
		assert!(x.is_empty());
		x.insert(2);
		x.insert(3);
		x.insert(5);
		x.insert(8);
		assert!(!x.is_empty());
		assert_eq!(x.len(),4);
		let mut y: GSet<i32> = GSet::new();
		y.insert(4);
		y.insert(5);
		y.insert(10);
		y.insert(12);
		assert!(!x.is_subset(&y));
		assert!(!y.is_subset(&x));
		let z = GSet::union(&x,&y);
		assert_eq!(z.len(),7);
		let mut w = GSet::new();
		w.insert(2);
		w.insert(4);
		w.insert(8);
		assert!(w.is_subset(&z));
		assert!(!z.is_subset(&w));
	}

	#[test]
	fn test_clock () {
		let mut x = LamportClock::new("0000");
		let y = LamportClock::new("0001");
		let mut z = LamportClock::new("0002");
		assert!(x < y);
		assert!(y < z);
		z.tick();
		x.merge(&z);
		assert!(x > y);
		let w = LamportClock::new("0003").set_time(4);
		assert!(x < w);
		for _ in 0..3 {
			x.tick();
		}
		assert!(x < w);
		x.tick();
		assert!(x > w);
	}

	#[test]
	fn log_join () {
		let id = Identity::new("0","1","2","3");
		let log_id = "xyz";
		let acc = AdHocAccess;
		let mut x = Log::new(id.clone(),Some(log_id),acc,&[],&[],None,None);
		x.append("to",None);
		x.append("set",None);
		x.append("your",None);
		x.append("global",None);

		let e2 = Entry::new(id.clone(),log_id,"second",&[],None);
		let e3 = Entry::new(id.clone(),log_id,"third",&[],None);
		let e1 = Entry::new(id.clone(),log_id,"first",&[EntryOrHash::Entry(&e2),EntryOrHash::Entry(&e3)],None);
		let es = &[Rc::new(e1),Rc::new(e2),Rc::new(e3)];
		let mut y = Log::new(id.clone(),Some(log_id),acc,es,&[],None,None);
		y.append("fifth",None);
		y.append("seventh",None);

		let mut z = Log::new(id.clone(),Some(log_id),acc,es,&[],None,None);
		z.append("fourth",None);
		z.append("sixth",None);
		z.append("eighth",None);

		println!("x:\t\t{}\ny:\t\t{}\nz:\t\t{}",x.all(),y.all(),z.all());

		println!("diff z-y\t{:?}",z.diff(&y));
		y.join(&z,None);
		println!("join z+y = y\t{}\n",y.all());
		println!("----\t\ty\t\t----\n{}",y.entries());
		//println!("diff y-z\t{:?}",y.diff(&z));
		//z.join(&y,None);
		//println!("join y+z = z\t{}",z.all());
		//println!("----\t\tz\t\t----\n{}",z.entries());

		println!("diff z-y\t{:?}",z.diff(&y));
		y.join(&z,None);
		println!("join z+y = y\t{}\n",y.all());
		println!("----\t\ty\t\t----\n{}",y.entries());

		println!("y (json)\t{}",y.json());
		println!("y (snapshot)\t{}\n",y.snapshot());

		println!("diff y-x\t{:?}",y.diff(&x));
		x.join(&y,None);
		println!("join y+x = x\t{}\n",x.all());
		println!("----\t\tx\t\t----\n{}",x.entries());
	}

	#[test]
	fn ipfs () {
		let client = IpfsClient::default();
		let data = Cursor::new("tinamämmi");
		let request = client.add(data).map(|r| println!("ipfs/{}",r.hash)).map_err(|e| eprintln!("{}",e));
		hyper::rt::run(request);
	}
}
