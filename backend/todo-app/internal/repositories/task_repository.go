package repository

import (
	"github.com/uptrace/bun"
	"github.com/abrishk26/Roadmap/backend/todo-app/internal/models"
	"context"
)

type TaskRepository struct {
	Db *bun.DB
}

func NewTaskRepository(db *bun.DB) *TaskRepository {
	return &TaskRepository { Db: db }
}

func(t *TaskRepository) GetByUserID(ctx context.Context, userID string) ([]*models.Task, error) {
	var tasks []*models.Task
	
	err := t.Db.NewSelect().Model(&tasks).Where("user_id = ?", userID).Scan(ctx)
	
	if err != nil {
		return nil, err
	}
	
	return tasks, nil
}

func(t *TaskRepository) Create(ctx context.Context, task *models.Task) (*models.Task, error) {
	_, err := t.Db.NewInsert().Model(task).Returning("*").Exec(ctx)
	
	if err != nil {
		return nil, err
	}
	
	return task, nil
}