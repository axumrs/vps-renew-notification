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

func ListProvider(c *gin.Context) error {
	var p payload.ListProvider
	if err := c.ShouldBindQuery(&p); err != nil {
		return err
	}

	sort := ""
	switch p.Sort {
	case payload.ListProviderSortID:
		sort = "id"
	case payload.ListProviderSortIDDesc:
		sort = "id DESC"
	case payload.ListProviderSortName:
		sort = "name"
	case payload.ListProviderSortNameDesc:
		sort = "name DESC"
	}
	ls, err := db.ListProvider(c.Request.Context(), state.Db, &filter.ProviderListFilter{Name: p.Name, Sort: sort})

	if err != nil {
		return err
	}

	resp.Ok(c, ls)
	return nil
}
func AddProvider(c *gin.Context) error {
	var p payload.AddProvider
	if err := c.ShouldBindJSON(&p); err != nil {
		return err
	}

	id, err := db.CreateProvider(c.Request.Context(), state.Db, &model.Provider{
		Name:       p.Name,
		RenewDays:  p.RenewDays,
		NotifyDays: p.RenewDays,
		Dateline:   time.Now(),
	})
	if err != nil {
		return err
	}

	resp.Ok(c, gin.H{"id": id})
	return nil
}
func EditProvider(c *gin.Context) error {
	var p payload.EditProvider
	if err := c.ShouldBindJSON(&p); err != nil {
		return err
	}

	aff, err := db.UpdateProvider(c.Request.Context(), state.Db, &model.Provider{
		Name:       p.Name,
		RenewDays:  p.RenewDays,
		NotifyDays: p.NotifyDays,
		ID:         p.ID,
	})
	if err != nil {
		return err
	}

	resp.Ok(c, gin.H{"aff": aff})
	return nil
}
func FindProvider(c *gin.Context) error {
	id, _ := c.Params.Get("id")
	m, err := db.FindProvider(c.Request.Context(), state.Db, id)
	if err != nil {
		if err == sql.ErrNoRows {
			return errors.New("不存在的记录")
		}
		return err
	}
	resp.Ok(c, m)
	return nil
}
func DelProvider(c *gin.Context) error {
	id, _ := c.Params.Get("id")
	aff, err := db.DeleteProvider(c.Request.Context(), state.Db, id)
	if err != nil {
		return err
	}

	resp.Ok(c, gin.H{"aff": aff})
	return nil
}
