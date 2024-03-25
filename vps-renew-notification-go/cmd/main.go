package main

import (
	"log"

	"github.com/axumrs/vps-renew-notification-go/internal/state"
	"github.com/axumrs/vps-renew-notification-go/tg"
	"github.com/axumrs/vps-renew-notification-go/web/router"
	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
)

func main() {
	state.Init()

	go tg.RenewNotify()

	app := gin.New()

	app.Use(gin.Logger(), gin.Recovery())
	corsCfg := cors.DefaultConfig()
	corsCfg.AllowAllOrigins = true
	corsCfg.AllowHeaders = append(corsCfg.AllowHeaders, "Authorization")
	app.Use(cors.New(corsCfg))

	router.Init(app)

	log.Fatal(app.Run(state.Cfg.Web.Addr))
}
