package handler

import (
	"database/sql"
	"errors"

	"github.com/axumrs/vps-renew-notification-go/internal/db"
	"github.com/axumrs/vps-renew-notification-go/internal/filter"
	"github.com/axumrs/vps-renew-notification-go/internal/pkg"
	"github.com/axumrs/vps-renew-notification-go/internal/state"
	"github.com/axumrs/vps-renew-notification-go/web/payload"
	"github.com/axumrs/vps-renew-notification-go/web/resp"
	"github.com/gin-gonic/gin"
)

func UserChangePassword(c *gin.Context) error {
	var p payload.UserChangePassword
	if err := c.ShouldBindJSON(&p); err != nil {
		return err
	}

	m, err := db.FindUser(c.Request.Context(), state.Db, &filter.UserFindBy{ID: p.ID})
	if err != nil {
		if err == sql.ErrNoRows {
			return errors.New("不存在的用户")
		}
		return err
	}

	if err := pkg.VerifyPassword(p.Password, m.Password); err != nil {
		return errors.New("密码错误")
	}

	hashedPwd, err := pkg.HashPassword(p.NewPassword)
	if err != nil {
		return err
	}

	m.Password = hashedPwd

	aff, err := db.UpdateUser(c.Request.Context(), state.Db, m)
	if err != nil {
		return err
	}

	resp.Ok(c, gin.H{"aff": aff})
	return nil
}
