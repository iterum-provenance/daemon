-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "dataset" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT UNIQUE NOT NULL,
    path TEXT NOT NULL,
    backend ENUM(Local, Google, Amazon) NOT NULL,
    description TEXT NOT NULL,
    head FOREIGN KEY

);

    -- pub name: String,
    -- pub path: String,
    -- pub backend: Backend,
    -- pub description: String,
    -- pub head: String,