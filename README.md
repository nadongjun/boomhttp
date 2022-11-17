# boomhttp

## Run 
```sh
source $HOME/.cargo/env
# configuration 파일을 읽음
cargo run -- configuration/config.json
```

## Test
```sh
cargo test
```

## Requirements
1. 설정 파일을 읽어오는 CLI application을 만든다.
2. TCP 소켓 연결요청을 수신한다.
3. HTTP 요청의 일부를 분석하고, 적절한 HTTP 응답을 만든다. CRUD를 할 수 있다.
4. Error handling, unit test를 추가한다.
5. (Optional) 스레드 풀을 이용해 서버의 응답속도를 개선한다.

## Structure
```
├── backend
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── cli
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── http
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── pool
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── src
│   └── main.rs
├── tests
│── http_test.rs
└── parser_test.rs
```


## Module
### http
- http 서버 실행 
  
### pool
- http 요청 멀티스레드 처리

### parser
- cli 파라미터 파싱
- Loop reedline 

### backend
- HTTP 응답 메세지 생성
- CRUD 요청 처리

## To-do
1. CRUD 기능 구현 (GET, PUT, DEL) [구현](backend/src/lib.rs)
   
## Reference
- [lib생성](https://webcache.googleusercontent.com/search?q=cache:MvISwrshgSYJ:https://bguru.tistory.com/55&cd=7&hl=ko&ct=clnk&gl=kr)