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

// CreateProvider 创建服务商
func CreateProvider(ctx context.Context, e sqlx.ExecerContext, m *model.Provider) (string, error) {
	id := pkg.NewID()
	if _, err := e.ExecContext(ctx, "INSERT INTO providers (id, name, renew_days, notify_days, dateline) VALUES ($1, $2, $3, $4, $5)", id, m.Name, m.RenewDays, m.NotifyDays, m.Dateline); err != nil {
		return "", err
	}

	return id, nil
}

// UpdateProvider 修改服务商
func UpdateProvider(ctx context.Context, e sqlx.ExecerContext, m *model.Provider) (int64, error) {
	r, err := e.ExecContext(ctx, "UPDATE providers SET name=$1, renew_days = $2, notify_days = $3 WHERE id=$4", m.Name, m.RenewDays, m.NotifyDays, m.ID)
	if err != nil {
		return 0, err
	}
	return r.RowsAffected()
}

// DeleteProvider 删除服务商
func DeleteProvider(ctx context.Context, e sqlx.ExecerContext, id string) (int64, error) {
	return Delete(ctx, e, "providers", id)
}

// FindProvider 查找服务商
func FindProvider(ctx context.Context, q sqlx.QueryerContext, id string) (*model.Provider, error) {
	var m model.Provider
	if err := sqlx.GetContext(ctx, q, &m, "SELECT id, name, renew_days, notify_days, dateline FROM providers WHERE id=$1", id); err != nil {
		return nil, err
	}
	return &m, nil
}

// ListProvider 服务商列表
func ListProvider(ctx context.Context, q sqlx.QueryerContext, f *filter.ProviderListFilter) ([]*model.Provider, error) {
	ls := make([]*model.Provider, 0)

	params := []any{}

	sb := strings.Builder{}
	sb.WriteString("SELECT id, name, renew_days, notify_days, dateline FROM providers WHERE 1=1")

	if !pkg.IsEmptyStr(f.Name) {
		sb.WriteString(" AND name ILIKE ")
		sb.WriteString(fmt.Sprintf("$%d", len(params)+1))
		params = append(params, fmt.Sprintf("%%%s%%", f.Name))
	}
	if !pkg.IsEmptyStr(f.Sort) {
		sb.WriteString(" ORDER BY ")
		sb.WriteString(f.Sort)
	} else {
		sb.WriteString(" ORDER BY id DESC")
	}

	if err := sqlx.SelectContext(ctx, q, &ls, sb.String(), params...); err != nil {
		return nil, err
	}
	return ls, nil
}
