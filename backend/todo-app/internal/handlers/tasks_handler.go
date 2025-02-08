package handlers

import (
	"log"
	"net/http"

	repository "github.com/abrishk26/Roadmap/backend/todo-app/internal/repositories"
	"github.com/gin-gonic/gin"
)

func TasksHandler(taskRepo *repository.TaskRepository) gin.HandlerFunc {
	return func(ctx *gin.Context) {
		// Extract userID from context
		userIDRaw, exists := ctx.Get("userID")
		if !exists {
			// Log the error when userID is not found
			log.Println("User ID not found in context")
			ctx.HTML(http.StatusUnauthorized, "index.html", gin.H{"error": "You must be logged in to view tasks."})
			ctx.Abort()
			return
		}

		// Ensure userID is a string
		userID, ok := userIDRaw.(string)
		if !ok {
			// Log the error if userID is not a valid string
			log.Println("Invalid user ID format")
			ctx.HTML(http.StatusUnauthorized, "index.html", gin.H{"error": "Invalid user data. Please log in again."})
			ctx.Abort()
			return
		}

		// Use the request's context (do not override ctx)
		tasks, err := taskRepo.GetByUserID(ctx, userID)
		if err != nil {
			// Log the error when fetching tasks
			log.Println("Failed to fetch tasks for userID", userID, ":", err)
			ctx.HTML(http.StatusInternalServerError, "index.html", gin.H{"error": "Failed to load tasks. Please try again later."})
			ctx.Abort()
			return
		}

		log.Println(len(tasks))
		// Render the tasks page
		ctx.HTML(http.StatusOK, "tasks.html", gin.H{
			"tasks": tasks,
		})
	}
}
