use aws_config::BehaviorVersion;
use aws_sdk_s3::{self as s3};

pub struct AwsConfig;

impl AwsConfig {
    pub async fn s3() -> s3::Client {
        let endpoint_url = std::env::var("AWS_ENDPOINT_URI").unwrap_or_else(|_| "http://localhost:4566".to_string());
        let behavior_version = BehaviorVersion::latest();

        let sdk_config = aws_config::defaults(behavior_version)
            .endpoint_url(endpoint_url)
            .load()
            .await;

        Self::s3_client(&sdk_config)
    }

    fn s3_client(conf: &aws_config::SdkConfig) -> aws_sdk_s3::Client {
        let s3_config_builder = aws_sdk_s3::config::Builder::from(conf)
            .force_path_style(true);
        aws_sdk_s3::Client::from_conf(s3_config_builder.build())
    }

    pub async fn s3_url(bucket: &str, key: &str) -> String {
        let localstack = std::env::var("USE_LOCALSTACK").is_ok();

        if localstack {
            format!("http://localhost:4566/{}/{}", bucket, key)
        } else {
            format!("https://{}.s3.amazonaws.com/{}", bucket, key)
        }
    }
}
