package middleware

import (
	"fmt"

	"github.com/abrishk26/Roadmap/backend/todo-app/pkg/jwt"
	"github.com/gin-gonic/gin"
)

func AuthMiddleWare() gin.HandlerFunc {
	return func(c *gin.Context) {
		tokenString, err := c.Cookie("token")
		if err != nil {
			fmt.Println("Token missing in cookie")
			c.Next()
			return
		}

		claims, err := jwt.ValidateToken(tokenString)
		if err != nil {
			fmt.Printf("Token verification failed: %v\\n", err)
			c.Next()
			return

		}

		userID, exists := claims["userID"]
		if !exists {
			fmt.Printf("Invalid token: userID missing")
			c.Next()
			return
		}

		// Store user ID in Gin context
		c.Set("userID", userID)

		c.Next()
	}
}
