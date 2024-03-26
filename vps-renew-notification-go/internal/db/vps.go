package db

import (
	"context"
	"fmt"
	"strings"
	"time"

	"github.com/axumrs/vps-renew-notification-go/internal/filter"
	"github.com/axumrs/vps-renew-notification-go/internal/model"
	"github.com/axumrs/vps-renew-notification-go/internal/pkg"
	"github.com/jmoiron/sqlx"
)

// CreateVps 创建VPS
func CreateVps(ctx context.Context, q sqlx.ExecerContext, m *model.VPS) (string, error) {
	id := pkg.NewID()
	if _, err := q.ExecContext(ctx, "INSERT INTO vpss (id, provider_id, name, expire, dateline) VALUES ($1, $2, $3, $4, $5)", id, m.ProviderID, m.Name, time.Time(m.Expire).In(time.Local), m.Dateline); err != nil {
		return "", err
	}
	return id, nil
}

// UpdateVps 更新Vps
func UpdateVps(ctx context.Context, q sqlx.ExecerContext, m *model.VPS) (int64, error) {
	r, err := q.ExecContext(ctx, "UPDATE vpss SET  provider_id=$1, name=$2, expire=$3 WHERE id=$4", m.ProviderID, m.Name, time.Time(m.Expire).In(time.Local), m.ID)
	if err != nil {
		return 0, err
	}
	return r.RowsAffected()
}

// DeleteVps 删除VPS
func DeleteVps(ctx context.Context, q sqlx.ExecerContext, id string) (int64, error) {
	return Delete(ctx, q, "vpss", id)
}

// FindVps 查找VPS
func FindVps(ctx context.Context, q sqlx.QueryerContext, id string) (*model.VPS, error) {
	var m model.VPS
	if err := sqlx.GetContext(ctx, q, &m, "SELECT id, provider_id, name, expire, dateline FROM vpss WHERE id=$1", id); err != nil {
		return nil, err
	}
	return &m, nil
}

// ListVps VPS列表
func ListVps(ctx context.Context, q sqlx.QueryerContext, f *filter.VpsListFilter) ([]*model.VPSWithProvider, error) {
	ls := []*model.VPSWithProvider{}

	params := []any{}
	sb := strings.Builder{}
	sb.WriteString("SELECT id, provider_id, name, expire, dateline,provider_name FROM v_vps_proiders WHERE 1=1")

	if !pkg.IsEmptyStr(f.Name) {
		sb.WriteString(fmt.Sprintf(" AND name ILIKE $%d", len(params)+1))
		params = append(params, fmt.Sprintf("%%%s%%", f.Name))
	}
	if !pkg.IsEmptyStr(f.ProviderID) {
		sb.WriteString(fmt.Sprintf(" AND provider_id = $%d", len(params)+1))
		params = append(params, f.ProviderID)
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
