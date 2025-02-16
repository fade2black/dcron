use etcd_client::{GetOptions, LockOptions};
use std::{env, error::Error};

type EtcdClient = etcd_client::Client;

const KEY_PREFIX: &str = "cron";
const LOCK_NAME: &str = "lock-cron";
const LEASE_TTL: i64 = 30;

pub struct Client(EtcdClient);

impl Client {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let url = env::var("ETCD_URL")?;
        Ok(Self(EtcdClient::connect([url], None).await?))
    }

    pub async fn lock(&mut self) -> Result<String, Box<dyn Error>> {
        let resp = self.0.lease_grant(LEASE_TTL, None).await?;
        let lock_options = LockOptions::new().with_lease(resp.id());

        let resp = self.0.lock(LOCK_NAME, Some(lock_options)).await?;
        let key_str = std::str::from_utf8(resp.key())?;

        Ok(key_str.into())
    }

    pub async fn unlock(&mut self, key: &str) -> Result<(), Box<dyn Error>> {
        self.0.unlock(key).await?;
        Ok(())
    }

    pub async fn store_cron_job(&mut self, key: &str, val: &str) -> Result<(), Box<dyn Error>> {
        self.0.put(key, val, None).await?;

        Ok(())
    }

    pub async fn get_cron_jobs(&mut self) -> Result<Vec<(String, String)>, Box<dyn Error>> {
        let options = Some(GetOptions::new().with_prefix());
        let resp: etcd_client::GetResponse = self.0.get(KEY_PREFIX, options).await?;

        let mut jobs = vec![];

        for kv in resp.kvs().into_iter() {
            let k = kv.key_str()?.to_string();
            let v = kv.value_str()?.to_string();

            jobs.push((k, v));
        }

        Ok(jobs)
    }

    pub async fn delete_cron_job(&mut self, key: &str) -> Result<(), Box<dyn Error>> {
        self.0.delete(key, None).await?;
        Ok(())
    }
}
