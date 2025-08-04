# auth-module
Generic auth module

## Local Setup using Docker
Run docker-compose up --build

This will start the server. This server has a dependency on Redis.

#### /get-otp endpoint will store the OTP in redis for 5 mins.
To check the OTP in redis search by phone number. Phone number is the key.

start: docker exec -it redis redis-cli
GET "phone number"
