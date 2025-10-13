Сначала надо забилдить оба контаркта:
~$ cargo build-sbf --manifest-path=code/caller/Cargo.toml
~$ cargo build-sbf --manifest-path=code/called/Cargo.toml

Потом запустить локальный валидатор:
~$ solana-test-validator (в сниппетах лежит усложененная команда вызова валидатора которая обходит проблемы WSL)

Задеплоить оба контракта:
~$ solana program deploy target/deploy/called.so
~$ solana program deploy target/deploy/caller.so

И запустить тест (для простоты я запускаю все тесты):
~$ cargo test -- --nocapture

