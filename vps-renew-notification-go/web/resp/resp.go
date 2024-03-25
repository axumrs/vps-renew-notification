package resp

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func resp(c *gin.Context, code int, msg string, data any) {
	c.JSON(http.StatusOK, gin.H{
		"code": code,
		"msg":  msg,
		"data": data,
	})
}

func Ok(c *gin.Context, data any) {
	resp(c, 0, "OK", data)
}
func Err(c *gin.Context, err error) {
	resp(c, -1, err.Error(), nil)
}
