package state

import (
	"flag"
	"log"

	"github.com/axumrs/vps-renew-notification-go/config"
	"github.com/axumrs/vps-renew-notification-go/internal/db"
	"github.com/jmoiron/sqlx"
)

var (
	Cfg *config.Config
	Db  *sqlx.DB
)

func Init() {
	log.SetFlags(log.Default().Flags() | log.Lshortfile)

	if err := initConfig(); err != nil {
		log.Fatal("初始化配置失败：", err)
	}

	if err := initDb(Cfg.Db); err != nil {
		log.Fatal("初始化数据库失败：", err)
	}
}

func initConfig() error {
	cfgFile := flag.String("c", "./config.json", "配置文件路径")

	var err error
	Cfg, err = config.Init(*cfgFile)

	return err

}

func initDb(cfg *config.DatabaseConfig) error {
	var err error
	Db, err = db.Init(cfg)

	return err
}
