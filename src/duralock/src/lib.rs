#[macro_use]
extern crate lazy_static;

mod dura_lock;
use dura_lock::DuraLockDB;
use ic_cdk_macros::{pre_upgrade, post_upgrade}; 
use std::{cell::RefCell, borrow::Borrow};

thread_local! {
static DURA_LOCK_DB: RefCell<DuraLockDB> = RefCell::default();
}

#[ic_cdk_macros::update]
fn gen_lock(name: String,start_date_time:String,end_date_time:String) -> String {
let mut key_ret = "done".to_string();
DURA_LOCK_DB.with(|db_ref| {
    let db_mut = db_ref.borrow_mut();
    key_ret = db_mut.generate_lock(&name,&start_date_time,&end_date_time);
});
format!("genLock, {} {} {} {}!", name,start_date_time,end_date_time,key_ret)
   
}


#[ic_cdk_macros::query]
fn get_key(name: String) -> String {
    let mut key_hash_ret: u64 = 0;
    DURA_LOCK_DB.with(|db_ref| {
        let db_ref = db_ref.borrow();
        key_hash_ret = db_ref.get_key_hash(&name);
    });
   
    format!("getKey, {}{}!", name,key_hash_ret)
}

#[pre_upgrade]
fn pre_upgrade() {
    DURA_LOCK_DB.with(|db_ref| {
        let db_mut = db_ref.borrow_mut();
        db_mut.clean();
      //  ic_cdk::storage::stable_save(db_mut);
    }); 
}

#[post_upgrade]
fn post_upgrade() {

}