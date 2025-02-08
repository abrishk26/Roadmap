package handlers

import(
	"context"
	"net/http"
	"github.com/gin-gonic/gin"
	"golang.org/x/crypto/bcrypt"
	repository "github.com/abrishk26/Roadmap/backend/todo-app/internal/repositories"
	"github.com/abrishk26/Roadmap/backend/todo-app/pkg/jwt"
)

type LoginRequest struct {
	Email string `json:"email" binding:"required"`
	Password string `json:"password" binding:"required"`
}

func LoginHandler(userRepo *repository.UserRepository) gin.HandlerFunc {
	return func(c *gin.Context) {
		var req LoginRequest
		
		if err := c.ShouldBindJSON(&req); err != nil {
			c.Redirect(http.StatusBadRequest, "/")
			return
		}
		ctx := context.Background()
		user, err := userRepo.FindByEmail(ctx, req.Email)
		
		if err != nil {
			c.Redirect(http.StatusInternalServerError, "/")
			return
		}
		
		err = bcrypt.CompareHashAndPassword([]byte(user.PasswordHash), []byte(req.Password))
		
		if err != nil {
			c.Redirect(http.StatusBadRequest, "/")
			return
		}
		
		tokenString, err := jwt.GenerateToken(user.ID.String())
		
		if err != nil {
			c.Redirect(http.StatusInternalServerError, "/")
			return
		}
		
		c.SetCookie("token", tokenString, 3600, "/", "", false, true)
		c.Redirect(http.StatusOK, "/tasks")
	}
}

