-- Add migration script here

-- Создаем расширение, если оно еще не установлено, для использования UUID
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Создаем таблицу пользователей
CREATE TABLE users (
-- Уникальный идентификатор пользователя, генерируется автоматически
id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

-- Имя пользователя, должно быть уникальным и непустым
username VARCHAR(255) UNIQUE NOT NULL,

-- Хеш пароля. Никогда не храните пароли в открытом виде!
-- Поле может быть NULL, так как у нас будет аутентификация без пароля (passkeys).
password_hash VARCHAR(255),

-- Email, должен быть уникальным.
email VARCHAR(255) UNIQUE NOT NULL,

-- Временные метки
created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Добавляем триггер для автоматического обновления поля updated_at
-- при любом изменении строки. Это очень удобно.
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
NEW.updated_at = NOW();
RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION trigger_set_timestamp();