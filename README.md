# FreshProductBackend
Серверная часть приложения FreshProduct

### Запуск:
  - Скачайте репозиторий
```bash
git clone git@github.com:Tardimgg/FreshProductBackend.git && cd FreshProductBackend
```
  - Создайте аккаунт https://heroku.com
  - Войдите в аккаунт
```bash
heroku login
```
  - Свяжите аккаунт heroku с git репозиторием
```bash
heroku git:remote -a fresh-product
```
  - Добавьте пакет сборки https://github.com/emk/heroku-buildpack-rust 
  - Загрузите приложение на сервер
```bash
git push heroku master
``` 
