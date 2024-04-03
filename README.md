
Start app wit docker-compose
```
docker compose up -d
```

```
docker compose exec app cargo run
```

Run migration with docker compose
```
docker compose exec app diesel migration run
```


Install postgresql client, and connect to db
```
sudo apt install postgresql-client-common postgresql-client-14
psql -h localhost -U db_user -d db_name -W
```
Insert password, probably `secret`
