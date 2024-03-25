package handler

import (
	"database/sql"
	"errors"

	"github.com/axumrs/vps-renew-notification-go/internal/db"
	"github.com/axumrs/vps-renew-notification-go/internal/filter"
	"github.com/axumrs/vps-renew-notification-go/internal/jwt"
	"github.com/axumrs/vps-renew-notification-go/internal/pkg"
	"github.com/axumrs/vps-renew-notification-go/internal/state"
	"github.com/axumrs/vps-renew-notification-go/web/payload"
	"github.com/axumrs/vps-renew-notification-go/web/resp"
	"github.com/gin-gonic/gin"
)

func Login(c *gin.Context) error {
	var p payload.Login
	if err := c.ShouldBind(&p); err != nil {
		return err
	}

	u, err := db.FindUser(c.Request.Context(), state.Db, &filter.UserFindBy{Username: p.Username})

	if err != nil {
		if err == sql.ErrNoRows {
			return errors.New("用户名/密码错误")
		}
		return err
	}

	if err := pkg.VerifyPassword(p.Password, u.Password); err != nil {
		return errors.New("用户名/密码错误")
	}

	token, claims, err := jwt.Token(u.ID, u.Username, state.Cfg.Jwt)
	if err != nil {
		return err
	}

	resp.Ok(c, gin.H{
		"auth": gin.H{"token": token, "token_type": "Bearer"},
		"data": claims,
	})
	return nil
}
