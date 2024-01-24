package app

import (
	"encoding/json"
	"fmt"
	"net/http"
)

type AppResponse struct {
	Message string `json:"message"`
}

func InitApp() {
	http.HandleFunc("/", appHandler)
}

func appHandler(response http.ResponseWriter, request *http.Request) {
	fmt.Println(request.Method)

	payload, err := json.Marshal(AppResponse{
		Message: "Hello, World!",
	})

	if err != nil {
		response.WriteHeader(500)
		response.Write([]byte("Failed to encode response as JSON"))
		return
	}

	response.Header().Add("Content-Type", "application/json")
	response.Write(payload)
}
