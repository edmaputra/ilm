package test

import (
	"encoding/json"
	"log"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/edmaputra/ilm/internal/server"
	"github.com/stretchr/testify/assert"
)

func BasicFlow(m *testing.M) {
	app := server.Setup()
	defer server.Teardown()

	go func() {
		err := app.Listen(":10001")
		if err != nil {
			log.Printf("Failed to start the server: %v", err)
		}
	}()

	time.Sleep(time.Duration(1) * time.Second)

	m.Run()
}

func TestGetOneProjectById(t *testing.T) {
	app := server.Setup()
	defer server.Teardown()

	resp, err := app.Test(httptest.NewRequest("GET", "/api/v1/projects?id=1", nil))
	if err != nil {
		log.Printf("Failed to create request: %v", err)
	}

	assert.Equal(t, resp.StatusCode, http.StatusOK)

	var resBody map[string]interface{}

	json.NewDecoder(resp.Body).Decode(&resBody)

	assert.Equal(t, "1", resBody["id"])
}
