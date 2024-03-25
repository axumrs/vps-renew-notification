package db

import (
	"context"
	"fmt"
	"strings"

	"github.com/axumrs/vps-renew-notification-go/internal/filter"
	"github.com/axumrs/vps-renew-notification-go/internal/model"
	"github.com/axumrs/vps-renew-notification-go/internal/pkg"
	"github.com/jmoiron/sqlx"
)

// CreateUser 创建用户
func CreateUser(ctx context.Context, q sqlx.ExecerContext, m *model.User) (string, error) {
	id := pkg.NewID()
	if _, err := q.ExecContext(ctx, "INSERT INTO users (id, username, password, dateline) VALUES ($1, $2, $3, $4)", id, m.Username, m.Password, m.Dateline); err != nil {
		return "", err
	}
	return id, nil
}

// UpdateUser 修改用户
func UpdateUser(ctx context.Context, q sqlx.ExecerContext, m *model.User) (int64, error) {
	r, err := q.ExecContext(ctx, "UPDATE users SET username=$1, password=$2 WHERE id=$3", m.Username, m.Password, m.ID)
	if err != nil {
		return 0, err
	}
	return r.RowsAffected()
}

// DeleteUser 删除用户
func DeleteUser(ctx context.Context, q sqlx.ExecerContext, id string) (int64, error) {
	return Delete(ctx, q, "users", id)
}

// FindUser 查找用户
func FindUser(ctx context.Context, q sqlx.QueryerContext, by *filter.UserFindBy) (*model.User, error) {
	var m model.User

	params := []any{}
	sb := strings.Builder{}
	sb.WriteString("SELECT id, username, password, dateline FROM users WHERE 1=1")

	if !pkg.IsEmptyStr(by.ID) {
		sb.WriteString(fmt.Sprintf(" AND id=$%d", len(params)+1))
		params = append(params, by.ID)
	}
	if !pkg.IsEmptyStr(by.Username) {
		sb.WriteString(fmt.Sprintf(" AND username=$%d", len(params)+1))
		params = append(params, by.Username)
	}

	if err := sqlx.GetContext(ctx, q, &m, sb.String(), params...); err != nil {
		return nil, err
	}

	return &m, nil
}
