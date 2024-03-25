package db

import (
	"context"
	"fmt"

	"github.com/axumrs/vps-renew-notification-go/config"
	"github.com/jmoiron/sqlx"
	_ "github.com/lib/pq"
)

func Init(cfg *config.DatabaseConfig) (*sqlx.DB, error) {
	database, err := sqlx.Connect("postgres", cfg.DSN)
	if err != nil {
		return nil, err
	}

	database.SetMaxOpenConns(cfg.MaxConns)

	return database, nil
}

// Delete 删除记录
func Delete(ctx context.Context, e sqlx.ExecerContext, table, id string) (int64, error) {
	r, err := e.ExecContext(ctx, fmt.Sprintf("DELETE FROM %s WHERE id=$1", table), id)
	if err != nil {
		return 0, err
	}
	return r.RowsAffected()
}
