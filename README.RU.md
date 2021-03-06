# WSZL

Добавляет поддержку Web-сценариев для механизма Zabbix [Low Level Discovery](https://www.zabbix.com/documentation/current/manual/discovery/low_level_discovery).

## С чего начать

### Установка

1. Установите утилиту [site discovery flea](https://github.com/lebe-dev/site-discovery-flea)  
   Она обеспечивает низкоуровневое обнаружение для виртуальных хостов из nginx\apache.
2. Скопируйте исполняемый файл `wszl` в `/etc/zabbix` на Zabbix-сервере
3. Установите право на исполнение:
    ```shell script
    chmod +x /etc/zabbix/wszl
    ```
4. Создайте файл конфигурации `/etc/zabbix/wszl.yml`:
    ```shell script
    cp wszl.yml-example /etc/zabbix/wszl.yml
    ```
    Отредактируйте файл, укажите имя пользователя и пароль для доступа к Zabbix API.
   
    Обновите права:
    ```shell script
    chmod o-rwx /etc/zabbix/wszl.yml
    chown zabbix: /etc/zabbix
    ```
    
5. Добавьте задачу в планировщик cron:
    Каждые 30 минут утилита будет создавать Web-сценарии и триггеры для обнаруженных элементов.
    ```
    */30 * * * * /etc/zabbix/wszl gen
    ```   

### Использование

#### Создание Web-сценариев и триггеров

```
$ wszl gen
```

#### Конфигурация

Файл `wszl.yml`.

## Как работает утилита

1. Утилита ищет элементы по маске vhost.item через Zabbix API. Маску можно переопределить опцией.
2. Затем создает вэб-сценарии и триггеры
    - Параметры Web-сценария:
      - Заголовок вида: "Check index page 'XYZ'"
      - Ожидаемый код ответа: 200
    - Параметры триггера: 
      - Уровень приоритета - High (4), 
      - Заголовок вида: 'Site XYZ is unavailable'
      - Выражение: `web.test.fail`  

## Решение проблем

Подробный лог `wszl.log`.

Вы можете менять уровни логирования с помощью опции `--log-level`.

## Roadmap

- Control item search mask with command option
- Control web scenario parameters with command options (title, return code, etc.)
- Control trigger parameters with command options (title, severity, description, etc.)
- Remove generated items
