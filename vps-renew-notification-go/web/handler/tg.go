package handler

import (
	"github.com/axumrs/vps-renew-notification-go/internal/pkg"
	"github.com/axumrs/vps-renew-notification-go/internal/state"
	"github.com/axumrs/vps-renew-notification-go/web/payload"
	"github.com/axumrs/vps-renew-notification-go/web/resp"
	"github.com/gin-gonic/gin"
)

func SentBotMessage(c *gin.Context) error {
	var p payload.BotMessage
	if err := c.ShouldBindJSON(&p); err != nil {
		return err
	}

	code, err := pkg.TgSentBotMessage(state.Cfg.Bot, p.Text)
	if err != nil {
		return err
	}

	resp.Ok(c, gin.H{"code": code})
	return nil
}
