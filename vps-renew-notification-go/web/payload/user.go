package payload

type UserChangePassword struct {
	ID          string `json:"id" form:"id" binding:"required,len=20"`
	Password    string `json:"password" form:"password" binding:"required"`
	NewPassword string `json:"new_password" form:"new_password" binding:"required"`
	RePassword  string `json:"re_password" form:"re_password" binding:"required,eqfield=NewPassword"`
}
