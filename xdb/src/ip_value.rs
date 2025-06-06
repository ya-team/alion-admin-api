/**
 * IP地址值转换模块
 * 
 * 该模块提供了将不同格式的IP地址转换为u32类型的统一接口。
 * 支持以下类型的转换：
 * - u32类型
 * - 字符串类型（支持点分十进制格式和纯数字格式）
 * - Ipv4Addr类型
 */

use std::{error::Error, net::Ipv4Addr, str::FromStr};

/**
 * IP地址转u32特征
 * 
 * 定义了将不同类型转换为u32格式IP地址的接口。
 * 所有实现了该特征的类型都可以转换为u32格式的IP地址。
 */
pub trait ToUIntIP {
    /**
     * 转换为u32格式的IP地址
     * 
     * # 返回
     * * `Result<u32, Box<dyn Error>>` - 成功返回u32格式的IP地址，失败返回错误
     */
    fn to_u32_ip(&self) -> Result<u32, Box<dyn Error>>;
}

/**
 * u32类型的IP地址转换实现
 * 
 * 直接返回u32值作为IP地址。
 */
impl ToUIntIP for u32 {
    #[inline(always)]
    fn to_u32_ip(&self) -> Result<u32, Box<dyn Error>> {
        Ok(*self)
    }
}

/**
 * 字符串类型的IP地址转换实现
 * 
 * 支持两种格式的字符串：
 * 1. 点分十进制格式（如："1.1.1.1"）
 * 2. 纯数字格式（如："12"）
 */
impl ToUIntIP for &str {
    #[inline(always)]
    fn to_u32_ip(&self) -> Result<u32, Box<dyn Error>> {
        if let Ok(num) = self.parse::<u32>() {
            return Ok(num);
        }
        Ok(u32::from(Ipv4Addr::from_str(self)?))
    }
}

/**
 * Ipv4Addr类型的IP地址转换实现
 * 
 * 将Ipv4Addr类型转换为u32格式的IP地址。
 */
impl ToUIntIP for Ipv4Addr {
    #[inline(always)]
    fn to_u32_ip(&self) -> Result<u32, Box<dyn Error>> {
        Ok(u32::from(*self))
    }
}

#[cfg(test)]
mod test_ip {
    use super::*;

    /**
     * 测试点分十进制格式的IP地址转换
     * 
     * 验证将点分十进制格式的IP地址字符串转换为u32格式的正确性。
     */
    #[test]
    fn test_ip_str_2_u32() {
        let ip_str = "1.1.1.1";
        let result = ip_str.to_u32_ip().unwrap();
        assert_eq!(result, 1 << 24 | 1 << 16 | 1 << 8 | 1);
    }

    /**
     * 测试纯数字格式的IP地址转换
     * 
     * 验证将纯数字格式的IP地址字符串转换为u32格式的正确性。
     */
    #[test]
    fn test_ip_u32_str() {
        let ip = "12";
        let result = ip.to_u32_ip().unwrap();
        assert_eq!(result, 12);
    }

    /**
     * 测试u32类型的IP地址转换
     * 
     * 验证将u32类型的IP地址转换为u32格式的正确性。
     */
    #[test]
    fn test_ip_u32() {
        let ip: u32 = 33;
        let result = ip.to_u32_ip().unwrap();
        assert_eq!(result, 33);
    }

    /**
     * 测试Ipv4Addr类型的IP地址转换
     * 
     * 验证将Ipv4Addr类型的IP地址转换为u32格式的正确性。
     */
    #[test]
    fn test_ip_addr() {
        let ip = Ipv4Addr::from_str("0.0.3.12").unwrap();
        let result = ip.to_u32_ip().unwrap();
        assert_eq!(result, 3 << 8 | 12)
    }
}
