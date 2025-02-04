package database

import (
	"context"
	"database/sql"
	"fmt"
	"log"
	"github.com/uptrace/bun"
    _ "github.com/lib/pq" 

)

// InitDB initializesthe database connection
func InitDB(connString string) (*bun.DB, error) {
	sqldb, err := sql.Open("postgres", connString)
	
	if err != nil {
		return nil, err
	}
	
	db := bun.NewDB(sqldb, nil)
	
	if err = db.Ping(); err != nil {
		return nil, err
	}
	
	fmt.Println("Successfully connected to a database.")
	return db, nil
}