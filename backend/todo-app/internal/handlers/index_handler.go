package handlers

import (
	"log"
	"net/http"

	"github.com/gin-gonic/gin"
)

func IndexHandler(ctx *gin.Context) {
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
	_, ok := userIDRaw.(string)
	if !ok {
		// Log the error if userID is not a valid string
		log.Println("Invalid user ID format")
		ctx.HTML(http.StatusUnauthorized, "index.html", gin.H{"error": "Invalid user data. Please log in again."})
		ctx.Abort()
		return
	}

	ctx.Redirect(http.StatusSeeOther, "/tasks")
}
