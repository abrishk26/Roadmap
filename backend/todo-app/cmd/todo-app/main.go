package main

import (
	"github.com/gin-gonic/gin"
	"github.com/joho/godotenv"
	"github.com/abrishk26/Roadmap/backend/todo-app/pkg/database"
	"github.com/abrishk26/Roadmap/backend/todo-app/internal/repositories"
	"github.com/abrishk26/Roadmap/backend/todo-app/internal/handlers"
	"fmt"
	"os"
)

func main() {
	err := godotenv.Load()

	if err != nil {
		fmt.Println("Error loading environment varialbe")
		return
	}

	dns, stat := os.LookupEnv("DATABASE_URL")

	if !stat {
		fmt.Println("env variable DATABASE_URL must be set.")
		return
	}

	db, err := database.InitDB(dns)
	
	if err != nil {
		fmt.Println("Unable to connect to the database.")
		return
	}
	userRepo := repository.UserRepository{ Db: db}
	
	
	

	r := gin.Default()
	
	r.LoadHTMLGlob("web/templates/*")
	r.GET("/ping", func(c *gin.Context) {
		c.HTML(200, "login.html", gin.H{
			"Name" : "Abreham",
			"Age" : "21",
		})
	})
	
	r.GET("/", func(c *gin.Context) {
		c.HTML(200, "signup.html", nil)
	})
	
	r.POST("/register", handlers.RegisterHandler(&userRepo))
	
	r.Run("127.0.0.1:3000")
}
