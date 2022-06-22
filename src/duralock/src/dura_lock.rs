const  DURATION_EXTRACT: u64 = 100000000; //3 digit days// 2 digit hours// 2 digit minutes

use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::hash_map::DefaultHasher;
use candid::CandidType;
use oorandom::Rand64;
use std::hash::Hash;
use std::hash::Hasher;
use chrono::{DateTime,Duration,Utc, NaiveDateTime};
use std::cmp::Ordering;
const  SOME_SEED:u128 = 4;
lazy_static! {
    static ref RANDOM_GEN:Rand64 =   {
        let m =oorandom::Rand64::new(SOME_SEED);
        m
    };
}


#[derive(Default,Eq,  PartialEq,  Clone )]
pub struct DuraKey{
    dura_slot: DuraSlot,
    key_hash: u64
}
impl Hash for DuraKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dura_slot.hash(state);
        self.key_hash.hash(state);
    }
}
impl Ord for DuraKey {
    fn cmp(&self, other:&Self) -> Ordering {
        let self_td = self.dura_slot.time_and_duration;
        let other_td = other.dura_slot.time_and_duration;
        if self_td > other_td {
            return Ordering::Less;
        }
        if self_td < other_td {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}
impl PartialOrd for DuraKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl DuraKey {

    pub fn new (duration:DuraSlot) -> Self {
        Self {
         dura_slot : duration,
         key_hash :0
        }

    }
    pub fn generate_key(&mut self)  -> u64{
       let random_key = 0 as u64;
      // let random_key = RANDOM_GEN.rand_u64();
       let mut s = DefaultHasher::new();
       random_key.hash(&mut s);
       self.key_hash = s.finish();
       return random_key;
    }

    pub fn get_key_hash(&self) -> u64 {
        return self.key_hash;
    }
}

#[derive(Default,Eq, PartialEq,  Clone)]
pub struct DuraSlot {
    time_and_duration: u64
}

impl Hash for DuraSlot{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.time_and_duration.hash(state);
    }
}

impl  DuraSlot {
    pub fn init (td: u64) -> Self {
       Self { time_and_duration: td }
    }

    pub fn new (sdt: String, edt: String) -> Self {
        Self { time_and_duration: 0 }
    }

    pub fn now_in_this(&self) -> bool {
        let time_now = Utc::now();
        let start_time = self.time_and_duration/ DURATION_EXTRACT;
        let duration: u64 = self.time_and_duration - start_time;

        let start_date_time = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(start_time as i64, 0), Utc);
        let end_date_time = start_date_time.checked_add_signed(Duration::seconds(duration as i64)).unwrap();
     
        if time_now>start_date_time&&time_now < end_date_time {
            return true;
        }
        else {
            return false;
        }
    }    
}
#[derive(Default,Eq, Hash, PartialEq,  Clone)]
pub struct AssetKey {
    asset_name: String,
    asset_id: u64
}

impl AssetKey {
    pub fn new (name:String, id:u64) -> Self {
        Self {
            asset_name: name,
            asset_id:id
        }
    }
}


#[derive(Default)]
pub struct DuraLockDB {
     asset_locks: HashMap<AssetKey,BTreeSet<DuraKey>>
}


impl DuraLockDB {

     fn new() -> Self {
        Self { 
           asset_locks:HashMap::default()
        }
    }
    pub fn get_key_hash(&self,name: &String) -> u64{
        let asset_key = AssetKey::new(name.to_string(),0);
        let mut return_val: u64=0;
        if let Some(time_slots) =self.asset_locks.get(&asset_key) {
            for dura_key  in time_slots.iter() {
                if dura_key.dura_slot.now_in_this() {
                    return_val = dura_key.key_hash;
                }
            }
        }
        return_val
    }

     pub fn generate_lock(&self, name:&String, start_date_time:&String, end_date_time:&String) -> String {
        let duration = DuraSlot::new(start_date_time.to_string(),end_date_time.to_string());
        let mut dura_key:DuraKey = DuraKey::new(duration);
        let key = dura_key.generate_key();
        let asset_key = AssetKey::new(name.to_string(),0);
        //let time_slots = self.asset_locks.get(&asset_key).unwrap();
        //time_slots.insert(dura_key);
        key.to_string()
    } 

    pub fn clean(&self) {

    } 

}

    
