package pkg

import "golang.org/x/crypto/bcrypt"

// HashPassword 哈希密码
func HashPassword(pwd string) (string, error) {
	buf, err := bcrypt.GenerateFromPassword([]byte(pwd), bcrypt.DefaultCost)
	if err != nil {
		return "", err
	}
	return string(buf), nil
}

// VerifyPassword 验证密码
func VerifyPassword(pwd, hashedPwd string) error {
	return bcrypt.CompareHashAndPassword([]byte(hashedPwd), []byte(pwd))
}
