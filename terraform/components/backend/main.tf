terraform {
  backend "s3" {
    region               = "ap-northeast-1"
    bucket               = "deploy-047980477351-ap-northeast-1-efg.river"
    workspace_key_prefix = "nextjs-neardev-rust-template"
    key                 = "components/backend/terraform.tfstate"
  }
}

locals {
  tags = {
    "env" = terraform.workspace
    "project" = "nextjs-neardev-rust-template"
  }
}
