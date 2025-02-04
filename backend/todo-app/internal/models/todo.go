package models

import (
	"github.com/google/uuid"
	"github.com/uptrace/bun"
)

type Task struct {
    bun.BaseModel `bun:"table:tasks"`                     // Specifies the table name
    TaskID        uuid.UUID `bun:",pk,type:uuid" json:"task_id"` // Primary key
    UserID        uuid.UUID `bun:",notnull" json:"user_id"`      // Foreign key to users table
    Title         string    `bun:",unique,notnull" json:"title"` // Unique, not null
    Description   string    `bun:",notnull" json:"description"`  // Required field
}