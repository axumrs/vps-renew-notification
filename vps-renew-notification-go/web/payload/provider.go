package payload

type AddProvider struct {
	Name       string `json:"name" form:"name" binding:"required,max=50"`
	RenewDays  int    `json:"renew_days" form:"renew_days" binding:"required,gte=1"`
	NotifyDays int    `json:"notify_days" form:"notify_days" binding:"required,gte=1"`
}
type EditProvider struct {
	ID string `json:"id" form:"id" binding:"required,len=20"`
	AddProvider
}

type ListProviderSort = string

const (
	ListProviderSortName     ListProviderSort = "name"
	ListProviderSortNameDesc ListProviderSort = "name_desc"
	ListProviderSortID       ListProviderSort = "id"
	ListProviderSortIDDesc   ListProviderSort = "id_desc"
)

type ListProvider struct {
	Name string           `json:"name" form:"name"`
	Sort ListProviderSort `json:"sort" form:"sort"`
}
