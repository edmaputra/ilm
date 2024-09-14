package http

import (
	"errors"
	"fmt"
	"log"
	"net/http"

	"github.com/edmaputra/ilm/internal/controller/project"
	"github.com/edmaputra/ilm/internal/e"
	"github.com/edmaputra/ilm/pkg/httputils"
	"github.com/gofiber/fiber/v2"
)

type Handler struct {
	controller *project.Controller
}

func New(controller *project.Controller) *Handler {
	return &Handler{controller}
}

func (h *Handler) GetOne(c *fiber.Ctx) error {
	id := c.Query("id")

	project, err := h.controller.Get(c.Context(), id)

	if err != nil && errors.Is(err, e.ErrNotFound) {
		return c.Status(http.StatusOK).JSON(httputils.NewJSONErrorResponse(http.StatusNotFound, "NOT_FOUND", fmt.Sprintf(err.Error(), "Project", id)))
	} else if err != nil {
		log.Printf("Repository error: %v\n", err)
		return c.Status(http.StatusInternalServerError).JSON(fiber.Map{"error": "500"})
	}

	response := httputils.NewJSONResponse(http.StatusOK, "OK", project)

	return c.Status(http.StatusOK).JSON(response)
}
