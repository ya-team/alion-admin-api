/// 验证宏，用于快速定义带有验证规则的结构体
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

/// 验证宏，用于快速定义分页请求
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

/// 验证宏，用于快速定义创建输入
#[macro_export]
macro_rules! create_input {
    ($name:ident) => {
        pub type Create$name = $name;
    };
}

/// 验证宏，用于快速定义更新输入
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