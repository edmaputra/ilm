package server

import (
	"log"

	"github.com/edmaputra/ilm/config"
	projectCtrl "github.com/edmaputra/ilm/internal/controller/project"
	"github.com/edmaputra/ilm/internal/db"
	projectHttp "github.com/edmaputra/ilm/internal/handler/http/project"
	projectDb "github.com/edmaputra/ilm/internal/repository/database/project"
	"github.com/gofiber/fiber/v2"
)

var app *fiber.App

const COMMON_API_PREFIX = "/api/v1"

func Setup() *fiber.App {
	config.LoadConfig()

	db.InitDB()

	projectHandler := projectHttp.New(projectCtrl.New(projectDb.New(db.DB)))

	app = fiber.New()

	app.Get(COMMON_API_PREFIX+"/projects", projectHandler.GetOne)

	return app
}

func Teardown() {
	defer db.CloseDB()

	if err := app.Shutdown(); err != nil {
		log.Printf("Error shutting down server: %v", err)
	}
}
