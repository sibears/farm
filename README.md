# SiBears_Farm 

## Использование

### Собраный докер

В гитлабе автоматически собираются докер образы
с беком и фронтом для фермы

Доступны по адресам:
 - registry.gitlab.com/sibears/infrastructure/sibears_farm:main
 - registry.gitlab.com/sibears/infrastructure/sibears-farm-front:main

*Или можно взять конкретную версию из gitlab*

Может потребоваться авторизоваться в gitlab registry из докера, т.к. образы приватные

Пример типичного **docker-compose.yml** для поднятия фермы, 
используя готовые образы

```yaml
version: '3'

services:
  farm_back:
    image: registry.gitlab.com/sibears/infrastructure/sibears_farm:main
    ports:
      - "8777:8777"
    depends_on:
      - postgres
    environment:
      - DATABASE_URL=postgres://postgres:sibears1cool@postgres/flags
      - ROCKET_LOG_LEVEL=normal
    volumes:
      - ./config.json:/srv/config.json:ro
      - ./start_sploit.py:/srv/start_sploit.py:ro
  
  farm_front:
    image: registry.gitlab.com/sibears/infrastructure/sibears-farm-front:main
    ports:
      - "8776:80"
  
  postgres:
    image: postgres:14.1-alpine
    environment:
      - POSTGRES_PASSWORD=sibears1cool
      - PGDATA=/var/lib/postgresql/data/pgdata
    volumes:
      - ./vol/external_db:/var/lib/postgresql/data/pgdata
  
  external_redis:
    image: redis:7.2-alpine
    ports:
      - "6378:6379"
    restart: unless-stopped
    command: ["redis-server", "--appendonly", "yes", "--requirepass", "sibears1cool"]
    volumes:
      - ./vol/external_redis:/data
```

И **config.json**

```json
{
	"database": {
		"database_url": "postgres://postgres:sibears1cool@postgres/flags"
	},
	"auth": {
		"password": "sibears1cool"
	},
	"ctf": {
		"protocol": {
			"protocol": "forcad_http",
			"team_token": "209732789a5900fb",
			"checksys_host": "forkad.docker.localhost",
			"checksys_port": 80
		},
		"flag_format": "\\w{31}=",
		"flag_lifetime": 300,
		"submit_period": 5,
		"submit_flag_limit": 100,
		"teams": {
			"First": "first.docker.localhost",
			"Second": "second.docker.localhost"
		}
	}
}
```

### Локальная сборка

`docker-compose up -d`

Для тестов или допила фермы проще собирать и запускать без докеров:
 - Поднять БД в докере
 - `cargo run`

Не забыть поменять url-ы в конфигах

### Запуск сплойтов

Скачайте файл **start_sploit.py**,
это модифицированный запускатор из destructive farm.
Модифицированный **start_sploit.py** передаёт в сплойт название команды и регулярку в argv 2 и 3 соответственно.
Также выводит статистику сколько удачных и неудачных запусков было за раунд.

Можно скачать с фермы по `/api/start_sploit.py`

### API

Все api описано в openapi, конфиг которого доступен по 
`/v1/openapi.json`. Файл можно импортировать
в insomnia или postman и удобно тыкаться в ферму.

Так же работает swagger по `/swagger-ui`

## Как работает

В целом почти как destructive farm, только на расте. 

## Архитектура

Высокоуровневая модель описана в [docs/design](./docs/design) в формате [c4](https://c4model.com/).
