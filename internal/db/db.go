package db

import (
	"log"
	"time"

	"github.com/edmaputra/ilm/config"
	"github.com/jmoiron/sqlx"
)

var DB *sqlx.DB

func InitDB() {
	dsn := config.AppConfig.GetConnectionString()

	DB, err := sqlx.Connect("postgres", dsn)

	if err != nil {
		log.Fatalf("Failed connect to database: %v", err)
	}

	DB.SetMaxOpenConns(3)
	DB.SetMaxIdleConns(3)
	DB.SetConnMaxLifetime(5 * time.Minute)
}

func CloseDB() {
	if err := DB.Close(); err != nil {
		log.Fatalf("Failed close the database: %v", err)
	}
}
