package jwt

import (
	"errors"
	"time"

	"github.com/axumrs/vps-renew-notification-go/config"
	"github.com/golang-jwt/jwt/v5"
)

func NewUserClaims(id, username string, expire time.Duration) jwt.MapClaims {
	now := time.Now()
	return jwt.MapClaims{
		"sub":      "VPS-RENEW-NOTIFICATION",
		"iss":      "axum.rs",
		"iat":      now.Unix(),
		"exp":      now.Add(expire).Unix(),
		"id":       id,
		"username": username,
	}
	// return &UserClaims{
	// 	ID:       id,
	// 	Username: username,
	// 	RegisteredClaims: jwt.RegisteredClaims{
	// 		ExpiresAt: jwt.NewNumericDate(now.Add(expire)),
	// 		IssuedAt:  jwt.NewNumericDate(now),
	// 		NotBefore: jwt.NewNumericDate(now),
	// 		Issuer:    "axum.rs",
	// 		Subject:   "VPS-RENEW-NOTIFICATION",
	// 	},
	// }
}

// signedToken 对token进行签名
func signedToken(claims jwt.MapClaims, key string) (string, error) {
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	return token.SignedString([]byte(key))
}

// Token 生成Token
func Token(id, username string, cfg *config.JwtConfig) (string, jwt.MapClaims, error) {
	claims := NewUserClaims(id, username, cfg.Expire)
	token, err := signedToken(claims, cfg.SecretKey)
	if err != nil {
		return "", nil, err
	}
	return token, claims, nil
}

// ParseAndVerifyToken 解析并验证Token
func ParseAndVerifyToken(tokenStr string, cfg *config.JwtConfig) (jwt.MapClaims, error) {
	token, err := jwt.Parse(tokenStr, func(t *jwt.Token) (interface{}, error) {
		if _, ok := t.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, errors.New("不支持的算法")
		}
		return []byte(cfg.SecretKey), nil
	})
	if err != nil {
		return nil, err
	}

	if claims, ok := token.Claims.(jwt.MapClaims); ok {
		return claims, nil
	}
	return nil, errors.New("非法的Token")
}
