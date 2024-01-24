package main

import (
	"fmt"
	"net/http"
	"server/app"
)

func main() {
	initServer()

	err := http.ListenAndServe(":8000", nil)
	if err != nil {
		panic(fmt.Errorf("cannot start server at port 8000 with error %s", err))
	}

	fmt.Println("Server listening at http://localhost:8000/")
}

func initServer() {
	app.InitApp()
}
