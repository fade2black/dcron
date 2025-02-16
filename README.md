# dcron
A simple distributed cron job scheduler based on `edcd`.

# Usage
First set `ETCD_URL` as URL to `etcd` server and then run the server. 
```bash
export ETCD_URL=http://1.2.3.4:1234
./dcron
```