package db

import (
	"log"
	"time"

	"github.com/edmaputra/ilm/config"
	"github.com/jmoiron/sqlx"
	_ "github.com/lib/pq"
)

var DB *sqlx.DB

func InitDB() {
	dsn := config.AppConfig.GetConnectionString()

	var err error
	DB, err = sqlx.Open("postgres", dsn)

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
