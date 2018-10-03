# diesel-bench
Project to benchmark the rust frameworks diesel (sql) and actix-web (web framework)

## postgres docker
create
``docker run --name orm_bench -e POSTGRES_PASSWORD=orm_bench -e POSTGRES_USER=orm_bench -d orm_bench``

connect
``docker run -it --rm --link orm_bench orm_bench psql -h orm_bench -U orm_bench``