package filter

type ProviderListFilter struct {
	Name string
	Sort string
}

type VpsListFilter struct {
	Name       string
	Sort       string
	ProviderID string
}

type UserFindBy struct {
	ID       string
	Username string
}
