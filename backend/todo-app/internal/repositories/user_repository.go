package repository

import (
	"github.com/uptrace/bun"
	"github.com/abrishk26/Roadmap/backend/todo-app/internal/models"
	"context"
)

type UserRepository struct {
	db *bun.DB
}

func NewUserRepository(db *bun.DB) *UserRepository {
	return &UserRepository{ db: db}
}

func (r *UserRepository) FindByEmail(email string, ctx context.Context) (*models.User, error) {
	user := new(models.User)
	
	err := r.db.NewSelect().Model(user).Where("email = ?", email).Scan(ctx)
	
	if err != nil {
		return nil, err
	}
	
	return user, nil
}

func (r *UserRepository) Create(ctx context.Context, user *models.User) (*models.User, error) {
	_, err := r.db.NewInsert().Model(user).Returning("*").Exec(ctx)
	
	if err != nil {
		return nil, err
	}
	
	return user, nil	
}