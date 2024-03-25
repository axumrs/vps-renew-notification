package pkg

import (
	"fmt"
	"net/http"
	"net/url"
	"time"

	"github.com/axumrs/vps-renew-notification-go/config"
)

func TgSentBotMessage(cfg *config.BotConfig, text string) (int, error) {
	apiUrl := fmt.Sprintf("https://api.telegram.org/bot%s/sendMessage", cfg.Token)
	data := url.Values{
		"chat_id": {fmt.Sprintf("%d", cfg.ChatID)},
		"text":    {text},
	}

	client := &http.Client{Timeout: 10 * time.Second}
	resp, err := client.PostForm(apiUrl, data)
	if err != nil {
		return 0, err
	}
	defer resp.Body.Close()

	return resp.StatusCode, nil
}
