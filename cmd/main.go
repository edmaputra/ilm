package main

import (
	"log"
	"os"
	"os/signal"
	"syscall"

	"github.com/edmaputra/ilm/internal/server"
)

const COMMON_API_PREFIX = "/api/v1"

func main() {
	// Handle graceful shutdown
	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt, syscall.SIGTERM)
	go func() {
		<-c
		log.Println("Shutting down...")
		server.Teardown()
		os.Exit(0)
	}()

	app := server.Setup()

	if err := app.Listen(":10001"); err != nil {
		log.Fatalf("Failed to start server: %v", err)
	}
}
