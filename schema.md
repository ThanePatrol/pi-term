# Device Table
DB purpose is to store config info to allow easy connection to any device in a cluster.
Each device should be identified by a primary key. 

| PK  | node_ip | baud_rate | user | preferred_port | tty_device_path |
|-----|---------|-----------|------|----------------|-----------------|
|     |         |           |      |                |                 |
|     |         |           |      |                |                 |

```sql
 create table devices
 (
     node_id   INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
     node_ip   INTEGER NOT NULL,
     baud_rate INTEGER NOT NULL,
     user      TEXT    NOT NULL,
     port      INTEGER NOT NULL,
     tty_path  TEXT    NOT NULL
 ); 
```

# Preferences Table
Table to store random preferences. Config such as dark/light mode, 
preferred shell, 
