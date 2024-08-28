package helper_test

import (
	"encoding/json"
	"io"
	"log"
	"os"
)

func ReadJSONFile(path string, data *map[string]interface{}) {
	file, err := os.Open(path)

	if err != nil {
		log.Fatalf("Failed to open JSON file: %v", err)
	}

	defer file.Close()

	byteValue, err := io.ReadAll(file)
	if err != nil {
		log.Fatalf("Failed to read JSON file: %v", err)
	}

	if err := json.Unmarshal(byteValue, data); err != nil {
		log.Fatalf("Failed to unmarshal JSON data: %v", err)
	}
}
