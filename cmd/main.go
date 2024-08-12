package main

import (
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"

	"github.com/edmaputra/ilm/config"
	"github.com/edmaputra/ilm/internal/controller/project"
	"github.com/edmaputra/ilm/internal/db"
	httpHandler "github.com/edmaputra/ilm/internal/handler/http"

	"github.com/edmaputra/ilm/internal/repository/database"
)

const COMMON_API_PREFIX = "/api/v1"

func main() {
	config.LoadConfig()

	db.InitDB()
	defer db.CloseDB()

	// Handle graceful shutdown
	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt, syscall.SIGTERM)
	go func() {
		<-c
		log.Println("Shutting down...")
		db.CloseDB()
		os.Exit(0)
	}()

	log.Println("Service start. Listening to port 10001...")

	repo := database.New(db.DB)
	controller := project.New(repo)
	h := httpHandler.New(controller)

	http.Handle(COMMON_API_PREFIX+"/projects", http.HandlerFunc(h.GetProject))

	if err := http.ListenAndServe(":10001", nil); err != nil {
		panic(err)
	}

}
