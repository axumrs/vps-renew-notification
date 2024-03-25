package tg

import (
	"context"
	"fmt"
	"log"
	"time"

	"github.com/axumrs/vps-renew-notification-go/internal/db"
	"github.com/axumrs/vps-renew-notification-go/internal/filter"
	"github.com/axumrs/vps-renew-notification-go/internal/pkg"
	"github.com/axumrs/vps-renew-notification-go/internal/state"
)

func RenewNotify() {
	ctx := context.Background()
BEGIN:
	for {
		log.Println("新一轮提示检测")

		// 服务商
		providerList, err := db.ListProvider(ctx, state.Db, &filter.ProviderListFilter{})
		if err != nil {
			log.Println("获取服务商列表失败：", err)
			continue BEGIN
		}

		// vps
		vpsList, err := db.ListVps(ctx, state.Db, &filter.VpsListFilter{})
		if err != nil {
			log.Println("获取VPS列表失败：", err)
			continue BEGIN
		}

		for _, vps := range vpsList {

			var notifyDays = 0
			for _, provider := range providerList {
				if provider.ID == vps.ProviderID {
					notifyDays = provider.NotifyDays
					break
				}
			}
			if notifyDays == 0 {
				log.Println("没有找到符合条件的服务商")
				continue
			}

			expectNotifyDateStr := time.Now().Add(time.Hour * time.Duration(notifyDays) * 24).Format("2006-01-02")
			expectNotifyDate, err := time.Parse("2006-01-02", expectNotifyDateStr)
			if err != nil {
				log.Println("解析时间出错")
				continue
			}
			// log.Println(vps.Name, expectNotifyDate, vps.Expire, int(vps.Expire.Sub(expectNotifyDate).Hours())/24)
			if int(vps.Expire.Sub(expectNotifyDate).Hours())/24 < notifyDays {
				msg := fmt.Sprintf("%s即将过期(%s)，请及时续期！\n%s", vps.Name, vps.Expire.Format("2006-01-02"), time.Now().Format("2006/01/02 15:04:05"))
				log.Println(msg)
				go func() {
					code, err := pkg.TgSentBotMessage(state.Cfg.Bot, msg)
					log.Println("sendmsg code:", code, ", err:", err)
				}()
			}
		}

		time.Sleep(state.Cfg.Bot.SleepDuration)
	}
}
