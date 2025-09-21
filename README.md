# URL Checker

[![Статус сборки](https://img.shields.io/github/actions/workflow/status/dorofi/url-checker/rust.yml?branch=main)](https://github.com/dorofi/url-checker/actions)
[![Лицензия](https://img.shields.io/github/license/dorofi/url-checker)](https://github.com/dorofi/url-checker/blob/main/LICENSE)

`url-checker` — это простой и быстрый асинхронный инструмент на Rust для проверки доступности URL-адресов. Он параллельно отправляет запросы к списку URL, измеряет время ответа, получает HTTP-статус и сохраняет результаты в CSV-файл для дальнейшего анализа.

---

A lightweight and fast asynchronous URL checker written in Rust. It concurrently checks a list of URLs, reports their status and response time, and saves the results to a CSV file.

## ✨ Возможности / Features

*   **Асинхронная проверка**: Использует асинхронную среду выполнения Tokio для одновременной проверки сотен URL-адресов.
*   **Подробные результаты**: Для каждого URL выводится статус-код HTTP, время ответа и возможная ошибка.
*   **Экспорт в CSV**: Все результаты автоматически сохраняются в файл `results.csv`.
*   **Простота использования**: Требуется только текстовый файл со списком URL.
*   **Кроссплатформенность**: Работает на Windows, macOS и Linux.

---

*   **Asynchronous Checking**: Leverages the Tokio async runtime to check hundreds of URLs concurrently.
*   **Detailed Results**: For each URL, it provides the HTTP status code, response time, and any potential errors.
*   **CSV Export**: All results are automatically saved to a `results.csv` file.
*   **Simple to Use**: Just provide a text file with a list of URLs.
*   **Cross-Platform**: Works on Windows, macOS, and Linux.

## 🚀 Установка / Installation

### Предварительные требования / Prerequisites

Для сборки проекта вам понадобится Rust. / You need to have Rust installed to build the project.

### Сборка из исходного кода / Building from Source

1.  Клонируйте репозиторий / Clone the repository:
```bash
git clone https://github.com/dorofi/url-checker.git
```
2.  Перейдите в директорию проекта / Navigate to the project directory:
```bash
cd url-checker
```
3.  Соберите проект в release-режиме / Build the project in release mode:
```bash
cargo build --release
