# Flair

Flair is a Rust-based cli tool designed to simplify database migration processes, specifically for PostgreSQL. This project is created primarily for fun and learning purposes. While it offers functional features for managing database migrations, it's important to note that this tool is not intended for production use.

## Features

- **Create Migrations**: Easily generate new migration files for your PostgreSQL database.
- **Migration Status**: Check the current status of all migrations, seeing which have been applied and which are pending.
- **Apply Migrations (Up)**: Apply pending migrations to your database, updating it to the latest version.
- **Revert Migration (Down)**: Revert the last applied migration, allowing for easy rollback of changes.

### Future Features
- **Checksum verification**
- **Up or down specific migration**
