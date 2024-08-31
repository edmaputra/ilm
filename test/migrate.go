package test

import (
	"log"

	"github.com/edmaputra/ilm/config"

	"database/sql"

	"github.com/golang-migrate/migrate/v4"
	"github.com/golang-migrate/migrate/v4/database/postgres"

	_ "github.com/golang-migrate/migrate/v4/source/file"
	_ "github.com/lib/pq"
)

func MigrateDataUp() {
	m := migrationSetup()

	if err := m.Up(); err != nil {
		log.Fatalf("Failed to migrate up: %v", err)
	}

	log.Println("Migration up successfully")
}

func MigrateDataDown() {
	m := migrationSetup()

	if err := m.Down(); err != nil {
		log.Fatalf("Failed to migrate down: %v", err)
	}

	log.Println("Migration down successfully")
}

func migrationSetup() *migrate.Migrate {
	db, err := sql.Open("postgres", config.AppConfig.GetMigrateConnectionUrl())
	if err != nil {
		log.Fatalf("Failed to open database: %v", err)
	}

	driver, err := postgres.WithInstance(db, &postgres.Config{})
	if err != nil {
		log.Fatalf("Failed to create database driver: %v", err)
	}

	m, err := migrate.NewWithDatabaseInstance("file://migrations", "postgres", driver)
	if err != nil {
		log.Fatalf("Failed to create migrate instance: %v", err)
	}

	return m
}
