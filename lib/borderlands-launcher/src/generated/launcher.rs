#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvailableData {
    #[prost(message, optional, tag = "1")]
    pub news_info: ::core::option::Option<DynamicContentUpdateInfo>,
    #[prost(message, optional, tag = "2")]
    pub background_info: ::core::option::Option<DynamicContentUpdateInfo>,
    #[prost(message, repeated, tag = "3")]
    pub news_stories: ::prost::alloc::vec::Vec<NewsStory>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicContentUpdateInfo {
    #[prost(int32, tag = "1")]
    pub last_updated: i32,
    #[prost(string, tag = "2")]
    pub data_url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewsStory {
    #[prost(string, tag = "1")]
    pub headline: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub date: i32,
    #[prost(string, tag = "3")]
    pub url: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub full_story: ::prost::alloc::string::String,
}
