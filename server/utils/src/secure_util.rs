/**
 * 安全工具模块
 * 
 * 该模块提供了密码哈希和验证的安全工具函数，使用 Argon2 算法进行密码处理。
 * Argon2 是一个现代化的密码哈希算法，被选为密码哈希竞赛（PHC）的获胜者。
 */

use std::error::Error;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use lazy_static::lazy_static;

lazy_static! {
    static ref ARGON2: Argon2<'static> = Argon2::default();
}

/**
 * 安全工具结构体
 * 
 * 提供密码哈希和验证的静态方法。
 */
pub struct SecureUtil;

impl SecureUtil {
    /**
     * 对密码进行哈希处理
     * 
     * 使用 Argon2 算法对密码进行哈希处理，生成安全的密码哈希值。
     * 每次调用都会生成新的随机盐值，确保相同的密码会产生不同的哈希值。
     * 
     * # 参数
     * * `password` - 需要哈希的密码字节数组
     * 
     * # 返回
     * * `Result<String, Box<dyn Error>>` - 成功返回密码哈希字符串，失败返回错误
     * 
     * # 示例
     * ```
     * let password = b"my_password";
     * let hash = SecureUtil::hash_password(password)?;
     * ```
     */
    pub fn hash_password(password: &[u8]) -> Result<String, Box<dyn Error>> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = ARGON2.hash_password(password, &salt)?.to_string();
        Ok(password_hash)
    }

    /**
     * 验证密码
     * 
     * 验证提供的密码是否与存储的密码哈希值匹配。
     * 
     * # 参数
     * * `password` - 需要验证的密码字节数组
     * * `password_hash` - 存储的密码哈希字符串
     * 
     * # 返回
     * * `Result<bool, Box<dyn Error>>` - 成功返回验证结果（true表示匹配），失败返回错误
     * 
     * # 示例
     * ```
     * let password = b"my_password";
     * let hash = SecureUtil::hash_password(password)?;
     * let is_valid = SecureUtil::verify_password(password, &hash)?;
     * ```
     */
    pub fn verify_password(password: &[u8], password_hash: &str) -> Result<bool, Box<dyn Error>> {
        let parsed_hash = PasswordHash::new(password_hash)?;

        match ARGON2.verify_password(password, &parsed_hash) {
            Ok(_) => Ok(true),
            Err(e) => Err(Box::new(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * 测试密码哈希和验证功能
     * 
     * 验证密码哈希和验证功能是否正常工作：
     * 1. 正确的密码应该能够通过验证
     * 2. 错误的密码应该验证失败
     */
    #[test]
    fn test_password_hash_and_verification() {
        let password = b"example_password";
        let password_hash = SecureUtil::hash_password(password)
            .expect("Failed to hash password, check the input and environment setup");

        assert!(
            SecureUtil::verify_password(password, &password_hash).is_ok(),
            "Password verification should succeed for the correct password"
        );

        let wrong_password = b"wrong_password";
        assert!(
            SecureUtil::verify_password(wrong_password, &password_hash).is_err(),
            "Password verification should fail for the wrong password"
        );
    }

    /**
     * 测试密码哈希输出
     * 
     * 生成并打印测试密码的哈希值，用于验证哈希算法的输出格式。
     */
    #[test]
    fn test_print_hashed_password() {
        let password = b"123456";
        let password_hash = SecureUtil::hash_password(password)
            .expect("Failed to hash password, check the input and environment setup");

        println!("Hashed password for '123456': {}", password_hash);
    }
}
