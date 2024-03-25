package config

import (
	"encoding/json"
	"os"
	"time"
)

type DatabaseConfig struct {
	DSN      string `json:"dsn"`
	MaxConns int    `json:"max_conns"`
}

type WebConfig struct {
	Addr string `json:"addr"`
}

type JwtConfig struct {
	SecretKey string        `json:"secret_key"`
	Expire    time.Duration `json:"expire"`
}

type BotConfig struct {
	Token         string        `json:"token"`
	ChatID        int64         `json:"chat_id"`
	SleepDuration time.Duration `json:"sleep_duration"`
}

type Config struct {
	Db  *DatabaseConfig
	Web *WebConfig
	Jwt *JwtConfig
	Bot *BotConfig
}

func Init(filename string) (*Config, error) {
	cfg := &Config{}
	buf, err := os.ReadFile(filename)
	if err != nil {
		return nil, err
	}

	if err := json.Unmarshal(buf, cfg); err != nil {
		return nil, err
	}

	cfg.Jwt.Expire = time.Minute * cfg.Jwt.Expire
	cfg.Bot.SleepDuration = time.Second * cfg.Bot.SleepDuration

	return cfg, nil
}
