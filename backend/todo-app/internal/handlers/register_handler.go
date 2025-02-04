package handlers

import (
	"context"
	"net/http"

	"github.com/abrishk26/Roadmap/backend/todo-app/internal/models"
	repository "github.com/abrishk26/Roadmap/backend/todo-app/internal/repositories"
	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
	"golang.org/x/crypto/bcrypt"
)

type RegisterRequest struct {
	Name     string `json:"name" binding:"required"`
	Email    string `json:"email" binding:"required"`
	Password string `json:"password" binding:"required"`
}

func RegisterHandler(userRepo *repository.UserRepository) gin.HandlerFunc {
	return func(c *gin.Context) {
		var req RegisterRequest

		if err := c.ShouldBindJSON(&req); err != nil {
			c.JSON(http.StatusBadRequest, gin.H{
				"error": "Invalid request",
			})
			return
		}

		hashedPassword, err := bcrypt.GenerateFromPassword([]byte(req.Password), bcrypt.DefaultCost)

		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{
				"error": "Failed to hash passwrod",
			})
			return
		}

		ctx := context.Background()
		newUser := models.User{
			ID:           uuid.New(),
			Name:         req.Name,
			Email:        req.Email,
			PasswordHash: string(hashedPassword),
		}

		user, err := userRepo.Create(ctx, &newUser)

		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{
				"error": "Failed to register user",
			})

			return
		}

		c.JSON(http.StatusOK, gin.H{
			"message": "User successfully registered",
			"user": gin.H{
				"id":    user.ID,
				"name":  user.Name,
				"email": user.Email,
			},
		})
	}
}
