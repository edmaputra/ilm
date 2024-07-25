package main

import (
	"log"
	"net/http"

	"github.com/edmaputra/ilm/internal/controller/project"
	httpHandler "github.com/edmaputra/ilm/internal/handler/http"
	"github.com/edmaputra/ilm/internal/repository/memory"
)

func main() {
	log.Println("Service start...")

	repo := memory.New()
	controller := project.New(repo)
	h := httpHandler.New(controller)

	http.Handle("/projects", http.HandlerFunc(h.GetProject))

	if err := http.ListenAndServe(":10001", nil); err != nil {
		panic(err)
	}

}
