package models

import (
	"github.com/google/uuid"
	"github.com/uptrace/bun"
)

type User struct {
	bun.BaseModel `bun:"table:users"`
	ID uuid.UUID  `bun:",pk,type:uuid" json:"id"`
	Name string `bun:",notnull" json:"name"`
	Email string `bun:",unique,notnull" json:"email"`
	PasswordHash string `bun:",notnull" json:"-"`
}