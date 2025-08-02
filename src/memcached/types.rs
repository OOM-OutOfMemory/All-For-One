use deadpool::managed::Object;
use deadpool_memcached::Manager;

pub type MemCachedClient = Object<Manager>;
