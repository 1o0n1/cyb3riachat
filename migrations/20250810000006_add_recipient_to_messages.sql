-- Add up migration script here
-- Добавляем колонку для ID получателя
ALTER TABLE messages ADD COLUMN recipient_id UUID;

-- Создаем связь с таблицей users, чтобы нельзя было отправить сообщение несуществующему пользователю
ALTER TABLE messages ADD CONSTRAINT fk_recipient
    FOREIGN KEY (recipient_id) REFERENCES users(id) ON DELETE SET NULL;

-- 'Down' миграция для отката
-- Add down migration script here
ALTER TABLE messages DROP CONSTRAINT fk_recipient;
ALTER TABLE messages DROP COLUMN recipient_id;