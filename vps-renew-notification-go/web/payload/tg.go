package payload

type BotMessage struct {
	Text string `json:"text" form:"text" binding:"required"`
}
