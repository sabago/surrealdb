use super::kv::Add;
use super::kv::Convert;
use super::Key;
use super::Val;
use crate::err::Error;
use crate::key::thing;
use crate::kvs::cache::Cache;
use crate::kvs::cache::Entry;
use crate::sql;
use crate::sql::thing::Thing;
use channel::Sender;
use sql::permission::Permissions;
use sql::statements::DefineDatabaseStatement;
use sql::statements::DefineEventStatement;
use sql::statements::DefineFieldStatement;
use sql::statements::DefineIndexStatement;
use sql::statements::DefineLoginStatement;
use sql::statements::DefineNamespaceStatement;
use sql::statements::DefineScopeStatement;
use sql::statements::DefineTableStatement;
use sql::statements::DefineTokenStatement;
use sql::statements::LiveStatement;
use std::ops::Range;
use std::sync::Arc;

/// A set of undoable updates and requests against a dataset.
pub struct Transaction {
	pub(super) inner: Inner,
	pub(super) cache: Cache,
}

#[allow(clippy::large_enum_variant)]
pub(super) enum Inner {
	#[cfg(feature = "kv-echodb")]
	Mem(super::mem::Transaction),
	#[cfg(feature = "kv-indxdb")]
	IxDB(super::ixdb::Transaction),
	#[cfg(feature = "kv-yokudb")]
	File(super::file::Transaction),
	#[cfg(feature = "kv-tikv")]
	TiKV(super::tikv::Transaction),
	#[cfg(feature = "kv-fdb")]
	FDB(super::fdb::Transaction),
}

impl Transaction {
	/// Check if transactions is finished.
	///
	/// If the transaction has been cancelled or committed,
	/// then this function will return [`true`], and any further
	/// calls to functions on this transaction will result
	/// in a [`Error::TxFinished`] error.
	pub async fn closed(&self) -> bool {
		match self {
			#[cfg(feature = "kv-echodb")]
			Transaction {
				inner: Inner::Mem(v),
				..
			} => v.closed(),
			#[cfg(feature = "kv-yokudb")]
			Transaction {
				inner: Inner::File(v),
				..
			} => v.closed(),
			#[cfg(feature = "kv-indxdb")]
			Transaction {
				inner: Inner::IxDB(v),
				..
			} => v.closed(),
			#[cfg(feature = "kv-tikv")]
			Transaction {
				inner: Inner::TiKV(v),
				..
			} => v.closed(),
			#[cfg(feature = "kv-fdb")]
			Transaction {
				inner: Inner::FDB(v),
				..
			} => v.closed(),
		}
	}
	/// Cancel a transaction.
	///
	/// This reverses all changes made within the transaction.
	pub async fn cancel(&mut self) -> Result<(), Error> {
		match self {
			#[cfg(feature = "kv-echodb")]
			Transaction {
				inner: Inner::Mem(v),
				..
			} => v.cancel(),
			#[cfg(feature = "kv-yokudb")]
			Transaction {
				inner: Inner::File(v),
				..
			} => v.cancel(),
			#[cfg(feature = "kv-indxdb")]
			Transaction {
				inner: Inner::IxDB(v),
				..
			} => v.cancel().await,
			#[cfg(feature = "kv-tikv")]
			Transaction {
				inner: Inner::TiKV(v),
				..
			} => v.cancel().await,
			#[cfg(feature = "kv-fdb")]
			Transaction {
				inner: Inner::FDB(v),
				..
			} => v.cancel().await,
		}
	}
	/// Commit a transaction.
	///
	/// This attempts to commit all changes made within the transaction.
	pub async fn commit(&mut self) -> Result<(), Error> {
		match self {
			#[cfg(feature = "kv-echodb")]
			Transaction {
				inner: Inner::Mem(v),
				..
			} => v.commit(),
			#[cfg(feature = "kv-yokudb")]
			Transaction {
				inner: Inner::File(v),
				..
			} => v.commit(),
			#[cfg(feature = "kv-indxdb")]
			Transaction {
				inner: Inner::IxDB(v),
				..
			} => v.commit().await,
			#[cfg(feature = "kv-tikv")]
			Transaction {
				inner: Inner::TiKV(v),
				..
			} => v.commit().await,
			#[cfg(feature = "kv-fdb")]
			Transaction {
				inner: Inner::FDB(v),
				..
			} => v.commit().await,
		}
	}
	/// Delete a key from the datastore.
	pub async fn del<K>(&mut self, key: K) -> Result<(), Error>
	where
		K: Into<Key>,
	{
		match self {
			#[cfg(feature = "kv-echodb")]
			Transaction {
				inner: Inner::Mem(v),
				..
			} => v.del(key),
			#[cfg(feature = "kv-yokudb")]
			Transaction {
				inner: Inner::File(v),
				..
			} => v.del(key),
			#[cfg(feature = "kv-indxdb")]
			Transaction {
				inner: Inner::IxDB(v),
				..
			} => v.del(key).await,
			#[cfg(feature = "kv-tikv")]
			Transaction {
				inner: Inner::TiKV(v),
				..
			} => v.del(key).await,
			#[cfg(feature = "kv-fdb")]
			Transaction {
				inner: Inner::FDB(v),
				..
			} => v.del(key).await,
		}
	}
	/// Check if a key exists in the datastore.
	pub async fn exi<K>(&mut self, key: K) -> Result<bool, Error>
	where
		K: Into<Key>,
	{
		match self {
			#[cfg(feature = "kv-echodb")]
			Transaction {
				inner: Inner::Mem(v),
				..
			} => v.exi(key),
			#[cfg(feature = "kv-yokudb")]
			Transaction {
				inner: Inner::File(v),
				..
			} => v.exi(key),
			#[cfg(feature = "kv-indxdb")]
			Transaction {
				inner: Inner::IxDB(v),
				..
			} => v.exi(key).await,
			#[cfg(feature = "kv-tikv")]
			Transaction {
				inner: Inner::TiKV(v),
				..
			} => v.exi(key).await,
			#[cfg(feature = "kv-fdb")]
			Transaction {
				inner: Inner::FDB(v),
				..
			} => v.exi(key).await,
		}
	}
	/// Fetch a key from the datastore.
	pub async fn get<K>(&mut self, key: K) -> Result<Option<Val>, Error>
	where
		K: Into<Key>,
	{
		match self {
			#[cfg(feature = "kv-echodb")]
			Transaction {
				inner: Inner::Mem(v),
				..
			} => v.get(key),
			#[cfg(feature = "kv-yokudb")]
			Transaction {
				inner: Inner::File(v),
				..
			} => v.get(key),
			#[cfg(feature = "kv-indxdb")]
			Transaction {
				inner: Inner::IxDB(v),
				..
			} => v.get(key).await,
			#[cfg(feature = "kv-tikv")]
			Transaction {
				inner: Inner::TiKV(v),
				..
			} => v.get(key).await,
			#[cfg(feature = "kv-fdb")]
			Transaction {
				inner: Inner::FDB(v),
				..
			} => v.get(key).await,
		}
	}
	/// Insert or update a key in the datastore.
	pub async fn set<K, V>(&mut self, key: K, val: V) -> Result<(), Error>
	where
		K: Into<Key>,
		V: Into<Val>,
	{
		match self {
			#[cfg(feature = "kv-echodb")]
			Transaction {
				inner: Inner::Mem(v),
				..
			} => v.set(key, val),
			#[cfg(feature = "kv-yokudb")]
			Transaction {
				inner: Inner::File(v),
				..
			} => v.set(key, val),
			#[cfg(feature = "kv-indxdb")]
			Transaction {
				inner: Inner::IxDB(v),
				..
			} => v.set(key, val).await,
			#[cfg(feature = "kv-tikv")]
			Transaction {
				inner: Inner::TiKV(v),
				..
			} => v.set(key, val).await,
			#[cfg(feature = "kv-fdb")]
			Transaction {
				inner: Inner::FDB(v),
				..
			} => v.set(key, val).await,
		}
	}
	/// Insert a key if it doesn't exist in the datastore.
	pub async fn put<K, V>(&mut self, key: K, val: V) -> Result<(), Error>
	where
		K: Into<Key>,
		V: Into<Val>,
	{
		match self {
			#[cfg(feature = "kv-echodb")]
			Transaction {
				inner: Inner::Mem(v),
				..
			} => v.put(key, val),
			#[cfg(feature = "kv-yokudb")]
			Transaction {
				inner: Inner::File(v),
				..
			} => v.put(key, val),
			#[cfg(feature = "kv-indxdb")]
			Transaction {
				inner: Inner::IxDB(v),
				..
			} => v.put(key, val).await,
			#[cfg(feature = "kv-tikv")]
			Transaction {
				inner: Inner::TiKV(v),
				..
			} => v.put(key, val).await,
			#[cfg(feature = "kv-fdb")]
			Transaction {
				inner: Inner::FDB(v),
				..
			} => v.put(key, val).await,
		}
	}
	/// Retrieve a specific range of keys from the datastore.
	///
	/// This function fetches the full range of key-value pairs, in a single request to the underlying datastore.
	pub async fn scan<K>(&mut self, rng: Range<K>, limit: u32) -> Result<Vec<(Key, Val)>, Error>
	where
		K: Into<Key>,
	{
		match self {
			#[cfg(feature = "kv-echodb")]
			Transaction {
				inner: Inner::Mem(v),
				..
			} => v.scan(rng, limit),
			#[cfg(feature = "kv-yokudb")]
			Transaction {
				inner: Inner::File(v),
				..
			} => v.scan(rng, limit),
			#[cfg(feature = "kv-indxdb")]
			Transaction {
				inner: Inner::IxDB(v),
				..
			} => v.scan(rng, limit).await,
			#[cfg(feature = "kv-tikv")]
			Transaction {
				inner: Inner::TiKV(v),
				..
			} => v.scan(rng, limit).await,
			#[cfg(feature = "kv-fdb")]
			Transaction {
				inner: Inner::FDB(v),
				..
			} => v.scan(rng, limit).await,
		}
	}
	/// Update a key in the datastore if the current value matches a condition.
	pub async fn putc<K, V>(&mut self, key: K, val: V, chk: Option<V>) -> Result<(), Error>
	where
		K: Into<Key>,
		V: Into<Val>,
	{
		match self {
			#[cfg(feature = "kv-echodb")]
			Transaction {
				inner: Inner::Mem(v),
				..
			} => v.putc(key, val, chk),
			#[cfg(feature = "kv-yokudb")]
			Transaction {
				inner: Inner::File(v),
				..
			} => v.putc(key, val, chk),
			#[cfg(feature = "kv-indxdb")]
			Transaction {
				inner: Inner::IxDB(v),
				..
			} => v.putc(key, val, chk).await,
			#[cfg(feature = "kv-tikv")]
			Transaction {
				inner: Inner::TiKV(v),
				..
			} => v.putc(key, val, chk).await,
			#[cfg(feature = "kv-fdb")]
			Transaction {
				inner: Inner::FDB(v),
				..
			} => v.putc(key, val, chk).await,
		}
	}
	/// Delete a key from the datastore if the current value matches a condition.
	pub async fn delc<K, V>(&mut self, key: K, chk: Option<V>) -> Result<(), Error>
	where
		K: Into<Key>,
		V: Into<Val>,
	{
		match self {
			#[cfg(feature = "kv-echodb")]
			Transaction {
				inner: Inner::Mem(v),
				..
			} => v.delc(key, chk),
			#[cfg(feature = "kv-yokudb")]
			Transaction {
				inner: Inner::File(v),
				..
			} => v.delc(key, chk),
			#[cfg(feature = "kv-indxdb")]
			Transaction {
				inner: Inner::IxDB(v),
				..
			} => v.delc(key, chk).await,
			#[cfg(feature = "kv-tikv")]
			Transaction {
				inner: Inner::TiKV(v),
				..
			} => v.delc(key, chk).await,
			#[cfg(feature = "kv-fdb")]
			Transaction {
				inner: Inner::FDB(v),
				..
			} => v.delc(key, chk).await,
		}
	}
	/// Retrieve a specific range of keys from the datastore.
	///
	/// This function fetches key-value pairs from the underlying datastore in batches of 1000.
	pub async fn getr<K>(&mut self, rng: Range<K>, limit: u32) -> Result<Vec<(Key, Val)>, Error>
	where
		K: Into<Key>,
	{
		let beg: Key = rng.start.into();
		let end: Key = rng.end.into();
		let mut nxt: Option<Key> = None;
		let mut num = limit;
		let mut out: Vec<(Key, Val)> = vec![];
		// Start processing
		while num > 0 {
			// Get records batch
			let res = match nxt {
				None => {
					let min = beg.clone();
					let max = end.clone();
					let num = std::cmp::min(1000, num);
					self.scan(min..max, num).await?
				}
				Some(ref mut beg) => {
					beg.push(0x00);
					let min = beg.clone();
					let max = end.clone();
					let num = std::cmp::min(1000, num);
					self.scan(min..max, num).await?
				}
			};
			// Get total results
			let n = res.len();
			// Exit when settled
			if n == 0 {
				break;
			}
			// Loop over results
			for (i, (k, v)) in res.into_iter().enumerate() {
				// Ready the next
				if n == i + 1 {
					nxt = Some(k.clone());
				}
				// Delete
				out.push((k, v));
				// Count
				num -= 1;
			}
		}
		Ok(out)
	}
	/// Delete a range of keys from the datastore.
	///
	/// This function fetches key-value pairs from the underlying datastore in batches of 1000.
	pub async fn delr<K>(&mut self, rng: Range<K>, limit: u32) -> Result<(), Error>
	where
		K: Into<Key>,
	{
		let beg: Key = rng.start.into();
		let end: Key = rng.end.into();
		let mut nxt: Option<Key> = None;
		let mut num = limit;
		// Start processing
		while num > 0 {
			// Get records batch
			let res = match nxt {
				None => {
					let min = beg.clone();
					let max = end.clone();
					let num = std::cmp::min(1000, num);
					self.scan(min..max, num).await?
				}
				Some(ref mut beg) => {
					beg.push(0x00);
					let min = beg.clone();
					let max = end.clone();
					let num = std::cmp::min(1000, num);
					self.scan(min..max, num).await?
				}
			};
			// Get total results
			let n = res.len();
			// Exit when settled
			if n == 0 {
				break;
			}
			// Loop over results
			for (i, (k, _)) in res.into_iter().enumerate() {
				// Ready the next
				if n == i + 1 {
					nxt = Some(k.clone());
				}
				// Delete
				self.del(k).await?;
				// Count
				num -= 1;
			}
		}
		Ok(())
	}
	/// Retrieve a specific prefix of keys from the datastore.
	///
	/// This function fetches key-value pairs from the underlying datastore in batches of 1000.
	pub async fn getp<K>(&mut self, key: K, limit: u32) -> Result<Vec<(Key, Val)>, Error>
	where
		K: Into<Key>,
	{
		let beg: Key = key.into();
		let end: Key = beg.clone().add(0xff);
		let mut nxt: Option<Key> = None;
		let mut num = limit;
		let mut out: Vec<(Key, Val)> = vec![];
		// Start processing
		while num > 0 {
			// Get records batch
			let res = match nxt {
				None => {
					let min = beg.clone();
					let max = end.clone();
					let num = std::cmp::min(1000, num);
					self.scan(min..max, num).await?
				}
				Some(ref mut beg) => {
					beg.push(0);
					let min = beg.clone();
					let max = end.clone();
					let num = std::cmp::min(1000, num);
					self.scan(min..max, num).await?
				}
			};
			// Get total results
			let n = res.len();
			// Exit when settled
			if n == 0 {
				break;
			}
			// Loop over results
			for (i, (k, v)) in res.into_iter().enumerate() {
				// Ready the next
				if n == i + 1 {
					nxt = Some(k.clone());
				}
				// Delete
				out.push((k, v));
				// Count
				num -= 1;
			}
		}
		Ok(out)
	}
	/// Delete a prefix of keys from the datastore.
	///
	/// This function fetches key-value pairs from the underlying datastore in batches of 1000.
	pub async fn delp<K>(&mut self, key: K, limit: u32) -> Result<(), Error>
	where
		K: Into<Key>,
	{
		let beg: Key = key.into();
		let end: Key = beg.clone().add(0xff);
		let mut nxt: Option<Key> = None;
		let mut num = limit;
		// Start processing
		while num > 0 {
			// Get records batch
			let res = match nxt {
				None => {
					let min = beg.clone();
					let max = end.clone();
					let num = std::cmp::min(1000, num);
					self.scan(min..max, num).await?
				}
				Some(ref mut beg) => {
					beg.push(0);
					let min = beg.clone();
					let max = end.clone();
					let num = std::cmp::min(1000, num);
					self.scan(min..max, num).await?
				}
			};
			// Get total results
			let n = res.len();
			// Exit when settled
			if n == 0 {
				break;
			}
			// Loop over results
			for (i, (k, _)) in res.into_iter().enumerate() {
				// Ready the next
				if n == i + 1 {
					nxt = Some(k.clone());
				}
				// Delete
				self.del(k).await?;
				// Count
				num -= 1;
			}
		}
		Ok(())
	}
	/// Retrieve all namespace definitions in a datastore.
	pub async fn all_ns(&mut self) -> Result<Arc<Vec<DefineNamespaceStatement>>, Error> {
		let key = crate::key::ns::prefix();
		match self.cache.exi(&key) {
			true => match self.cache.get(&key) {
				Some(Entry::Nss(v)) => Ok(v),
				_ => unreachable!(),
			},
			_ => {
				let beg = crate::key::ns::prefix();
				let end = crate::key::ns::suffix();
				let val = self.getr(beg..end, u32::MAX).await?;
				let val = Arc::new(val.convert());
				self.cache.set(key, Entry::Nss(val.clone()));
				Ok(val)
			}
		}
	}
	/// Retrieve all namespace login definitions for a specific namespace.
	pub async fn all_nl(&mut self, ns: &str) -> Result<Arc<Vec<DefineLoginStatement>>, Error> {
		let key = crate::key::nl::prefix(ns);
		match self.cache.exi(&key) {
			true => match self.cache.get(&key) {
				Some(Entry::Nls(v)) => Ok(v),
				_ => unreachable!(),
			},
			_ => {
				let beg = crate::key::nl::prefix(ns);
				let end = crate::key::nl::suffix(ns);
				let val = self.getr(beg..end, u32::MAX).await?;
				let val = Arc::new(val.convert());
				self.cache.set(key, Entry::Nls(val.clone()));
				Ok(val)
			}
		}
	}
	/// Retrieve all namespace token definitions for a specific namespace.
	pub async fn all_nt(&mut self, ns: &str) -> Result<Arc<Vec<DefineTokenStatement>>, Error> {
		let key = crate::key::nt::prefix(ns);
		match self.cache.exi(&key) {
			true => match self.cache.get(&key) {
				Some(Entry::Nts(v)) => Ok(v),
				_ => unreachable!(),
			},
			_ => {
				let beg = crate::key::nt::prefix(ns);
				let end = crate::key::nt::suffix(ns);
				let val = self.getr(beg..end, u32::MAX).await?;
				let val = Arc::new(val.convert());
				self.cache.set(key, Entry::Nts(val.clone()));
				Ok(val)
			}
		}
	}
	/// Retrieve all database definitions for a specific namespace.
	pub async fn all_db(&mut self, ns: &str) -> Result<Arc<Vec<DefineDatabaseStatement>>, Error> {
		let key = crate::key::db::prefix(ns);
		match self.cache.exi(&key) {
			true => match self.cache.get(&key) {
				Some(Entry::Dbs(v)) => Ok(v),
				_ => unreachable!(),
			},
			_ => {
				let beg = crate::key::db::prefix(ns);
				let end = crate::key::db::suffix(ns);
				let val = self.getr(beg..end, u32::MAX).await?;
				let val = Arc::new(val.convert());
				self.cache.set(key, Entry::Dbs(val.clone()));
				Ok(val)
			}
		}
	}
	/// Retrieve all database login definitions for a specific database.
	pub async fn all_dl(
		&mut self,
		ns: &str,
		db: &str,
	) -> Result<Arc<Vec<DefineLoginStatement>>, Error> {
		let key = crate::key::dl::prefix(ns, db);
		match self.cache.exi(&key) {
			true => match self.cache.get(&key) {
				Some(Entry::Dls(v)) => Ok(v),
				_ => unreachable!(),
			},
			_ => {
				let beg = crate::key::dl::prefix(ns, db);
				let end = crate::key::dl::suffix(ns, db);
				let val = self.getr(beg..end, u32::MAX).await?;
				let val = Arc::new(val.convert());
				self.cache.set(key, Entry::Dls(val.clone()));
				Ok(val)
			}
		}
	}
	/// Retrieve all database token definitions for a specific database.
	pub async fn all_dt(
		&mut self,
		ns: &str,
		db: &str,
	) -> Result<Arc<Vec<DefineTokenStatement>>, Error> {
		let key = crate::key::dt::prefix(ns, db);
		match self.cache.exi(&key) {
			true => match self.cache.get(&key) {
				Some(Entry::Dts(v)) => Ok(v),
				_ => unreachable!(),
			},
			_ => {
				let beg = crate::key::dt::prefix(ns, db);
				let end = crate::key::dt::suffix(ns, db);
				let val = self.getr(beg..end, u32::MAX).await?;
				let val = Arc::new(val.convert());
				self.cache.set(key, Entry::Dts(val.clone()));
				Ok(val)
			}
		}
	}
	/// Retrieve all scope definitions for a specific database.
	pub async fn all_sc(
		&mut self,
		ns: &str,
		db: &str,
	) -> Result<Arc<Vec<DefineScopeStatement>>, Error> {
		let key = crate::key::sc::prefix(ns, db);
		match self.cache.exi(&key) {
			true => match self.cache.get(&key) {
				Some(Entry::Scs(v)) => Ok(v),
				_ => unreachable!(),
			},
			_ => {
				let beg = crate::key::sc::prefix(ns, db);
				let end = crate::key::sc::suffix(ns, db);
				let val = self.getr(beg..end, u32::MAX).await?;
				let val = Arc::new(val.convert());
				self.cache.set(key, Entry::Scs(val.clone()));
				Ok(val)
			}
		}
	}
	/// Retrieve all scope token definitions for a scope.
	pub async fn all_st(
		&mut self,
		ns: &str,
		db: &str,
		sc: &str,
	) -> Result<Arc<Vec<DefineTokenStatement>>, Error> {
		let key = crate::key::st::prefix(ns, db, sc);
		match self.cache.exi(&key) {
			true => match self.cache.get(&key) {
				Some(Entry::Sts(v)) => Ok(v),
				_ => unreachable!(),
			},
			_ => {
				let beg = crate::key::st::prefix(ns, db, sc);
				let end = crate::key::st::suffix(ns, db, sc);
				let val = self.getr(beg..end, u32::MAX).await?;
				let val = Arc::new(val.convert());
				self.cache.set(key, Entry::Sts(val.clone()));
				Ok(val)
			}
		}
	}
	/// Retrieve all table definitions for a specific database.
	pub async fn all_tb(
		&mut self,
		ns: &str,
		db: &str,
	) -> Result<Arc<Vec<DefineTableStatement>>, Error> {
		let key = crate::key::tb::prefix(ns, db);
		match self.cache.exi(&key) {
			true => match self.cache.get(&key) {
				Some(Entry::Tbs(v)) => Ok(v),
				_ => unreachable!(),
			},
			_ => {
				let beg = crate::key::tb::prefix(ns, db);
				let end = crate::key::tb::suffix(ns, db);
				let val = self.getr(beg..end, u32::MAX).await?;
				let val = Arc::new(val.convert());
				self.cache.set(key, Entry::Tbs(val.clone()));
				Ok(val)
			}
		}
	}
	/// Retrieve all event definitions for a specific table.
	pub async fn all_ev(
		&mut self,
		ns: &str,
		db: &str,
		tb: &str,
	) -> Result<Arc<Vec<DefineEventStatement>>, Error> {
		let key = crate::key::ev::prefix(ns, db, tb);
		match self.cache.exi(&key) {
			true => match self.cache.get(&key) {
				Some(Entry::Evs(v)) => Ok(v),
				_ => unreachable!(),
			},
			_ => {
				let beg = crate::key::ev::prefix(ns, db, tb);
				let end = crate::key::ev::suffix(ns, db, tb);
				let val = self.getr(beg..end, u32::MAX).await?;
				let val = Arc::new(val.convert());
				self.cache.set(key, Entry::Evs(val.clone()));
				Ok(val)
			}
		}
	}
	/// Retrieve all field definitions for a specific table.
	pub async fn all_fd(
		&mut self,
		ns: &str,
		db: &str,
		tb: &str,
	) -> Result<Arc<Vec<DefineFieldStatement>>, Error> {
		let key = crate::key::fd::prefix(ns, db, tb);
		match self.cache.exi(&key) {
			true => match self.cache.get(&key) {
				Some(Entry::Fds(v)) => Ok(v),
				_ => unreachable!(),
			},
			_ => {
				let beg = crate::key::fd::prefix(ns, db, tb);
				let end = crate::key::fd::suffix(ns, db, tb);
				let val = self.getr(beg..end, u32::MAX).await?;
				let val = Arc::new(val.convert());
				self.cache.set(key, Entry::Fds(val.clone()));
				Ok(val)
			}
		}
	}
	/// Retrieve all index definitions for a specific table.
	pub async fn all_ix(
		&mut self,
		ns: &str,
		db: &str,
		tb: &str,
	) -> Result<Arc<Vec<DefineIndexStatement>>, Error> {
		let key = crate::key::ix::prefix(ns, db, tb);
		match self.cache.exi(&key) {
			true => match self.cache.get(&key) {
				Some(Entry::Ixs(v)) => Ok(v),
				_ => unreachable!(),
			},
			_ => {
				let beg = crate::key::ix::prefix(ns, db, tb);
				let end = crate::key::ix::suffix(ns, db, tb);
				let val = self.getr(beg..end, u32::MAX).await?;
				let val = Arc::new(val.convert());
				self.cache.set(key, Entry::Ixs(val.clone()));
				Ok(val)
			}
		}
	}
	/// Retrieve all view definitions for a specific table.
	pub async fn all_ft(
		&mut self,
		ns: &str,
		db: &str,
		tb: &str,
	) -> Result<Arc<Vec<DefineTableStatement>>, Error> {
		let key = crate::key::ft::prefix(ns, db, tb);
		match self.cache.exi(&key) {
			true => match self.cache.get(&key) {
				Some(Entry::Fts(v)) => Ok(v),
				_ => unreachable!(),
			},
			_ => {
				let beg = crate::key::ft::prefix(ns, db, tb);
				let end = crate::key::ft::suffix(ns, db, tb);
				let val = self.getr(beg..end, u32::MAX).await?;
				let val = Arc::new(val.convert());
				self.cache.set(key, Entry::Fts(val.clone()));
				Ok(val)
			}
		}
	}
	/// Retrieve all live definitions for a specific table.
	pub async fn all_lv(
		&mut self,
		ns: &str,
		db: &str,
		tb: &str,
	) -> Result<Arc<Vec<LiveStatement>>, Error> {
		let key = crate::key::lv::prefix(ns, db, tb);
		match self.cache.exi(&key) {
			true => match self.cache.get(&key) {
				Some(Entry::Lvs(v)) => Ok(v),
				_ => unreachable!(),
			},
			_ => {
				let beg = crate::key::lv::prefix(ns, db, tb);
				let end = crate::key::lv::suffix(ns, db, tb);
				let val = self.getr(beg..end, u32::MAX).await?;
				let val = Arc::new(val.convert());
				self.cache.set(key, Entry::Lvs(val.clone()));
				Ok(val)
			}
		}
	}
	/// Retrieve a specific namespace definition.
	pub async fn get_ns(&mut self, ns: &str) -> Result<DefineNamespaceStatement, Error> {
		let key = crate::key::ns::new(ns);
		let val = self.get(key).await?.ok_or(Error::NsNotFound)?;
		Ok(val.into())
	}
	/// Retrieve a specific namespace login definition.
	pub async fn get_nl(&mut self, ns: &str, nl: &str) -> Result<DefineLoginStatement, Error> {
		let key = crate::key::nl::new(ns, nl);
		let val = self.get(key).await?.ok_or(Error::NlNotFound)?;
		Ok(val.into())
	}
	/// Retrieve a specific namespace token definition.
	pub async fn get_nt(&mut self, ns: &str, nt: &str) -> Result<DefineTokenStatement, Error> {
		let key = crate::key::nt::new(ns, nt);
		let val = self.get(key).await?.ok_or(Error::NtNotFound)?;
		Ok(val.into())
	}
	/// Retrieve a specific database definition.
	pub async fn get_db(&mut self, ns: &str, db: &str) -> Result<DefineDatabaseStatement, Error> {
		let key = crate::key::db::new(ns, db);
		let val = self.get(key).await?.ok_or(Error::DbNotFound)?;
		Ok(val.into())
	}
	/// Retrieve a specific database login definition.
	pub async fn get_dl(
		&mut self,
		ns: &str,
		db: &str,
		dl: &str,
	) -> Result<DefineLoginStatement, Error> {
		let key = crate::key::dl::new(ns, db, dl);
		let val = self.get(key).await?.ok_or(Error::DlNotFound)?;
		Ok(val.into())
	}
	/// Retrieve a specific database token definition.
	pub async fn get_dt(
		&mut self,
		ns: &str,
		db: &str,
		dt: &str,
	) -> Result<DefineTokenStatement, Error> {
		let key = crate::key::dt::new(ns, db, dt);
		let val = self.get(key).await?.ok_or(Error::DtNotFound)?;
		Ok(val.into())
	}
	/// Retrieve a specific scope definition.
	pub async fn get_sc(
		&mut self,
		ns: &str,
		db: &str,
		sc: &str,
	) -> Result<DefineScopeStatement, Error> {
		let key = crate::key::sc::new(ns, db, sc);
		let val = self.get(key).await?.ok_or(Error::ScNotFound)?;
		Ok(val.into())
	}
	/// Retrieve a specific scope token definition.
	pub async fn get_st(
		&mut self,
		ns: &str,
		db: &str,
		sc: &str,
		st: &str,
	) -> Result<DefineTokenStatement, Error> {
		let key = crate::key::st::new(ns, db, sc, st);
		let val = self.get(key).await?.ok_or(Error::StNotFound)?;
		Ok(val.into())
	}
	/// Retrieve a specific table definition.
	pub async fn get_tb(
		&mut self,
		ns: &str,
		db: &str,
		tb: &str,
	) -> Result<DefineTableStatement, Error> {
		let key = crate::key::tb::new(ns, db, tb);
		let val = self.get(key).await?.ok_or(Error::TbNotFound)?;
		Ok(val.into())
	}
	/// Add a namespace with a default configuration, only if we are in dynamic mode.
	pub async fn add_ns(
		&mut self,
		ns: &str,
		strict: bool,
	) -> Result<DefineNamespaceStatement, Error> {
		match self.get_ns(ns).await {
			Err(Error::NsNotFound) => match strict {
				false => {
					let key = crate::key::ns::new(ns);
					let val = DefineNamespaceStatement {
						name: ns.to_owned().into(),
					};
					self.put(key, &val).await?;
					Ok(val)
				}
				true => Err(Error::NsNotFound),
			},
			Err(e) => Err(e),
			Ok(v) => Ok(v),
		}
	}
	/// Add a database with a default configuration, only if we are in dynamic mode.
	pub async fn add_db(
		&mut self,
		ns: &str,
		db: &str,
		strict: bool,
	) -> Result<DefineDatabaseStatement, Error> {
		match self.get_db(ns, db).await {
			Err(Error::DbNotFound) => match strict {
				false => {
					let key = crate::key::db::new(ns, db);
					let val = DefineDatabaseStatement {
						name: db.to_owned().into(),
					};
					self.put(key, &val).await?;
					Ok(val)
				}
				true => Err(Error::DbNotFound),
			},
			Err(e) => Err(e),
			Ok(v) => Ok(v),
		}
	}
	/// Add a table with a default configuration, only if we are in dynamic mode.
	pub async fn add_tb(
		&mut self,
		ns: &str,
		db: &str,
		tb: &str,
		strict: bool,
	) -> Result<DefineTableStatement, Error> {
		match self.get_tb(ns, db, tb).await {
			Err(Error::TbNotFound) => match strict {
				false => {
					let key = crate::key::tb::new(ns, db, tb);
					let val = DefineTableStatement {
						name: tb.to_owned().into(),
						permissions: Permissions::none(),
						..DefineTableStatement::default()
					};
					self.put(key, &val).await?;
					Ok(val)
				}
				true => Err(Error::TbNotFound),
			},
			Err(e) => Err(e),
			Ok(v) => Ok(v),
		}
	}
	/// Retrieve and cache a specific namespace definition.
	pub async fn get_and_cache_ns(
		&mut self,
		ns: &str,
	) -> Result<Arc<DefineNamespaceStatement>, Error> {
		let key = crate::key::ns::new(ns).encode()?;
		match self.cache.exi(&key) {
			true => match self.cache.get(&key) {
				Some(Entry::Ns(v)) => Ok(v),
				_ => unreachable!(),
			},
			_ => {
				let val = self.get(key.clone()).await?.ok_or(Error::NsNotFound)?;
				let val: Arc<DefineNamespaceStatement> = Arc::new(val.into());
				self.cache.set(key, Entry::Ns(val.clone()));
				Ok(val)
			}
		}
	}
	/// Retrieve and cache a specific database definition.
	pub async fn get_and_cache_db(
		&mut self,
		ns: &str,
		db: &str,
	) -> Result<Arc<DefineDatabaseStatement>, Error> {
		let key = crate::key::db::new(ns, db).encode()?;
		match self.cache.exi(&key) {
			true => match self.cache.get(&key) {
				Some(Entry::Db(v)) => Ok(v),
				_ => unreachable!(),
			},
			_ => {
				let val = self.get(key.clone()).await?.ok_or(Error::DbNotFound)?;
				let val: Arc<DefineDatabaseStatement> = Arc::new(val.into());
				self.cache.set(key, Entry::Db(val.clone()));
				Ok(val)
			}
		}
	}
	/// Retrieve and cache a specific table definition.
	pub async fn get_and_cache_tb(
		&mut self,
		ns: &str,
		db: &str,
		tb: &str,
	) -> Result<Arc<DefineTableStatement>, Error> {
		let key = crate::key::tb::new(ns, db, tb).encode()?;
		match self.cache.exi(&key) {
			true => match self.cache.get(&key) {
				Some(Entry::Tb(v)) => Ok(v),
				_ => unreachable!(),
			},
			_ => {
				let val = self.get(key.clone()).await?.ok_or(Error::TbNotFound)?;
				let val: Arc<DefineTableStatement> = Arc::new(val.into());
				self.cache.set(key, Entry::Tb(val.clone()));
				Ok(val)
			}
		}
	}
	/// Add a namespace with a default configuration, only if we are in dynamic mode.
	pub async fn add_and_cache_ns(
		&mut self,
		ns: &str,
		strict: bool,
	) -> Result<Arc<DefineNamespaceStatement>, Error> {
		match self.get_and_cache_ns(ns).await {
			Err(Error::NsNotFound) => match strict {
				false => {
					let key = crate::key::ns::new(ns);
					let val = DefineNamespaceStatement {
						name: ns.to_owned().into(),
					};
					self.put(key, &val).await?;
					Ok(Arc::new(val))
				}
				true => Err(Error::NsNotFound),
			},
			Err(e) => Err(e),
			Ok(v) => Ok(v),
		}
	}
	/// Add a database with a default configuration, only if we are in dynamic mode.
	pub async fn add_and_cache_db(
		&mut self,
		ns: &str,
		db: &str,
		strict: bool,
	) -> Result<Arc<DefineDatabaseStatement>, Error> {
		match self.get_and_cache_db(ns, db).await {
			Err(Error::DbNotFound) => match strict {
				false => {
					let key = crate::key::db::new(ns, db);
					let val = DefineDatabaseStatement {
						name: db.to_owned().into(),
					};
					self.put(key, &val).await?;
					Ok(Arc::new(val))
				}
				true => Err(Error::DbNotFound),
			},
			Err(e) => Err(e),
			Ok(v) => Ok(v),
		}
	}
	/// Add a table with a default configuration, only if we are in dynamic mode.
	pub async fn add_and_cache_tb(
		&mut self,
		ns: &str,
		db: &str,
		tb: &str,
		strict: bool,
	) -> Result<Arc<DefineTableStatement>, Error> {
		match self.get_and_cache_tb(ns, db, tb).await {
			Err(Error::TbNotFound) => match strict {
				false => {
					let key = crate::key::tb::new(ns, db, tb);
					let val = DefineTableStatement {
						name: tb.to_owned().into(),
						permissions: Permissions::none(),
						..DefineTableStatement::default()
					};
					self.put(key, &val).await?;
					Ok(Arc::new(val))
				}
				true => Err(Error::TbNotFound),
			},
			Err(e) => Err(e),
			Ok(v) => Ok(v),
		}
	}
	/// Retrieve and cache a specific table definition.
	pub async fn check_ns_db_tb(
		&mut self,
		ns: &str,
		db: &str,
		tb: &str,
		strict: bool,
	) -> Result<(), Error> {
		match strict {
			// Strict mode is disabled
			false => Ok(()),
			// Strict mode is enabled
			true => {
				self.get_and_cache_ns(ns).await?;
				self.get_and_cache_db(ns, db).await?;
				self.get_and_cache_tb(ns, db, tb).await?;
				Ok(())
			}
		}
	}
	/// Writes the full database contents as binary SQL.
	pub async fn export(&mut self, ns: &str, db: &str, chn: Sender<Vec<u8>>) -> Result<(), Error> {
		// Output OPTIONS
		{
			chn.send(bytes!("-- ------------------------------")).await?;
			chn.send(bytes!("-- OPTION")).await?;
			chn.send(bytes!("-- ------------------------------")).await?;
			chn.send(bytes!("")).await?;
			chn.send(bytes!("OPTION IMPORT;")).await?;
			chn.send(bytes!("")).await?;
		}
		// Output LOGINS
		{
			let dls = self.all_dl(ns, db).await?;
			if !dls.is_empty() {
				chn.send(bytes!("-- ------------------------------")).await?;
				chn.send(bytes!("-- LOGINS")).await?;
				chn.send(bytes!("-- ------------------------------")).await?;
				chn.send(bytes!("")).await?;
				for dl in dls.iter() {
					chn.send(bytes!(format!("{};", dl))).await?;
				}
				chn.send(bytes!("")).await?;
			}
		}
		// Output TOKENS
		{
			let dts = self.all_dt(ns, db).await?;
			if !dts.is_empty() {
				chn.send(bytes!("-- ------------------------------")).await?;
				chn.send(bytes!("-- TOKENS")).await?;
				chn.send(bytes!("-- ------------------------------")).await?;
				chn.send(bytes!("")).await?;
				for dt in dts.iter() {
					chn.send(bytes!(format!("{};", dt))).await?;
				}
				chn.send(bytes!("")).await?;
			}
		}
		// Output SCOPES
		{
			let scs = self.all_sc(ns, db).await?;
			if !scs.is_empty() {
				chn.send(bytes!("-- ------------------------------")).await?;
				chn.send(bytes!("-- SCOPES")).await?;
				chn.send(bytes!("-- ------------------------------")).await?;
				chn.send(bytes!("")).await?;
				for sc in scs.iter() {
					chn.send(bytes!(format!("{};", sc))).await?;
				}
				chn.send(bytes!("")).await?;
			}
		}
		// Output TABLES
		{
			let tbs = self.all_tb(ns, db).await?;
			if !tbs.is_empty() {
				for tb in tbs.iter() {
					// Output TABLE
					chn.send(bytes!("-- ------------------------------")).await?;
					chn.send(bytes!(format!("-- TABLE: {}", tb.name))).await?;
					chn.send(bytes!("-- ------------------------------")).await?;
					chn.send(bytes!("")).await?;
					chn.send(bytes!(format!("{};", tb))).await?;
					chn.send(bytes!("")).await?;
					// Output FIELDS
					{
						let fds = self.all_fd(ns, db, &tb.name).await?;
						if !fds.is_empty() {
							for fd in fds.iter() {
								chn.send(bytes!(format!("{};", fd))).await?;
							}
							chn.send(bytes!("")).await?;
						}
					}
					// Output INDEXES
					let ixs = self.all_ix(ns, db, &tb.name).await?;
					if !ixs.is_empty() {
						for ix in ixs.iter() {
							chn.send(bytes!(format!("{};", ix))).await?;
						}
						chn.send(bytes!("")).await?;
					}
					// Output EVENTS
					let evs = self.all_ev(ns, db, &tb.name).await?;
					if !evs.is_empty() {
						for ev in evs.iter() {
							chn.send(bytes!(format!("{};", ev))).await?;
						}
						chn.send(bytes!("")).await?;
					}
				}
				// Start transaction
				chn.send(bytes!("-- ------------------------------")).await?;
				chn.send(bytes!("-- TRANSACTION")).await?;
				chn.send(bytes!("-- ------------------------------")).await?;
				chn.send(bytes!("")).await?;
				chn.send(bytes!("BEGIN TRANSACTION;")).await?;
				chn.send(bytes!("")).await?;
				// Output TABLE data
				for tb in tbs.iter() {
					// Start records
					chn.send(bytes!("-- ------------------------------")).await?;
					chn.send(bytes!(format!("-- TABLE DATA: {}", tb.name))).await?;
					chn.send(bytes!("-- ------------------------------")).await?;
					chn.send(bytes!("")).await?;
					// Fetch records
					let beg = thing::prefix(ns, db, &tb.name);
					let end = thing::suffix(ns, db, &tb.name);
					let mut nxt: Option<Vec<u8>> = None;
					loop {
						let res = match nxt {
							None => {
								let min = beg.clone();
								let max = end.clone();
								self.scan(min..max, 1000).await?
							}
							Some(ref mut beg) => {
								beg.push(0x00);
								let min = beg.clone();
								let max = end.clone();
								self.scan(min..max, 1000).await?
							}
						};
						if !res.is_empty() {
							// Get total results
							let n = res.len();
							// Exit when settled
							if n == 0 {
								break;
							}
							// Loop over results
							for (i, (k, v)) in res.into_iter().enumerate() {
								// Ready the next
								if n == i + 1 {
									nxt = Some(k.clone());
								}
								// Parse the key-value
								let k: crate::key::thing::Thing = (&k).into();
								let v: crate::sql::value::Value = (&v).into();
								let t = Thing::from((k.tb, k.id));
								// Write record
								chn.send(bytes!(format!("UPDATE {} CONTENT {};", t, v))).await?;
							}
							continue;
						}
						break;
					}
					chn.send(bytes!("")).await?;
				}
				// Commit transaction
				chn.send(bytes!("-- ------------------------------")).await?;
				chn.send(bytes!("-- TRANSACTION")).await?;
				chn.send(bytes!("-- ------------------------------")).await?;
				chn.send(bytes!("")).await?;
				chn.send(bytes!("COMMIT TRANSACTION;")).await?;
				chn.send(bytes!("")).await?;
			}
		}
		// Everything exported
		Ok(())
	}
}
