/// Enum to represent different platforms or audiences for JWT authentication.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Audience {
    /// Audience for the official website.
    OfficialWebsite,
    /// Audience for the admin control panel.
    ManagementPlatform,
    /// Audience for the mobile application.
    MobileApp,
    /// Audience for mini-programs or widgets.
    MiniProgram,
}

impl Audience {
    /// Returns the audience string associated with each platform.
    pub fn as_str(self) -> &'static str {
        match self {
            Audience::OfficialWebsite => "official_website",
            Audience::ManagementPlatform => "management_platform",
            Audience::MobileApp => "mobile_app",
            Audience::MiniProgram => "mini_program",
        }
    }
}
