package payload

type Login struct {
	Username string `json:"username" form:"username" binding:"required,max=50"`
	Password string `json:"password" form:"password" binding:"required"`
}
