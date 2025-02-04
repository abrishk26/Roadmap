package handlers

import(
	"context"
	"net/http"
	"github.com/gin-gonic/gin"
	"golang.org/x/crypto/bcrypt"
	repository "github.com/abrishk26/Roadmap/backend/todo-app/internal/repositories"
)

type LoginRequest struct {
	Email string `json:"email" binding:"required"`
	Password string `json:"password" binding:"required"`
}

func LoginHandler(userRepo *repository.UserRepository) gin.HandlerFunc {
	return func(c *gin.Context) {
		var req LoginRequest
		
		if err := c.ShouldBindJSON(&req); err != nil {
			c.JSON(http.StatusBadRequest, gin.H{
				"error": "Invalid request",
			})
			return
		}
		ctx := context.Background()
		user, err := userRepo.FindByEmail(ctx, req.Email)
		
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{
				"error": err.Error(),
			})
			return
		}
		
		err = bcrypt.CompareHashAndPassword([]byte(user.PasswordHash), []byte(req.Password))
		
		if err != nil {
			c.JSON(http.StatusBadRequest, gin.H{
				"error": "Invalid email or password",
			})
			return
		}
		
		c.JSON(http.StatusOK, gin.H{
			"message": "Login completed successfully",
			"user": gin.H{
				"name": user.Name,
				"email": user.Email,
			},
		})
	}
}

