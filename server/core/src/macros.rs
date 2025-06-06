/// 宏定义模块
/// 
/// 该模块提供了一系列用于简化代码编写的宏，包括：
/// - 验证结构体定义宏
/// - 分页请求定义宏
/// - 创建输入定义宏
/// - 更新输入定义宏

/// 验证结构体定义宏
/// 
/// 用于快速定义带有验证规则的结构体，自动实现序列化、反序列化和验证特性。
/// 
/// # 参数
/// * `$name` - 结构体名称
/// * `$field` - 字段名称
/// * `$ty` - 字段类型
/// * `$validate` - 验证规则
/// 
/// # 示例
/// 
/// validated_struct! {
///     pub struct User {
///         #[validate(length(min = 3, max = 50))]
///         pub username: String,
///         #[validate(email)]
///         pub email: String,
///     }
/// }
/// 
#[macro_export]
macro_rules! validated_struct {
    ($(#[$attr:meta])* $vis:vis struct $name:ident {
        $(
            $(#[$field_attr:meta])*
            $field_vis:vis $field:ident: $ty:ty $(=> $($validate:tt)*)?
        ),* $(,)?
    }) => {
        $(#[$attr])*
        #[derive(Debug, Serialize, Deserialize, Validate)]
        $vis struct $name {
            $(
                $(#[$field_attr])*
                $(
                    #[validate($($validate)*)]
                )?
                $field_vis $field: $ty,
            )*
        }
    };
}

/// 分页请求定义宏
/// 
/// 用于快速定义分页请求结构体，自动包含分页参数和关键词搜索字段。
/// 
/// # 参数
/// * `$name` - 结构体名称
/// * `$field` - 可选字段名称
/// * `$ty` - 可选字段类型
/// 
/// # 示例
/// 
/// page_request!(UserListRequest, role: String, status: bool);
/// 
#[macro_export]
macro_rules! page_request {
    ($name:ident) => {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct $name {
            #[serde(flatten)]
            pub page_details: PageRequest,
            #[serde(default)]
            pub keywords: Option<String>,
        }
    };

    ($name:ident, $($field:ident: $ty:ty),* $(,)?) => {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct $name {
            #[serde(flatten)]
            pub page_details: PageRequest,
            #[serde(default)]
            pub keywords: Option<String>,
            $(
                #[serde(default)]
                pub $field: Option<$ty>,
            )*
        }
    };
}

/// 创建输入定义宏
/// 
/// 用于快速定义创建操作的输入类型，通常与实体类型相同。
/// 
/// # 参数
/// * `$name` - 实体名称
/// 
/// # 示例
/// 
/// create_input!(User);
/// 
#[macro_export]
macro_rules! create_input {
    ($name:ident) => {
        pub type Create$name = $name;
    };
}

/// 更新输入定义宏
/// 
/// 用于快速定义更新操作的输入类型，包含ID字段和实体字段。
/// 
/// # 参数
/// * `$name` - 实体名称
/// * `$id_ty` - ID字段类型
/// 
/// # 示例
/// 
/// update_input!(User, String);
/// 
#[macro_export]
macro_rules! update_input {
    ($name:ident, $id_ty:ty) => {
        #[derive(Deserialize, Validate)]
        pub struct Update$name {
            #[validate(required(message = "ID is required"))]
            pub id: $id_ty,
            
            #[serde(flatten)]
            #[validate]
            pub $name: $name,
        }

        impl ValidateInput for Update$name {}
    };
} 