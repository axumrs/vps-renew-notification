package model

import "time"

type Provider struct {
	ID         string    `json:"id" db:"id"`
	Name       string    `json:"name" db:"name"`
	RenewDays  int       `json:"renew_days" db:"renew_days"`
	NotifyDays int       `json:"notify_days" db:"notify_days"`
	Dateline   time.Time `json:"dateline" db:"dateline"`
}

type VPS struct {
	ID         string    `json:"id" db:"id"`
	ProviderID string    `json:"provider_id" db:"provider_id"`
	Name       string    `json:"name" db:"name"`
	Expire     time.Time `json:"expire" db:"expire"`
	Dateline   time.Time `json:"dateline" db:"dateline"`
}

type User struct {
	ID       string    `json:"id" db:"id"`
	Username string    `json:"username" db:"username"`
	Password string    `json:"-" db:"password"`
	Dateline time.Time `json:"dateline" db:"dateline"`
}

type VPSWithProvider struct {
	VPS
	ProviderName string `json:"provider_name" db:"provider_name"`
}
