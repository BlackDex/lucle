CREATE TABLE USERS (
        id SERIAL PRIMARY KEY,
        username TEXT NOT NULL,
        password TEXT NOT NULL,
        email TEXT NOT NULL,
        createdAt TEXT NOT NULL,
        modifiedAt TEXT NOT NULL,
        privilege TEXT NOT NULL
      )