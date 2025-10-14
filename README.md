
1. **Сборка контрактов**  

Сначала нужно собрать оба контракта:  

```bash
cargo build-sbf --manifest-path=code/caller/Cargo.toml
cargo build-sbf --manifest-path=code/called/Cargo.toml
````

2. **Запуск локального валидатора**

Запустить локальный Solana тестовый валидатор:

```bash
solana-test-validator
```

> В snippets.md лежит усложнённая команда запуска валидатора, которая обходит проблемы WSL.

3. **Деплой контрактов**

Задеплоить оба контракта на локальный валидатор:

```bash
solana program deploy target/deploy/called.so
solana program deploy target/deploy/caller.so
```

> **Важно:** после деплоя замените переменные для `pubkey` смартконтрактов в тестах на актуальные адреса, которые вы получите после деплоя.

4. **Запуск тестов**

Для простоты можно запустить все тесты:

```bash
cargo test -- --nocapture
```
