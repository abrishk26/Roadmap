package main

import (
	"fmt"
	"os"

	"github.com/abrishk26/Roadmap/backend/todo-app/internal/handlers"
	repository "github.com/abrishk26/Roadmap/backend/todo-app/internal/repositories"
	"github.com/abrishk26/Roadmap/backend/todo-app/pkg/database"
	"github.com/gin-gonic/gin"
	"github.com/joho/godotenv"
)

func main() {
	
	// Load DATABASE_URL into the environment variable
	err := godotenv.Load()

	if err != nil {
		fmt.Println("Error loading environment varialbe")
		return
	}

	// Retrieve the value of DATABASE_URL environment variable
	dns, stat := os.LookupEnv("DATABASE_URL")

	if !stat {
		fmt.Println("env variable DATABASE_URL must be set.")
		return
	}
	
	// Create a database connection
	db, err := database.InitDB(dns)

	if err != nil {
		fmt.Println("Unable to connect to the database.")
		return
	}
	
	// Create a user and task repository
	userRepo := repository.UserRepository{ Db: db}

	// Initialize the router
	r := gin.Default()
	
	// Load templates
	r.LoadHTMLGlob("web/templates/*")
	
	// Register handlers
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
	r.POST("/login", handlers.LoginHandler(&userRepo))
	
	// Set the listening address
	r.Run("127.0.0.1:3000")
}
