package handler

import (
	"database/sql"
	"errors"
	"time"

	"github.com/axumrs/vps-renew-notification-go/internal/db"
	"github.com/axumrs/vps-renew-notification-go/internal/filter"
	"github.com/axumrs/vps-renew-notification-go/internal/model"
	"github.com/axumrs/vps-renew-notification-go/internal/state"
	"github.com/axumrs/vps-renew-notification-go/web/payload"
	"github.com/axumrs/vps-renew-notification-go/web/resp"
	"github.com/gin-gonic/gin"
)

func ListVps(c *gin.Context) error {
	var p payload.ListVps
	if err := c.ShouldBindQuery(&p); err != nil {
		return err
	}

	sort := ""
	switch p.Sort {
	case payload.ListVpsSortID:
		sort = "id"
	case payload.ListVpsSortIDDesc:
		sort = "id DESC"
	case payload.ListVpsSortName:
		sort = "name"
	case payload.ListVpsSortNameDesc:
		sort = "name DESC"
	case payload.ListVpsSortExpire:
		sort = "expire"
	case payload.ListVpsSortExpireDesc:
		sort = "expire DESC"
	}

	ls, err := db.ListVps(c.Request.Context(), state.Db, &filter.VpsListFilter{
		Name:       p.Name,
		ProviderID: p.ProviderID,
		Sort:       sort,
	})
	if err != nil {
		return err
	}

	resp.Ok(c, ls)
	return nil
}
func AddVps(c *gin.Context) error {
	var p payload.AddVps
	if err := c.ShouldBindJSON(&p); err != nil {
		return err
	}

	id, err := db.CreateVps(c.Request.Context(), state.Db, &model.VPS{ProviderID: p.ProviderID, Name: p.Name, Expire: p.Expire, Dateline: time.Now()})
	if err != nil {
		return err
	}

	resp.Ok(c, gin.H{"id": id})
	return nil
}
func EditVps(c *gin.Context) error {
	var p payload.EditVps
	if err := c.ShouldBindJSON(&p); err != nil {
		return err
	}

	aff, err := db.UpdateVps(c.Request.Context(), state.Db, &model.VPS{ID: p.ID, ProviderID: p.ProviderID, Name: p.Name, Expire: p.Expire})
	if err != nil {
		return err
	}

	resp.Ok(c, gin.H{"aff": aff})
	return nil
}
func FindVps(c *gin.Context) error {
	id := c.Param("id")

	m, err := db.FindVps(c.Request.Context(), state.Db, id)
	if err != nil {
		if err == sql.ErrNoRows {
			return errors.New("不存在的记录")
		}
		return err
	}

	resp.Ok(c, m)
	return nil
}
func DelVps(c *gin.Context) error {
	id := c.Param("id")

	aff, err := db.DeleteVps(c.Request.Context(), state.Db, id)
	if err != nil {
		return err
	}

	resp.Ok(c, gin.H{"aff": aff})
	return nil
}
func RenewVps(c *gin.Context) error {
	id := c.Param("id")

	tx, err := state.Db.Beginx()
	if err != nil {
		return err
	}

	v, err := db.FindVps(c.Request.Context(), tx, id)
	if err != nil {
		tx.Rollback()
		if err == sql.ErrNoRows {
			return errors.New("不存在的记录")
		}
		return err
	}

	provider, err := db.FindProvider(c.Request.Context(), tx, v.ProviderID)
	if err != nil {
		tx.Rollback()
		if err == sql.ErrNoRows {
			return errors.New("不存在的记录")
		}
		return err
	}

	expire := v.Expire.Add(time.Duration(provider.RenewDays) * 24 * time.Hour)

	v.Expire = expire

	aff, err := db.UpdateVps(c.Request.Context(), tx, v)
	if err != nil {
		tx.Rollback()
		return err
	}
	tx.Commit()

	resp.Ok(c, gin.H{"aff": aff})
	return nil
}
