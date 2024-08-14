package http

import (
	"encoding/json"
	"errors"
	"log"
	"net/http"

	"github.com/edmaputra/ilm/internal/controller/project"
	"github.com/edmaputra/ilm/internal/repository"
	"github.com/gofiber/fiber/v2"
)

type Handler struct {
	controller *project.Controller
}

func New(controller *project.Controller) *Handler {
	return &Handler{controller}
}

func (h *Handler) GetProject(w http.ResponseWriter, req *http.Request) {
	id := req.FormValue("id")
	if id == "" {
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	ctx := req.Context()

	project, err := h.controller.Get(ctx, id)

	if err != nil && errors.Is(err, repository.ErrNotFound) {
		w.WriteHeader(http.StatusNotFound)
		return
	} else if err != nil {
		log.Printf("Repository error: %v\n", err)
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	if err := json.NewEncoder(w).Encode(project); err != nil {
		log.Printf("Encoding the response error: %v\n", err)
	}
}

func (h *Handler) GetOne(c *fiber.Ctx) error {
	id := c.Query("id")

	project, err := h.controller.Get(c.Context(), id)

	if err != nil && errors.Is(err, repository.ErrNotFound) {
		return c.Status(http.StatusInternalServerError).JSON(fiber.Map{"error": "500"})
	} else if err != nil {
		log.Printf("Repository error: %v\n", err)
		return c.Status(http.StatusInternalServerError).JSON(fiber.Map{"error": "500"})
	}

	return c.Status(http.StatusOK).JSON(project)
}
