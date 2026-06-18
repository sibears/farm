---
title: "feat: per-sploit/per-team metrics and Grafana stack for the farm"
type: feat
date: 2026-06-18
origin: docs/brainstorms/2026-06-18-ctf-metrics-graphs-requirements.md
---

# feat: Метрики по сплойтам/командам и стек Grafana для фермы

## Summary

Добавляем размеченные событийные Prometheus-счётчики (получено `{sploit,team}`,
обработано `{sploit,team,result}`) рядом с существующими gauge, со встроенной
защитой кардинальности. Отдельный `docker-compose.full.yml` поднимает ферму +
Postgres + Prometheus + Grafana с провиженными datasources и дашбордами-как-код.
Матрицу «сплойт × команда» и точные итоги Grafana берёт из Postgres; индексы на
`sploit`/`team` добавляются миграцией.

## Problem Frame

Бэкенд экспортирует только пять глобальных gauge по статусам
(`back/src/application/metrics/service.rs`) — без разрезов по сплойту и команде.
Во время CTF команде нужно в реальном времени видеть, какой сплойт приносит
флаги, у каких команд берём и какие сплойты ломаются (много reject). Сейчас этих
данных в метриках нет, и решения по ходу игры принимаются вслепую (см. origin:
`docs/brainstorms/2026-06-18-ctf-metrics-graphs-requirements.md`).

---

## Requirements

**Сбор метрик (бэкенд)**

- R1. Событийный счётчик «получено флагов» с метками `sploit`, `team`,
  инкремент по фактически вставленным флагам (с учётом дедупа).
- R2. Событийный счётчик «обработано флагов» с метками `sploit`, `team`,
  `result` (`accepted`/`rejected`/`skipped`), инкремент в момент терминального
  перехода статуса.
- R3. Существующие 5 глобальных gauge сохраняются; бэклог (`queued`/`waiting`)
  остаётся общим current-state gauge без разрезов.

**Кардинальность и метки**

- R4. Метки `sploit`/`team` нормализуются: пусто → `"unknown"`; при превышении
  капа уникальных значений новые схлопываются в `"other"`.

**Стек и развёртывание**

- R5. Новый `docker-compose.full.yml` поднимает полный стек (ферма, Postgres,
  Prometheus, Grafana). Базовый `docker-compose.yml` не меняется.
- R6. Prometheus скрейпит `/metrics` бэкенда; Grafana стартует с провиженными
  datasources (Prometheus + PostgreSQL) и дашбордами.
- R7. Grafana подключается к Postgres под отдельным read-only пользователем.

**Дашборды**

- R8. Панели: получено по сплойту (rate), % сдачи и accepted/rejected по
  сплойту, разрез по командам, общий обзор (бэклог из существующих gauge
  `flags_queued`/`flags_waiting`, суммарные rate'ы).
- R9. Матрица `sploit × team` — панель на PostgreSQL datasource (SQL по флагам с
  фильтром по времени), не через метки Prometheus.
- R10. Абсолютные «итого получено/сдано» — на PostgreSQL datasource; live-rate'ы
  — на Prometheus.
- R11. Дашборды и datasources провижинятся как код и поднимаются автоматически.

**Производительность**

- R12. Миграция добавляет индексы на `sploit` и `team`, чтобы `GROUP BY`/фильтры
  SQL-панелей не деградировали.

**Безопасность и экспозиция** (добавлено по итогам ревью документа)

- R13. `/metrics`, Prometheus и Grafana не выставляются шире необходимого.
  Prometheus наружу не публикуется (доступ только из сети compose); посадка
  портов Grafana и `/metrics` — по выбранной в Open Questions постуре. Цель —
  не отдавать оперативную CTF-разведку (какие сплойты/команды работают)
  противнику.
- R14. Grafana не стартует с дефолтными `admin/admin`: пароль админа задаётся
  через env. Пароль `grafana_ro` подставляется в datasource через env-переменную
  и не коммитится (`.env` в `.gitignore`).
- R15. SQL-панели используют параметризацию/макросы Grafana (`$__timeFilter` и
  т.п.); клиентские `sploit`/`team` участвуют только как значения столбцов в
  `GROUP BY`, не интерполируются как сырой SQL.

---

## Key Technical Decisions

- KTD1. Событийные `CounterVec`, а не gauge-снимки из БД, для новых разрезов.
  `rate()` требует монотонных счётчиков; сброс при рестарте допустим (`rate()`
  его переживает, абсолютные итоги берём из Postgres). Существующие gauge
  не трогаем — они остаются источником current-state бэклога.
- KTD2. Инкремент счётчиков на уровне контроллеров (как уже сделано для gauge:
  `metrics_service` в state, вызов после операции). Сервисы возвращают
  затронутые флаги, контроллеры по ним инкрементят. Это по существующей
  конвенции и не связывает доменные сервисы с метриками. Граница: учитываются
  только переходы, проходящие через контроллеры `post_flag(s)` (получено) и
  `update_flags_from_sending` (обработано). Python-сендер опрашивает
  `/api/get_sending_flags` в цикле (`flag_senders/farm/http_client.py`), что
  гонит `QUEUED → WAITING` — это намеренно НЕ считается (не терминальный переход
  и не «получено»); см. U3.
- KTD3. Точность «получено» под дедупом (R1): путь сохранения возвращает реально
  вставленные элементы как `Vec<SaveFlag>` (не `Vec<Flag>` — у `Flag` есть
  DB-`id`, которого нет без `RETURNING`). Postgres-цикл уже знает
  `rows_affected()` по каждому флагу и набирает вставленные `SaveFlag` без
  изменения SQL и без `RETURNING` → регенерация `.sqlx` не нужна (безусловно при
  этом подходе). `SaveFlag` несёт `sploit`/`team`, которых достаточно для меток.
- KTD4. Защита кардинальности в слое метрик: пусто → `"unknown"`; на каждую
  метку — своё множество «виденных» значений с капом, при переполнении →
  `"other"`. Капы для `sploit` и `team` раздельные. Рекомендуемый диапазон капа
  50–500 (дефолт 200); `team` берётся из статического `config.json` → его
  кардинальность низкая, кап в первую очередь страхует `sploit`. Состояние
  множеств — через внутреннюю мутабельность (`Arc<Mutex<HashSet<String>>>`),
  т.к. сервис в Rocket делится как `&State`; поля `Arc<Mutex<...>>` совместимы с
  `#[derive(Clone)]`.
- KTD5. Гибрид datasource: Prometheus — низкокардинальные live-rate'ы по
  сплойту/команде/статусу; PostgreSQL datasource — матрица `sploit × team` и
  кумулятивные итоги (origin: Key Decisions).
- KTD6. `docker-compose.full.yml` — самостоятельный файл в корне, поднимающий
  весь стек, а не overlay. Так не возникает коллизии с существующим
  `tests/docker-compose.full.yml` (тестовый, без prom/grafana).
- KTD7. Read-only роль Grafana создаётся init-скриптом в
  `/docker-entrypoint-initdb.d/` контейнера Postgres (`SELECT` на `flags`).
  Caveat: init-скрипты выполняются только на чистом томе → на существующем томе
  роль не создастся (см. Risks с готовым fallback-`GRANT`).
- KTD8. Значение метки `result` приводится к нижнему регистру явно
  (`status.to_string().to_lowercase()`): `Display` у `FlagStatus` (strum, без
  `serialize_all`) отдаёт ВЕРХНИЙ регистр (`ACCEPTED`/`REJECTED`/`SKIPPED`) —
  подтверждается `back/src/presentation/flags/controllers.rs:119`, где
  `status.to_string()` уже даёт верхний регистр. Значения меток должны совпадать
  с PromQL дашбордов (`accepted`/`rejected`/`skipped`).
- KTD9. Экспозиция сервисов задаётся осознанно (R13–R15): Prometheus не
  публикуется наружу; Grafana с заданным админ-паролем; креды — через env.
  Конкретная постура `/metrics` и Grafana (только localhost vs токен) — в Open
  Questions.

---

## High-Level Technical Design

Точки инкремента в жизненном цикле флага и разделение datasource'ов:

```mermaid
flowchart TB
  senders["flag_senders (Python)"] -->|POST /api/flag(s)| ctrlIn["flags controller"]
  ctrlIn -->|save_flags| svc["FlagService.save_flags"]
  svc -->|inserted SaveFlag| ctrlIn
  ctrlIn -->|record_received{sploit,team}| metrics["FlagMetricsService<br/>CounterVec + cap-guard"]

  senders -.->|poll /api/get_sending_flags<br/>QUEUED->WAITING: НЕ считаем| ctrlSend
  ctrlSend["sending controller"] -->|update_flags_from_sending| send["SendingService<br/>WAITING -> ACCEPTED/REJECTED/SKIPPED"]
  send -->|resolved flags| ctrlSend
  ctrlSend -->|record_processed{sploit,team,result}| metrics

  metrics -->|/metrics| prom["Prometheus<br/>scrape (внутр. сеть)"]
  prom --> graf["Grafana"]
  pg[("Postgres flags<br/>(idx sploit, team)")] -->|read-only SQL| graf

  graf --> dRate["Live rate / % сдачи<br/>(Prometheus)"]
  graf --> dMatrix["Матрица sploit x team + итоги<br/>(Postgres)"]
```

Диаграмма — направляющая, не спецификация: именование методов и точная форма
возвращаемых значений уточняются при реализации.

---

## Implementation Units

### U1. Размеченные счётчики и защита кардинальности

- Goal: добавить в `FlagMetricsService` два `CounterVec` и guard меток, не
  затрагивая существующие gauge.
- Requirements: R1, R2, R3, R4.
- Dependencies: none.
- Files: `back/src/application/metrics/service.rs`.
- Approach: завести `flags_received_total{sploit,team}` и
  `flags_processed_total{sploit,team,result}` через тот же путь, что и gauge
  (`rocket_prometheus::prometheus::{CounterVec, Opts}`, регистрация в
  `prometheus.registry()`). Добавить раздельные множества «виденных» значений
  под `Arc<Mutex<HashSet<String>>>` и helper нормализации (пусто → `"unknown"`,
  переполнение капа → `"other"`). `result` приводить к нижнему регистру явно
  (KTD8). Публичные методы `record_received(&[SaveFlag])` и
  `record_processed(&[Flag])` инкрементят счётчики через нормализованные метки.
  Структура остаётся `Clone` + `Send + Sync`, хранится в Rocket как `&State`.
- Patterns to follow: существующая регистрация gauge в этом же файле.
- Test scenarios:
  - `record_received` дважды с одной парой `{sploit,team}` → значение счётчика
    для пары == 2 (happy).
  - Флаг с `sploit = None` → метка `sploit="unknown"`. Covers AE3.
  - Подать кап+1 различных значений `sploit` → значения сверх капа учитываются
    под `"other"`, число серий ограничено. Covers AE4.
  - `record_processed` для accepted и rejected одного сплойта →
    `flags_processed_total{result="accepted"}==1` и `{result="rejected"}==1`;
    значения меток в нижнем регистре. Covers AE2.
  - Кап `team` независим от капа `sploit` (edge).
  - Существующие 5 gauge по-прежнему зарегистрированы (smoke).
- Verification: новые серии видны в `/metrics`; значения `result` —
  `accepted`/`rejected`/`skipped`; gauge не изменили поведение.

### U2. Возврат реально вставленных флагов из пути сохранения

- Goal: дать вызывающему коду знать, какие флаги вставлены, для точного «получено»
  под дедупом.
- Requirements: R1.
- Dependencies: none.
- Files: `back/src/domain/flags/repository.rs`,
  `back/src/infrastructure/flags/postgres_repository.rs`,
  `back/src/infrastructure/flags/inmemory_repository.rs`,
  `back/src/application/flags/service.rs`,
  `back/src/infrastructure/flags/mod.rs` (тесты).
- Approach: изменить контракт `FlagRepo::save` с `Result<usize>` на
  `Result<Vec<SaveFlag>>` (реально вставленные). Postgres набирает их в цикле по
  `rows_affected() == 1` (SQL и `ON CONFLICT DO NOTHING` без изменений → `.sqlx`
  не регенерим). In-memory набирает вставленные там, где сейчас считает
  `inserted_count`. `FlagService::save_flags` возвращает вставленные вызывающему;
  число для JSON-ответа контроллеров получается как `.len()` (контракт ответа не
  меняется). Изменение контракта компиляционно ломает существующие тесты,
  сравнивающие счётчик-`usize`, — мигрировать их на `.len()` в том же коммите,
  чтобы билд оставался зелёным.
- Patterns to follow: текущий цикл сохранения в postgres-реализации.
- Test scenarios:
  - `save_flags([dup, dup])` → длина вставленных == 1. Covers AE1.
  - `save_flags` двух различных → длина == 2 (happy).
  - Повторное сохранение ранее сохранённого → длина == 0 (edge).
  - Мигрировать `save_flags_ignores_duplicate_flags` и тесты в
    `infrastructure/flags/mod.rs` на проверку через `.len()` (компиляция).
- Verification: контракт компилируется во всех реализациях и тестах; число
  вставленных в JSON-ответе контроллеров не изменилось.

### U3. Возврат терминальных флагов из sending и проводка инкрементов

- Goal: инкрементить «получено» и «обработано» на контроллерах по реально
  затронутым флагам.
- Requirements: R1, R2.
- Dependencies: U1, U2.
- Files: `back/src/application/sending/service.rs`,
  `back/src/presentation/sending/controllers.rs`,
  `back/src/presentation/flags/controllers.rs`,
  `back/src/infrastructure/flags/inmemory_repository.rs`.
- Approach: `SendingService::update_flags_from_sending` меняет сигнатуру с
  `Result<(), _>` на `Result<Vec<Flag>, SendingServiceError>` и возвращает
  `flags_to_update` (только перешедшие из `WAITING`). Sending-контроллер (сейчас
  игнорирует возврат) по результату вызывает `record_processed`;
  flags-контроллеры (`post_flag`, `post_flags`) по вставленным из U2 вызывают
  `record_received`, продолжая обновлять gauge как сейчас. Переход
  `QUEUED → WAITING` в `get_flags_for_senders` намеренно не инкрементит счётчики
  (KTD2) — зафиксировать это явным тестом-границей. Реализовать
  `InMemoryFlagRepository::update` (сейчас `todo!()`) в этом же юните: без него
  сервисный тонкий тест ниже падает в `todo!()`. Все правки
  `inmemory_repository.rs` для U2 (`save`) и U3 (`update`) координировать, чтобы
  не конфликтовать на одном файле.
- Patterns to follow: существующий вызов `metrics_service.update_flags_count` в
  тех же контроллерах.
- Test scenarios:
  - `update_flags_from_sending` возвращает только бывшие `WAITING` флаги с новым
    статусом (happy). Covers AE2 (через последующий `record_processed`).
  - Флаги не в `WAITING` не обновляются и не возвращаются (edge).
  - `InMemoryFlagRepository::update` меняет статус существующего флага и не падает
    на отсутствующем id согласно контракту (integration).
  - `get_flags_for_senders` (`QUEUED → WAITING`) не вызывает `record_*`
    (граница). Covers AE1 косвенно (received только на `post_flag(s)`).
- Verification: после резолва пачки sending счётчик `flags_processed_total`
  растёт по соответствующим `result`; после `post_flag(s)` растёт
  `flags_received_total` (Covers AE1, AE2 — проводку подтвердить ревью при
  отсутствии Rocket-интеграционного теста).

### U4. Миграция индексов на sploit/team

- Goal: ускорить SQL-панели матрицы и итогов.
- Requirements: R12.
- Dependencies: none.
- Files: `back/migrations/0003_add_sploit_team_indices.sql`.
- Approach: `CREATE INDEX IF NOT EXISTS` по `flags(sploit)` и `flags(team)`
  (без `CONCURRENTLY` — миграции идут в транзакции мигратора). Имя файла по
  существующей конвенции `000N_*.sql` (следующий — `0003`).
- Patterns to follow: `back/migrations/0002_create_flags_table.sql`.
- Test scenarios: Test expectation: none — изменение схемы.
- Verification: мигратор применяет файл на старте; индексы присутствуют в схеме.

### U5. docker-compose.full.yml и конфиг Prometheus

- Goal: один файл поднимает весь стек, Prometheus скрейпит бэкенд.
- Requirements: R5, R6, R13.
- Dependencies: U1 (метрики), U4 (индексы — желательны до прод-нагрузки).
- Files: `docker-compose.full.yml`, `prometheus/prometheus.yml`.
- Approach: самостоятельный compose в корне, повторяющий сервисы базового
  (`sibears_farm_back`, `sibears_farm_front`, `postgres`, при необходимости
  `external_redis`/swagger) плюс `prometheus` и `grafana`. `prometheus.yml`
  задаёт scrape-job на `sibears_farm_back:8777/metrics`. Prometheus наружу не
  публикуется (без host-проброса порта); постура `/metrics`/Grafana — по Open
  Questions. Не модифицировать базовый `docker-compose.yml`. Заголовком файла
  отметить, что он и `tests/docker-compose.full.yml` нельзя поднимать
  одновременно (одинаковые порты 8777/8776/5432/6378/8780).
- Patterns to follow: сервис/сеть/volume-стиль `docker-compose.yml`.
- Test scenarios: Test expectation: none — инфраструктура.
- Verification: `docker compose -f docker-compose.full.yml config` валиден; стек
  стартует; target бэкенда в Prometheus в состоянии UP; Prometheus не доступен с
  хоста.

### U6. Провижининг Grafana и read-only роль Postgres

- Goal: Grafana поднимается с обоими datasources, заданным админ-паролем и
  доступом только на чтение к БД.
- Requirements: R6, R7, R11, R14.
- Dependencies: U5.
- Files: `grafana/provisioning/datasources/datasources.yml`,
  `grafana/provisioning/dashboards/dashboards.yml`,
  `postgres/initdb/10_grafana_ro.sql`.
- Approach: datasource-провижининг для Prometheus и PostgreSQL (PostgreSQL
  ссылается на внутренний хост `postgres:5432` и read-only учётку; пароль через
  `${...}` env-интерполяцию, не литералом). `GF_SECURITY_ADMIN_PASSWORD` в env
  Grafana (из `.env`, в `.gitignore`). Init-скрипт создаёт роль `grafana_ro` и
  выдаёт `SELECT` на `flags` (+ `USAGE` на схему; доступ к типу `flag_status`
  при необходимости каста). Dashboard-провайдер указывает каталог с JSON.
- Patterns to follow: стандартная раскладка `grafana/provisioning/{datasources,dashboards}`.
- Test scenarios: Test expectation: none — провижининг.
- Verification: оба datasource «healthy»; Grafana не пускает по `admin/admin`;
  `grafana_ro` делает `SELECT`, но не пишет.

### U7. Дашборды-как-код

- Goal: готовые панели по R8–R10.
- Requirements: R8, R9, R10, R11, R15.
- Dependencies: U1, U6.
- Files: `grafana/dashboards/ctf-overview.json` (один или несколько JSON).
- Approach: панели на Prometheus — получено по сплойту (`rate`), accepted/
  rejected и % сдачи по сплойту (`flags_processed_total{result="accepted"}`),
  разрез по командам, общий обзор с бэклогом из существующих gauge
  (`flags_queued`, `flags_waiting`). Панели на PostgreSQL — матрица
  `sploit × team` (table/heatmap, `GROUP BY sploit, team` с `$__timeFilter`) и
  кумулятивные итоги; только параметризация/макросы, без сырой интерполяции
  клиентских строк (R15). Конкретные запросы/PromQL — направляющие.
- Patterns to follow: экспортируемый JSON-формат дашбордов Grafana.
- Test scenarios: Test expectation: none — конфигурация дашбордов.
- Verification: дашборды загружаются; rate-панели используют Prometheus, матрица
  и итоги — PostgreSQL; панели отрисовывают данные.

---

## Acceptance Examples

- AE1. Дубль в батче считается один раз. Дано: `save_flags([dup, dup])`. Тогда:
  вставлен 1 → `flags_received_total` для метки растёт на 1. Covers U1, U2, U3.
- AE2. Переход `WAITING → REJECTED`. Тогда: `flags_processed_total{result="rejected"}`
  для меток флага +1; % сдачи считается как `accepted/(accepted+rejected)`.
  Covers U1, U3.
- AE3. Флаг с пустым `sploit`. Тогда: метрика пишется с `sploit="unknown"`.
  Covers U1.
- AE4. Уникальных `sploit` больше капа. Тогда: новые значения идут в `"other"`,
  рост числа серий Prometheus останавливается. Covers U1.

---

## Scope Boundaries

- Алертинг (Alertmanager) и нотификации — не входят.
- Строгий реестр/валидация имён сплойтов — не входит (кап достаточно).
- Изменения модели данных флага — не требуются (`sploit`/`team` уже есть).
- Удаление/рефакторинг существующих gauge — не трогаем.
- TLS и SSO/внешняя аутентификация Grafana — не входят (базовый админ-пароль и
  ограничение экспозиции — входят, R13–R14).

### Deferred to Follow-Up Work

- Разрез бэклога (`queued`/`waiting`) по сплойту/команде — пока общий gauge.
- Перенос инкрементов в доменные сервисы, если появится не-HTTP вызыватель.

---

## Risks & Dependencies

- `FlagStatus::to_string()` даёт верхний регистр (strum `Display` без
  `serialize_all`) — без явного `.to_lowercase()` метка `result` не совпадёт с
  PromQL и панели по `result` молча пусты (KTD8, U1).
- Init-скрипт read-only роли выполняется только на чистом томе Postgres. На
  существующем томе роль не создастся, SQL-панели падают на аутентификации.
  Fallback (в README/Operational), идемпотентно:
  `CREATE ROLE grafana_ro LOGIN PASSWORD '...' NOSUPERUSER NOCREATEDB NOCREATEROLE;`
  `GRANT CONNECT ON DATABASE flags TO grafana_ro;`
  `GRANT USAGE ON SCHEMA public TO grafana_ro;`
  `GRANT SELECT ON flags TO grafana_ro;`
- Кардинальность: `flags_processed_total` = sploit × team × result. Кап
  ограничивает рост, но в худшем случае до срабатывания капа серий много;
  при первом схлопывании нового сплойта в `"other"` будет разовый всплеск
  `rate()` на серии `"other"` — отметить в описании панели как агрегат.
- Сброс счётчиков при рестарте бэкенда: `rate()` переживает, абсолютные итоги
  берём из Postgres (KTD1, R10) — расхождений на дашбордах быть не должно.
- Контракт `save` → `Vec<SaveFlag>` (U2) ломает компиляцию существующих тестов
  до их миграции — обновлять атомарно в одном коммите. SQL не меняется → `.sqlx`
  не регенерим.
- `InMemoryFlagRepository.update` сейчас `todo!()` — реализуется в U3.
  `get_by_status` тоже `todo!()`: если тест пойдёт через `get_waiting_flags`,
  он упадёт раньше — тесты строить так, чтобы не дёргать незаполненные методы,
  либо реализовать `get_by_status` по необходимости.
- Безопасность: `/metrics` без auth и Grafana/Prometheus на `0.0.0.0` отдают
  CTF-разведку противнику; дефолтные `admin/admin` Grafana и плейнтекст-креды в
  provisioning — векторы утечки `flags`. Закрывается R13–R15; постура — Open
  Questions.
- Зависимости уже в проекте: `rocket_prometheus` 0.10.1 (re-export `prometheus`
  0.13.4), Postgres, Docker; новых крейтов не требуется.

---

## Open Questions

**Resolve before/at implementation**

- Постура экспозиции `/metrics` и Grafana: только localhost (`127.0.0.1:...`) на
  хосте команды, или доступ по токену/обратному прокси? По умолчанию —
  только localhost; Prometheus наружу не публикуем (R13).
- Точное значение капа кардинальности в выбранном диапазоне 50–500 (KTD4).

**Deferred to implementation**

- Путь статуса `skipped`: подтвердить, что `SKIPPED` приходит через
  `update_flags_from_sending` (ответ чек-системы у сендера), чтобы
  `record_processed` его покрывал; иначе `result="skipped"` не будет
  инкрементиться.
- Нужен ли `grafana_ro` доступ к типу `flag_status` для каста в SQL-панелях.

---

## Sources & Research

- `back/src/application/metrics/service.rs` — текущие 5 gauge, паттерн регистрации.
- `back/src/main.rs:57-87` — `PrometheusMetrics::new`, `.manage(metrics_service)`,
  `mount("/metrics", ...)`.
- `back/src/application/sending/service.rs:54-81` — единственная точка
  терминального перехода статуса; `:26-34` — `get_flags_for_senders`
  (`QUEUED → WAITING`).
- `back/src/presentation/flags/controllers.rs` (в т.ч. `:119` —
  `status.to_string()` отдаёт верхний регистр),
  `back/src/presentation/sending/controllers.rs` — текущие вызовы метрик.
- `back/src/domain/flags/entities.rs:97-110` — `FlagStatus` (strum `Display`,
  sqlx `rename_all="lowercase"`).
- `back/src/domain/flags/repository.rs`, `back/src/infrastructure/flags/*` —
  контракт `save`/`update` и реализации (`update`/`get_by_status` — `todo!()` в
  in-memory).
- `back/migrations/0001_*.sql`, `0002_*.sql` — конвенция миграций.
- `docker-compose.yml`, `tests/docker-compose.full.yml` — состав сервисов,
  совпадающие порты и коллизия имени full-compose.
- `flag_senders/farm/http_client.py` — опрос `/api/get_sending_flags`.
- Grounding dossier: `/tmp/compound-engineering/ce-brainstorm/metrics-graphs/grounding.md`.
