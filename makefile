.PHONY: all test dev prod

all: test prod

test:
	GO_ENV=test go test ./test/... -v;

dev:	
	GO_ENV=dev go run ./cmd/main.go

prod:
	GO_ENV= go run main.go
