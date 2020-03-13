-- Your SQL goes here

CREATE TABLE "commit" (
    hash TEXT UNIQUE NOT NULL,
    name TEXT,
    parent TEXT,
    branch TEXT NOT NULL,
    description TEXT,
    deprecated TEXT,
    PRIMARY KEY (hash),
    FOREIGN KEY (parent)
        REFERENCES "commit" (hash)
            ON DELETE CASCADE 
            ON UPDATE NO ACTION
);

CREATE TABLE "branch" (
    branch_hash TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    head TEXT NOT NULL,
    PRIMARY KEY (branch_hash)
    FOREIGN KEY (head)
        REFERENCES "commit" (hash)
            ON DELETE CASCADE 
            ON UPDATE NO ACTION
);

CREATE TABLE "committed_files" (
    hash TEXT NOT NULL,
    file TEXT NOT NULL,
    PRIMARY KEY (hash, file)
    FOREIGN KEY (hash)
        REFERENCES "commit" (hash)
            ON DELETE CASCADE 
            ON UPDATE NO ACTION
    FOREIGN KEY (file)
        REFERENCES "file" (name)
            ON DELETE CASCADE 
            ON UPDATE NO ACTION
);

CREATE TABLE "file" (
    name TEXT UNIQUE NOT NULL,
    PRIMARY KEY (name)
);

CREATE TABLE "commit_diffs" (
    hash TEXT NOT NULL,
    file TEXT NOT NULL,
    operation TEXT NOT NULL,
    replacement TEXT,
    PRIMARY KEY (hash, file, operation)
    FOREIGN KEY (hash)
        REFERENCES "commit" (hash)
            ON DELETE CASCADE 
            ON UPDATE NO ACTION
    FOREIGN KEY (file)
        REFERENCES "file" (name)
            ON DELETE CASCADE 
            ON UPDATE NO ACTION
    FOREIGN KEY (replacement)
        REFERENCES "file" (name)
            ON DELETE CASCADE 
            ON UPDATE NO ACTION
);


CREATE TABLE "dataset" (
    dataset_id INTEGER NOT NULL,
    name TEXT UNIQUE NOT NULL,
    path TEXT NOT NULL,
    backend TEXT NOT NULL,
    description TEXT NOT NULL,
    head TEXT,
    PRIMARY KEY (dataset_id)
    FOREIGN KEY (head)
        REFERENCES "commit" (hash)
            ON DELETE CASCADE 
            ON UPDATE NO ACTION
);



    -- pub name: String,
    -- pub path: String,
    -- pub backend: Backend,
    -- pub description: String,
    -- pub head: String,