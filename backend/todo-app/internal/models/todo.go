package models

import "github.com/google/uuid"

type Todo struct {
	ID uuid.UUID `json:id`
	Title string `json:title`
	Description string `json:description`
	UserID uuid.UUID `json:user_id`
}