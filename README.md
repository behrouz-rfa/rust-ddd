<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://camo.githubusercontent.com/734a3468bce992fbc3b729562d41c92f4912c99a/68747470733a2f2f7777772e727573742d6c616e672e6f72672f7374617469632f696d616765732f727573742d6c6f676f2d626c6b2e737667" height="120" width="120" />
  </div>
  <h1 align="center">rust-ddd</h1>
  <h4 align="center">Rust Domain-Driven-Design (DDD) </h4>
</div>

## Summery

This repository is used to present how I find implementing DDD in Rust projects works out

## Running 

1. Clone the repository locally

```shell
https://github.com/behrouz-rfa/rust-ddd.git
```

2. Execute the `bin/dotenv` script to create a `.env` file
   or copy the contents of the `.env.sample` file into a new file
   with the name `.env`

3. Run the Docker 

```shell
docker compose up -d
```

4. Install dependencies and execute the server

```bash
cargo run
```

## Examples
1. For insert or registration
```
curl --location --request POST 'localhost:8082/api/users/insert' \
--header 'Content-Type: application/json' \
--data-raw '{
"username":"behrouz.r.fa",
"email":"behrouz.r.fa@gmail.com'\''",
"password":"pass@123"
}'
```
2. for login
```
curl --location --request POST 'localhost:8082/api/users/login' \
--header 'Content-Type: application/json' \
--data-raw '{
"email":"behrouz.r.fa@gmail.com'\''",
"password":"pass@123"
}'
```
3. update user
```
curl --location --request PUT 'localhost:8082/api/user' \
--header 'Authorization: Token eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2NzMzMTY3OTUsImlkIjoyOCwidXNlcm5hbWUiOiJ0ZXN0MTEyMjIifQ._N82DPNiw27gVdBFuEPv2Tps_TbUH6wXgq-wBIxUQfc' \
--header 'Content-Type: application/json' \
--data-raw '{
"email":"master@gmail.com'\''",
"username":"master@123",
"bio": "this test for bio"
}'
```