package main

import (
	"log"
	"os"
	"os/signal"
	"syscall"

	"github.com/edmaputra/ilm/config"
	"github.com/edmaputra/ilm/internal/controller/project"
	"github.com/edmaputra/ilm/internal/db"
	httpHandler "github.com/edmaputra/ilm/internal/handler/http"
	"github.com/gofiber/fiber/v2"

	"github.com/edmaputra/ilm/internal/repository/database"
)

const COMMON_API_PREFIX = "/api/v1"

func main() {
	// Handle graceful shutdown
	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt, syscall.SIGTERM)
	go func() {
		<-c
		log.Println("Shutting down...")
		db.CloseDB()
		os.Exit(0)
	}()

	SetupServer()
}

func SetupServer() error {
	config.LoadConfig()

	db.InitDB()
	defer db.CloseDB()

	log.Println("Service start. Listening to port 10001...")

	repo := database.New(db.DB)
	controller := project.New(repo)

	h := httpHandler.New(controller)

	app := fiber.New()

	db.InitDB()
	defer db.CloseDB()

	app.Get(COMMON_API_PREFIX+"/projects", h.GetOne)

	return app.Listen(":10001")

}
