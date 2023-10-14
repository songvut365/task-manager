# Task Manager

## How to run
```shell
docker-compose up -d
```

## APIs

### GET /task
```shell
curl --location 'http://localhost:8080/task' \
--header 'user-id: 18291616-7b5d-4675-ab39-0a884c23e900' \
--header 'x-ref-id: 7b5c23bc-5224-445e-9826-9c5775f878ac'
```

### GET /task/:task_id
```shell
curl --location 'http://localhost:8080/task/db2eac14-d0d9-4581-91b6-bd1de5aebb32' \
--header 'user-id: 18291616-7b5d-4675-ab39-0a884c23e900' \
--header 'x-ref-id: b07d76af-8c4a-4e9a-9578-4689c68c5ba9'
```

### POST /task
```shell
curl --location 'http://localhost:8080/task' \
--header 'x-ref-id: 8135d438-c070-43dc-be25-99e447b42588' \
--header 'user-id: 18291616-7b5d-4675-ab39-0a884c23e900' \
--header 'Content-Type: application/json' \
--data '{
    "title": "code",
    "description": "code some rust program",
    "completed": false
}'
```

### PUT /task/:task_id
```shell
curl --location --request PUT 'http://localhost:8080/task' \
--header 'x-ref-id: 30642bc7-1d3b-4c2f-8fea-7def70442032' \
--header 'user-id: 18291616-7b5d-4675-ab39-0a884c23e900' \
--header 'Content-Type: application/json' \
--data '{
    "title": "code",
    "description": "code some rust program",
    "completed": true
}'
```

### DELETE /task/:task_id
```shell
curl --location --request DELETE 'http://localhost:8080/task/db2eac14-d0d9-4581-91b6-bd1de5aebb32' \
--header 'x-ref-id: c2b41783-c911-43a9-a767-abfad39a7c96' \
--header 'user-id: 18291616-7b5d-4675-ab39-0a884c23e900'
```


## How to fix development environment issues (Windows)
### Error: RUST_BACKTRACE=1
To enable backtraces for Rust errors, set the `RUST_BACKTRACE` environment variable to `1`
 
```
set RUST_BACKTRACE=1
```

### Error: LINK libpg.lib
set the `PQ_LIB_DIR` environment variable to the path where your PostgreSQL library files are located
```
setx PQ_LIB_DIR "C:\Program Files\PostgreSQL\16\lib"
```

### Error: STATUS_DLL_NOT_FOUND

Add the PostgreSQL library directory to the `PATH`

```
1. C:\Program Files\PostgreSQL\16\lib
2. C:\Program Files\PostgreSQL\16\bin
```

Make sure to restart your development environment or terminal after making these changes to apply the settings.