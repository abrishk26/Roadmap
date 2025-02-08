package handlers

import (
	"database/sql"
	"errors"
	"fmt"
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
				"error": err.Error(),
			})
			fmt.Println(err.Error())
			return
		}

		existingUser, err := userRepo.FindByEmail(c, req.Email)

		if err != nil {
			if errors.Is(err, sql.ErrNoRows) { // No user found is NOT an error
			} else {
				c.JSON(http.StatusBadRequest, gin.H{
					"error": err.Error(),
				})
				fmt.Println(err.Error())
				return // Actual error
			}

		}
		if existingUser != nil {
			c.JSON(http.StatusBadRequest, gin.H{
				"error": "user with this email already exists",
			})
			return
		}

		hashedPassword, err := bcrypt.GenerateFromPassword([]byte(req.Password), bcrypt.DefaultCost)

		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{
				"error": "Failed to hash passwrod",
			})
			fmt.Println(err.Error())
			return
		}

		newUser := models.User{
			ID:           uuid.New(),
			Name:         req.Name,
			Email:        req.Email,
			PasswordHash: string(hashedPassword),
		}

		_, err = userRepo.Create(c, &newUser)

		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{
				"error": err.Error(),
			})
			fmt.Println(err.Error())
			return
		}

		c.Redirect(http.StatusOK, "/login")
	}
}
