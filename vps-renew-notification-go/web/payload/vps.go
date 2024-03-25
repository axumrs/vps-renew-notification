package payload

import "time"

type AddVps struct {
	Name       string    `json:"name" form:"name" binding:"required,max=50"`
	ProviderID string    `json:"provider_id" form:"provider_id" binding:"required,len=20"`
	Expire     time.Time `json:"expire" form:"expire" binding:"required"`
}

type EditVps struct {
	ID string `json:"id" form:"id" binding:"required,len=20"`
	AddVps
}

type ListVpsSort = string

const (
	ListVpsSortName       ListVpsSort = "name"
	ListVpsSortNameDesc   ListVpsSort = "name_desc"
	ListVpsSortID         ListVpsSort = "id"
	ListVpsSortIDDesc     ListVpsSort = "id_desc"
	ListVpsSortExpire     ListVpsSort = "expire"
	ListVpsSortExpireDesc ListVpsSort = "expire_desc"
)

type ListVps struct {
	Name       string      `json:"name" form:"name"`
	ProviderID string      `json:"provider_id" form:"provider_id"`
	Sort       ListVpsSort `json:"sort" form:"sort"`
}
