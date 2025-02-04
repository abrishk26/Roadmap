package database

import (
	"fmt"
	"database/sql"
	"github.com/uptrace/bun"
	"github.com/uptrace/bun/dialect/pgdialect"
	"github.com/uptrace/bun/driver/pgdriver"
)

// InitDB initializes the database connection
func InitDB(connString string) (*bun.DB, error) {
	// Use Bun's pgdriver instead of sql.Open
	sqldb := sql.OpenDB(pgdriver.NewConnector(pgdriver.WithDSN(connString)))

	// Create a Bun DB instance with the PostgreSQL dialect
	db := bun.NewDB(sqldb, pgdialect.New())

	// Ping the database to check if the connection works
	if err := db.Ping(); err != nil {
		return nil, fmt.Errorf("failed to connect to database: %w", err)
	}

	fmt.Println("✅ Successfully connected to the database.")
	return db, nil
}
