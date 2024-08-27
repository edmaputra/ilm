package test

import (
	"log"
	"time"

	"github.com/edmaputra/ilm/config"
	"github.com/edmaputra/ilm/internal/controller/project"
	"github.com/edmaputra/ilm/internal/db"
	httpHandler "github.com/edmaputra/ilm/internal/handler/http"
	"github.com/edmaputra/ilm/internal/repository/database"
	"github.com/gofiber/fiber/v2"
)

var app *fiber.App

const COMMON_API_PREFIX = "/api/v1"

func SetupServer() *fiber.App {
	config.LoadConfig()

	db.InitDB()

	repo := database.New(db.DB)
	controller := project.New(repo)

	h := httpHandler.New(controller)

	app = fiber.New()

	app.Get(COMMON_API_PREFIX+"/projects", h.GetOne)

	go func() {
		err := app.Listen(":10001")
		if err != nil {
			log.Printf("Failed to start the server: %v", err)
		}
	}()

	time.Sleep(time.Duration(1) * time.Second)

	return app
}

func TeardownServer() {
	defer db.CloseDB()

	if err := app.Shutdown(); err != nil {
		log.Printf("Error shutting down server: %v", err)
	}
}
