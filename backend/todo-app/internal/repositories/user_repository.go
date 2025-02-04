package repository

import (
	"github.com/uptrace/bun"
	"github.com/abrishk26/Roadmap/backend/todo-app/internal/models"
	"context"
)

type UserRepository struct {
	Db *bun.DB
}

func NewUserRepository(db *bun.DB) *UserRepository {
	return &UserRepository{ Db: db}
}

func (r *UserRepository) FindByEmail( ctx context.Context, email string) (*models.User, error) {
	user := new(models.User)
	
	err := r.Db.NewSelect().Model(user).Where("email = ?", email).Scan(ctx)
	
	if err != nil {
		return nil, err
	}
	
	return user, nil
}

func (r *UserRepository) Create(ctx context.Context, user *models.User) (*models.User, error) {
	err := r.Db.NewInsert().Model(user).Returning("*").Scan(ctx, user)
	
	if err != nil {
		return nil, err
	}
	
	return user, nil	
}