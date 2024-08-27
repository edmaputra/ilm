package server

import (
	"log"

	"github.com/edmaputra/ilm/config"
	"github.com/edmaputra/ilm/internal/controller/project"
	"github.com/edmaputra/ilm/internal/db"
	httpHandler "github.com/edmaputra/ilm/internal/handler/http"
	"github.com/edmaputra/ilm/internal/repository/database"
	"github.com/gofiber/fiber/v2"
)

var app *fiber.App

const COMMON_API_PREFIX = "/api/v1"

func Setup() *fiber.App {
	config.LoadConfig()

	db.InitDB()

	repo := database.New(db.DB)
	controller := project.New(repo)

	h := httpHandler.New(controller)

	app = fiber.New()

	app.Get(COMMON_API_PREFIX+"/projects", h.GetOne)

	return app
}

func Teardown() {
	defer db.CloseDB()

	if err := app.Shutdown(); err != nil {
		log.Printf("Error shutting down server: %v", err)
	}
}
