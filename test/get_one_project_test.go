package maintest_test

import (
	"encoding/json"
	"log"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/edmaputra/ilm/internal/controller/project"
	"github.com/edmaputra/ilm/internal/db"
	httpHandler "github.com/edmaputra/ilm/internal/handler/http"
	"github.com/gofiber/fiber/v2"
	"github.com/stretchr/testify/assert"

	"github.com/edmaputra/ilm/internal/repository/database"
)

const COMMON_API_PREFIX = "/api/v1"

func TestGetOneProjectById(t *testing.T) {
	LoadConfig()

	db.InitDB()
	defer db.CloseDB()

	repo := database.New(db.DB)
	controller := project.New(repo)

	h := httpHandler.New(controller)

	app := fiber.New()

	db.InitDB()

	app.Get(COMMON_API_PREFIX+"/projects", h.GetOne)

	go func() {
		err := app.Listen(":10001")
		if err != nil {
			log.Printf("Failed to start the server: %v", err)
		}
	}()

	time.Sleep(time.Duration(1) * time.Second)

	resp, err := app.Test(httptest.NewRequest("GET", "/api/v1/projects?id=1", nil))
	if err != nil {
		log.Printf("Failed to create request: %v", err)
	}

	assert.Equal(t, resp.StatusCode, http.StatusOK)

	var resBody map[string]interface{}

	json.NewDecoder(resp.Body).Decode(&resBody)

	assert.Equal(t, "1", resBody["id"])

	app.Shutdown()

}
