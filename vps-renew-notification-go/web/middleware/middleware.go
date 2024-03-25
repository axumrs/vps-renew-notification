package middleware

import (
	"errors"
	"log"
	"strings"

	"github.com/axumrs/vps-renew-notification-go/internal/jwt"
	"github.com/axumrs/vps-renew-notification-go/internal/pkg"
	"github.com/axumrs/vps-renew-notification-go/internal/state"
	"github.com/gin-gonic/gin"
)

func AuthUser(c *gin.Context) error {
	if c.Request.RequestURI == "/auth/login" {
		return nil
	}

	authorization := c.Request.Header.Get("Authorization")
	if pkg.IsEmptyStr(authorization) {
		return errors.New("未授权1")
	}

	authorizationSlice := strings.Split(authorization, " ")
	if len(authorizationSlice) != 2 {
		return errors.New("未授权2")
	}

	token := authorizationSlice[1]

	if _, err := jwt.ParseAndVerifyToken(token, state.Cfg.Jwt); err != nil {
		log.Println(err)
		return errors.New("未授权3")
	}

	return nil
}
