package main

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func main() {
	router := gin.Default()

	server := http.Server{
		Addr:    "127.0.0.1:8080",
		Handler: router,
	}

	router.GET("/", func(ctx *gin.Context) {
		ctx.JSON(200, gin.H{
			"message": "hello world",
		})
	})

	server.ListenAndServe()
}
