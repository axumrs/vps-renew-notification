package router

import (
	"log"

	"github.com/axumrs/vps-renew-notification-go/web/handler"
	"github.com/axumrs/vps-renew-notification-go/web/middleware"
	"github.com/axumrs/vps-renew-notification-go/web/resp"
	"github.com/gin-gonic/gin"
)

func w(f func(*gin.Context) error) gin.HandlerFunc {
	return func(c *gin.Context) {
		if err := f(c); err != nil {
			log.Printf("%s - %v\n", c.Request.RequestURI, err)
			resp.Err(c, err)
			c.Abort()
			return
		}
	}
}

func Init(e *gin.Engine) {
	e.Use(w(middleware.AuthUser))

	initAuth(e.Group("/auth"))
	initProvider(e.Group("/provider"))
	initVps(e.Group("/vps"))
	initUser(e.Group("/user"))
	initBot(e.Group("/bot"))
}

func initAuth(g *gin.RouterGroup) {
	g.POST("/login", w(handler.Login))
}

func initProvider(g *gin.RouterGroup) {
	g.GET("", w(handler.ListProvider))
	g.POST("", w(handler.AddProvider))
	g.PUT("", w(handler.EditProvider))
	g.GET("/:id", w(handler.FindProvider))
	g.DELETE("/:id", w(handler.DelProvider))
}

func initVps(g *gin.RouterGroup) {
	g.GET("", w(handler.ListVps))
	g.POST("", w(handler.AddVps))
	g.PUT("", w(handler.EditVps))
	g.GET("/:id", w(handler.FindVps))
	g.DELETE("/:id", w(handler.DelVps))
	g.PATCH("/:id", w(handler.RenewVps))
}

func initUser(g *gin.RouterGroup) {
	g.PUT("/change-password", w(handler.UserChangePassword))
}

func initBot(g *gin.RouterGroup) {
	g.POST("/send-message", w(handler.SentBotMessage))
}
