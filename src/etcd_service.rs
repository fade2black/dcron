use etcd_client::{GetOptions, LockOptions};
use std::{env, error::Error};

type EtcdClient = etcd_client::Client;
pub struct Client(EtcdClient);

impl Client {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let url = env::var("ETCD_URL")?;
        Ok(Self(EtcdClient::connect([url], None).await?))
    }

    pub async fn lock(&mut self) -> Result<String, Box<dyn Error>> {
        let resp = self.0.lease_grant(60, None).await?;
        let lock_options = LockOptions::new().with_lease(resp.id());

        let resp = self.0.lock("lock-cron", Some(lock_options)).await?;
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

    pub async fn get_cron_jobs(
        &mut self,
        key_prefix: &str,
    ) -> Result<Vec<(String, String)>, Box<dyn Error>> {
        let options = Some(GetOptions::new().with_prefix());
        let resp: etcd_client::GetResponse = self.0.get(key_prefix, options).await?;

        let mut jobs = vec![];

        for kv in resp.kvs().into_iter() {
            let k = kv.key_str()?.to_string();
            let v = kv.value_str()?.to_string();

            jobs.push((k, v));
        }

        Ok(jobs)
    }
}
